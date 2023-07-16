use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct EventInfo {
    pub name: String,
    pub date: String,
    pub location: String,
}
