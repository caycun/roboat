use crate::{Client, RoboatError};
use reqwest::header;
use serde::{Deserialize, Serialize};

const CLIENT_SETTINGS_V2_API: &str = "https://clientsettings.roblox.com/v2";

/// Client version details.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct ClientVersion {
    pub version: String,
    #[serde(rename = "clientVersionUpload")]
    pub upload: String,
}

/// User channel details.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct UserChannel {
    #[serde(rename = "channelName")]
    pub name: String,
    pub token: Option<String>,
}

impl Client {
    /// Gets the client version information for a specific binary type.
    ///
    /// # Notes
    /// * Uses the endpoint `GET /v2/client-version/{binaryType}`.
    /// * Common values for `binary_type` are `WindowsPlayer`, `WindowsStudio`, `WindowsStudio64`, `MacPlayer`, and `MacStudio`.
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    /// let version_info = client.client_version("WindowsPlayer".to_string()).await?;
    /// println!("Current version: {}", version_info.version);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn client_version(
        &self,
        binary_type: String,
    ) -> Result<ClientVersion, RoboatError> {
        let formatted_url = format!("{}/client-version/{}", CLIENT_SETTINGS_V2_API, binary_type);
        let request_result = self.reqwest_client.get(&formatted_url).send().await;
        let response = Self::validate_request_result(request_result).await?;
        Self::parse_to_raw::<ClientVersion>(response).await
    }

    /// Gets the client version information for a specific binary type and channel.
    ///
    /// # Notes
    /// * Uses the endpoint `GET /v2/client-version/{binaryType}/channel/{channelName}`.
    /// * Common values for `binary_type` are `WindowsPlayer`, `WindowsStudio`, `WindowsStudio64`, `MacPlayer`, and `MacStudio`.
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    /// let version_info = client.client_version_for_channel("WindowsStudio".to_string(), "zintegration".to_string()).await?;
    /// println!("Channel version: {}", version_info.version);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn client_version_for_channel(
        &self,
        binary_type: String,
        channel_name: String,
    ) -> Result<ClientVersion, RoboatError> {
        let formatted_url = format!(
            "{}/client-version/{}/channel/{}",
            CLIENT_SETTINGS_V2_API, binary_type, channel_name
        );
        let request_result = self.reqwest_client.get(&formatted_url).send().await;
        let response = Self::validate_request_result(request_result).await?;
        Self::parse_to_raw::<ClientVersion>(response).await
    }

    /// Gets the channel name for the currently logged in user.
    ///
    /// # Notes
    /// * Uses the endpoint `GET /v2/user-channel`.
    /// * Requires a valid roblosecurity.
    /// * Common values for `binary_type` are `WindowsPlayer`, `WindowsStudio`, `WindowsStudio64`, `MacPlayer`, and `MacStudio`.
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    /// let user_channel = client.user_channel(Some("WindowsPlayer".to_string())).await?;
    /// println!("User channel: {}", user_channel.channel_name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn user_channel(
        &self,
        binary_type: Option<String>,
    ) -> Result<UserChannel, RoboatError> {
        let cookie_string = self.cookie_string()?;

        let mut formatted_url = format!("{}/user-channel", CLIENT_SETTINGS_V2_API);
        if let Some(bt) = binary_type {
            formatted_url.push_str(&format!("?binaryType={}", bt));
        }

        let request_result = self
            .reqwest_client
            .get(&formatted_url)
            .header(header::COOKIE, cookie_string)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        Self::parse_to_raw::<UserChannel>(response).await
    }
}