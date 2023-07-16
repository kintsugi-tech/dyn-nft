use badges::Badge;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdError, StdResult};
use sg721_base::ExecuteMsg as ExecuteMsgLegacy;
use sg_metadata::Metadata;

#[cw_serde]
pub struct InstantiateMsg {
    pub factory: String,
    pub api_url: String,
    pub collection_info: sg721::CollectionInfo<sg721::RoyaltyInfoResponse>,
    pub metadata: Vec<Metadata>,
    pub roles: Vec<String>,
    pub badge: Badge,
}

impl InstantiateMsg {
    pub fn validate(&self) -> StdResult<()> {
        if self.metadata.len() != self.roles.len() {
            Err(StdError::generic_err(
                "metadata and roles should be of the same length",
            ))
        } else {
            Ok(())
        }
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    TurnMetadata { token_id: String, role: String },
    Legacy(ExecuteMsgLegacy),
}
