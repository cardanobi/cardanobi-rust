use crate::utils::api_client::APIClient;
use crate::domains::bi::Bi; // Ensure `Bi` is imported properly
use crate::utils::misc::ApiClientError;

pub struct CardanoBI {
    client: APIClient,
    pub bi: Bi, // Add the Bi struct as a public field
}

// impl CardanoBI {
//     pub fn new(api_key: Option<&str>, api_secret: Option<&str>, network: Option<&str>) -> Result<Self, Error> {
//         let client = APIClient::new(api_key, api_secret, network)?; // directly pass Options
//         let bi = Bi::new(client.clone()); // Assuming APIClient is cloneable or alternatively using Arc/Mutex for shared state
//         Ok(CardanoBI {
//             client,
//             bi, // Initialize the Bi field here
//         })
//     }
// }

impl CardanoBI {
    pub async fn new(api_key: Option<&str>, api_secret: Option<&str>, network: Option<&str>) -> Result<Self, ApiClientError> {
        let mut client = APIClient::new(api_key, api_secret, network)?;
        client.get_access_token().await?;  // Ensure token is fetched here

        let bi = Bi::new(client.clone()); // Client is ready with the token, no need for further mutability
        Ok(CardanoBI { client, bi })
    }
}

