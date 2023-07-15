use std::marker::PhantomData;

use badges::{Badge, MintRule};
use cosmwasm_std::{testing::{mock_info, mock_env, MockQuerier, MockStorage, MockApi}, Addr, WasmQuery, QuerierResult, to_binary, ContractInfoResponse, Empty, OwnedDeps};
use sg721::CollectionInfo;
use sg_metadata::Metadata;
use crate::{msg::InstantiateMsg, entry::instantiate};

#[test]
fn proper_instantiate() {
    let mut deps = OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockQuerier::default(),
        custom_query_type: PhantomData,
    };
    deps.querier.update_wasm(wasm_querier_handler);

    let info = mock_info("badge", &[]);
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
        manager: Addr::unchecked("ale"),
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

fn mock_metadata(input: &str) -> Metadata {
    let image_name = format!("ipfs://hash{}", input);
    Metadata {
        image: Some(image_name),
        ..Default::default()
    }
}

/// sg721 requires that the deployer must be a contract:
/// https://github.com/public-awesome/launchpad/blob/v0.21.1/contracts/sg721-base/src/contract.rs#L39-L47
///
/// to pass the test, we use a custom wasm query handler that returns "badge_hub"
/// as a valid contract, and make sure to use "badge_hub" here as the sender.
fn wasm_querier_handler(query: &WasmQuery) -> QuerierResult {
    match query {
        WasmQuery::ContractInfo {
            contract_addr,
        } if contract_addr == "badge" => {
            Ok(to_binary(&ContractInfoResponse::new(69420, "larry")).into()).into()
        },
        _ => panic!("[mock]: unimplemented wasm query: {query:?}"),
    }
}