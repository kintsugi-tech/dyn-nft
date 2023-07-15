use std::marker::PhantomData;
use badges::{Badge, MintRule};
use common::badge_nft::BadgeInfoResponse;
use cosmwasm_std::{testing::{mock_info, mock_env, MockQuerier, MockStorage, MockApi}, Addr, WasmQuery, QuerierResult, to_binary, ContractInfoResponse, OwnedDeps, from_binary};
use sg721::CollectionInfo;
use sg_metadata::{Metadata, Trait};
use crate::{msg::{InstantiateMsg, ExecuteMsg}, entry::{instantiate, execute, query}};

/// sg721 requires that the deployer must be a contract:
/// https://github.com/public-awesome/launchpad/blob/v0.21.1/contracts/sg721-base/src/contract.rs#L39-L47
///
/// to pass the test, we use a custom wasm query handler that returns "badge_hub"
/// as a valid contract, and make sure to use "badge_hub" here as the sender.
fn wasm_querier_handler(query: &WasmQuery) -> QuerierResult {
    match query {
        WasmQuery::ContractInfo {
            contract_addr,
        } if contract_addr == "factory" => {
            Ok(to_binary(&ContractInfoResponse::new(69420, "larry")).into()).into()
        },
        _ => panic!("[mock]: unimplemented wasm query: {query:?}"),
    }
}

fn mock_metadata(input: &str) -> Metadata {
    let image_name = format!("ipfs://hash{}", input);
    Metadata {
        image: Some(image_name),
        ..Default::default()
    }
}

#[test]
fn proper_instantiate() {
    let mut deps = OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockQuerier::default(),
        custom_query_type: PhantomData,
    };
    deps.querier.update_wasm(wasm_querier_handler);

    let info = mock_info("factory", &[]);
    let env = mock_env();

    let collection_info = CollectionInfo {
        creator: "creator".to_string(),
        description: "description_test".to_string(),
        image: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
        external_link: None,
        explicit_content: None,
        start_trading_time: None,
        royalty_info: None
    };

    let badge = Badge {
        manager: Addr::unchecked("niil"),
        metadata: mock_metadata("default"),
        transferrable: false,
        rule: MintRule::ByKeys,
        expiry: None,
        max_supply: None,
        current_supply: 100,
    };

    let msg = InstantiateMsg {
        factory: "factory".to_string(),
        api_url: "hackwasm_berlin_2023".to_string(),
        collection_info: collection_info,
        metadata: vec![mock_metadata("default"), mock_metadata("special1")],
        roles: vec!["default".to_string(), "special1".to_string()],
        badge: badge
    };

    instantiate(deps.as_mut(), env, info, msg).unwrap();

}

#[test]
fn turn_metadata() {
    let mut deps = OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockQuerier::default(),
        custom_query_type: PhantomData,
    };
    deps.querier.update_wasm(wasm_querier_handler);

    let info = mock_info("factory", &[]);
    let env = mock_env();

    let collection_info = CollectionInfo {
        creator: "creator".to_string(),
        description: "description_test".to_string(),
        image: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
        external_link: None,
        explicit_content: None,
        start_trading_time: None,
        royalty_info: None
    };

    let badge = Badge {
        manager: Addr::unchecked("niil"),
        metadata: mock_metadata("default"),
        transferrable: false,
        rule: MintRule::ByKeys,
        expiry: None,
        max_supply: None,
        current_supply: 100,
    };

    let msg = InstantiateMsg {
        factory: "factory".to_string(),
        api_url: "hackwasm_berlin_2023".to_string(),
        collection_info: collection_info,
        metadata: vec![mock_metadata("default"), mock_metadata("special1")],
        roles: vec!["default".to_string(), "special1".to_string()],
        badge: badge
    };

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let mint_msg = ExecuteMsg::Legacy(sg721::ExecuteMsg::Mint(cw721_base::MintMsg {
        token_id: "primo".to_string(),
        owner: "dimi".to_string(),
        token_uri: None,
        extension: None
    }));

    execute(deps.as_mut(), env.clone(), info.clone(), mint_msg).unwrap();

    let res: BadgeInfoResponse = from_binary(&query(deps.as_ref(), env.clone(), sg721_base::msg::QueryMsg::NftInfo { token_id: "primo".to_string() }).unwrap()).unwrap();

    let mut expected = mock_metadata("default");
    expected.attributes = Some(vec![
        Trait {
            display_type: None,
            trait_type: "id".to_string(),
            value: "primo".to_string(),
        },
        Trait {
            display_type: None,
            trait_type: "role".to_string(),
            value: "default".to_string(),
        }
    ]);

    assert_eq!(res.extension, expected);

    let turn_metadata_msg = ExecuteMsg::TurnMetadata {
        token_id: "primo".to_string(),
        role: "special1".to_string()
    };

    execute(deps.as_mut(), env.clone(), info, turn_metadata_msg).unwrap();

    let res: BadgeInfoResponse = from_binary(&query(deps.as_ref(), env, sg721_base::msg::QueryMsg::NftInfo { token_id: "primo".to_string() }).unwrap()).unwrap();

    let mut expected = mock_metadata("special1");
    expected.attributes = Some(vec![
        Trait {
            display_type: None,
            trait_type: "id".to_string(),
            value: "primo".to_string(),
        },
        Trait {
            display_type: None,
            trait_type: "role".to_string(),
            value: "special1".to_string(),
        }
    ]);

    assert_eq!(res.extension, expected);
    
}