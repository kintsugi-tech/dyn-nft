use badges::Badge;
use cw_storage_plus::{Item, Map};
use sg_metadata::Metadata;

pub const API_URL: Item<String> = Item::new("api_url");

pub const METADATA: Map<String, Metadata> = Map::new("metadata");

pub const ID_TO_SPECIAL_ROLE: Map<String, String> = Map::new("id_to_special_role");

pub const BADGE: Item<Badge> = Item::new("badge");