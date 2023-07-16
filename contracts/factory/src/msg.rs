use badges::Badge;
use common::factory::EventInfo;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use sg_metadata::Metadata;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub badge_nft_code_id: u64
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateBadgeNft {
        api_url: String,
        event_info: EventInfo,
        collection_info: sg721::CollectionInfo<sg721::RoyaltyInfoResponse>,
        metadata: Vec<Metadata>,
        roles: Vec<String>,
        badge: Badge
    },
}
