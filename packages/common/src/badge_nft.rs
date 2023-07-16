use badges::MintRule;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use sg_metadata::Metadata;

pub type Extension = Option<Empty>;

// message types
pub type ExecuteMsg = sg721::ExecuteMsg<Extension, Empty>;
pub type QueryMsg = sg721_base::msg::QueryMsg;

// response types
pub type ContractInfoResponse = cw721::ContractInfoResponse;
pub type NumTokensResponse = cw721::NumTokensResponse;
pub type OwnerOfResponse = cw721::OwnerOfResponse;
pub type TokensResponse = cw721::TokensResponse;
pub type ApprovalResponse = cw721::ApprovalResponse;
pub type ApprovalsResponse = cw721::ApprovalsResponse;
pub type OperatorsResponse = cw721::OperatorsResponse;
pub type BadgeInfoResponse = cw721::NftInfoResponse<Metadata>;
pub type AllBadgesInfoResponse = cw721::AllNftInfoResponse<Metadata>;
pub type MinterResponse = cw721_base::MinterResponse;
pub type CollectionInfoResponse = sg721_base::msg::CollectionInfoResponse;

#[cw_serde]
pub struct BadgeResponse {
    pub manager: String,
    pub metadata: Metadata,
    pub transferrable: bool,
    pub rule: MintRule,
    pub expiry: Option<u64>,
    pub max_supply: Option<u64>,
    pub current_supply: u64,
}
