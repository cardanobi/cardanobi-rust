use phf::{phf_map};

pub static NETWORKS: &[&str] = &["mainnet", "preprod", "preview"];

pub static API_BASE_URLS: phf::Map<&'static str, &'static str> = phf_map! {
    "mainnet" => "https://cardanobi.io:4000",
    "preprod" => "https://preprod.cardanobi.io:4000",
    "preview" => "https://preview.cardanobi.io:4000",
};

pub static IDS_BASE_URLS: phf::Map<&'static str, &'static str> = phf_map! {
    "mainnet" => "https://cardanobi.io:44010",
    "preprod" => "https://cardanobi.io:44010",
    "preview" => "https://cardanobi.io:44010",
};

pub const API_VERSION: u8 = 1;
pub const BATCH_SIZE: usize = 10;
pub const SORT_DIRECTION: &str = "asc";
pub const PAGINATION_PAGE_COUNT: usize = 1;
pub const PAGINATION_PAGE_ITEMS_COUNT: usize = 100;