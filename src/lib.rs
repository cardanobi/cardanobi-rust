// lib.rs

// Make the modules available within the crate
pub mod utils {
    pub mod api_client;
    pub mod constants;
    pub mod misc;
}

pub mod domains {
    pub mod bi;
}

pub mod cardanobi;


// Re-export the public API
pub use crate::utils::api_client::APIClient;
pub use crate::cardanobi::CardanoBI;
pub use crate::domains::bi::Bi;


