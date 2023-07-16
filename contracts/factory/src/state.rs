use common::factory::EventInfo;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const ADMIN: Item<Addr> = Item::new("admin");

pub const BADGE_TO_EVENT: Map<u64, (EventInfo, Option<Addr>)> = Map::new("badge_to_event");

pub const TOTAL_BADGES: Item<u64> = Item::new("total_badges");

pub const BADGE_NFT_CODE_ID: Item<u64> = Item::new("badge_nft_code_id");
