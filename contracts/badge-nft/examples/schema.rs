use std::{env::current_dir, fs::create_dir_all};

use badge_nft::msg::{ExecuteMsg, InstantiateMsg};
use common::badge_nft::{AllBadgesInfoResponse, BadgeInfoResponse, QueryMsg};
use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);

    export_schema(&schema_for!(BadgeInfoResponse), &out_dir);
    export_schema(&schema_for!(AllBadgesInfoResponse), &out_dir);
}
