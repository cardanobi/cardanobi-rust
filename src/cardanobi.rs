use crate::utils::api_client::APIClient;
use crate::domains::core::Core; // Ensure `Core` is imported properly
use crate::domains::bi::Bi; // Ensure `Bi` is imported properly
use crate::utils::misc::ApiClientError;

pub struct CardanoBI {
    client: APIClient,
    pub core: Core, // Add the Bi struct as a public field
    pub bi: Bi, // Add the Bi struct as a public field
}

impl CardanoBI {
    pub async fn new(api_key: Option<&str>, api_secret: Option<&str>, network: Option<&str>) -> Result<Self, ApiClientError> {
        let mut client = APIClient::new(api_key, api_secret, network)?;
        client.get_access_token().await?;  // Ensure token is fetched here

        let core = Core::new(client.clone()); // Client is ready with the token, no need for further mutability
        let bi = Bi::new(client.clone()); // Client is ready with the token, no need for further mutability
        Ok(CardanoBI { client, core, bi })
    }
}

