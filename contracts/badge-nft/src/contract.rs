use std::iter::zip;

use badges::nft::Extension;
use common::badge_nft::{BadgeInfoResponse, AllBadgesInfoResponse, BadgeResponse};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Deps, StdResult, Storage, StdError};
use cw721::Cw721Query;
use sg721_base::ContractError;
use sg_metadata::{Trait, Metadata};
use sg_std::Response;
use crate::{state::{API_URL, BADGE, ID_TO_SPECIAL_ROLE, METADATA}, msg::InstantiateMsg};

#[derive(Default)]
pub struct BadgeContract<'a> {
    pub parent: sg721_base::Sg721Contract<'a, Extension>,
}

pub const CONTRACT_NAME: &str = "crates.io:badge-plus-contract";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> BadgeContract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, sg721_base::ContractError> {
        msg.validate()?;
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        API_URL.save(deps.storage, &msg.api_url)?;

        let _ = zip(msg.roles, msg.metadata).map(|(role, meta)| -> Result<(), ContractError>{
            METADATA.save(deps.storage, role, &meta)?;
            Ok(())
        });

        BADGE.save(deps.storage, &msg.badge)?;

        self.parent.instantiate(
            deps,
            env,
            info,
            sg721::InstantiateMsg {
                name: "Badges-plus".to_string(),
                symbol: "BP".to_string(),
                minter: msg.factory,
                collection_info: msg.collection_info,
            },
        )
    }

    pub fn nft_info(&self, deps: Deps, token_id: String) -> StdResult<BadgeInfoResponse> {
        let uri = uri(deps.storage, &token_id)?;
        let mut badge = BADGE.load(deps.storage)?;
        let mut role = "default".to_string();
        if let Some(special_role) = ID_TO_SPECIAL_ROLE.may_load(deps.storage, token_id.clone())? {
            badge.metadata = METADATA.load(deps.storage, special_role.clone())?;
            role = special_role;
        }
        Ok(BadgeInfoResponse {
            token_uri: Some(uri),
            extension: prepend_traits(badge.metadata, token_id, role),
        })
    }

    pub fn all_nft_info(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: Option<bool>,
    ) -> StdResult<AllBadgesInfoResponse> {
        let access = self.parent.parent.owner_of(
            deps,
            env,
            token_id.to_string(),
            include_expired.unwrap_or(false),
        )?;
        let info = self.nft_info(deps, token_id)?;
        Ok(AllBadgesInfoResponse {
            access,
            info,
        })
    }

    fn query_badge(&self, deps: Deps, token_id: String) -> StdResult<BadgeResponse> {
        let mut badge = BADGE.load(deps.storage)?;
        if let Some(special_role) = ID_TO_SPECIAL_ROLE.may_load(deps.storage, token_id.clone())? {
            badge.metadata = METADATA.load(deps.storage, special_role)?;
        };
        let response = BadgeResponse {
            manager: badge.manager.into_string(),
            metadata: badge.metadata,
            transferrable: badge.transferrable,
            rule: badge.rule,
            expiry: badge.expiry,
            max_supply: badge.max_supply,
            current_supply: badge.current_supply
        };
        Ok(response)
    }

    pub fn assert_transferrable(&self, deps: Deps, token_id: String) -> StdResult<()> {
        let badge = self.query_badge(deps, token_id.clone())?;
        if badge.transferrable {
            Ok(())
        } else {
            Err(StdError::generic_err(format!("badge {} is not transferrable", token_id)))
        }
    }
}

pub fn uri(store: &dyn Storage, id: &str) -> StdResult<String> {
    let api_url = API_URL.load(store)?;
    Ok(format!("{}?id={}", api_url, id))
}

pub fn prepend_traits(mut metadata: Metadata, id: impl ToString, role: impl ToString) -> Metadata {
    let mut traits = vec![
        Trait {
            display_type: None,
            trait_type: "id".to_string(),
            value: id.to_string(),
        },
        Trait {
            display_type: None,
            trait_type: "role".to_string(),
            value: role.to_string(),
        }
    ];

    traits.extend(metadata.attributes.unwrap_or_default().into_iter());

    metadata.attributes = Some(traits);
    metadata
}