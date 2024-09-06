use crate::{Client, RoboatError, User, XCSRF_HEADER};
use reqwest::header::{self, HeaderValue};
use serde::{Deserialize, Serialize};

mod request_types;

const AUTHENTICATED_USER_DETAILS_API: &str = "https://users.roblox.com/v1/users/authenticated";
const USERS_SEARCH_API: &str = "https://users.roblox.com/v1/users/search";
const USER_DETAILS_API: &str = "https://users.roblox.com/v1/users/{user_id}";
const USER_FROM_USERNAME_API: &str = "https://users.roblox.com/v1/usernames/users";

// TODO: try to make a unified user details struct

/// Basic information about the account of the Roblosecurity. Retrieved
/// from <https://users.roblox.com/v1/users/authenticated>.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub(crate) struct ClientUserInformation {
    #[serde(alias = "id")]
    pub user_id: u64,
    #[serde(alias = "name")]
    pub username: String,
    #[serde(alias = "displayName")]
    pub display_name: String,
}

/// The details of a user. Fetched from <https://users.roblox.com/v1/users/{user_id}>.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct UserDetails {
    #[serde(alias = "name")]
    pub username: String,
    #[serde(alias = "displayName")]
    pub display_name: String,
    pub id: u64,
    pub description: String,
    /// A time string of when the account was created. Follows ISO 8061 format.
    #[serde(alias = "created")]
    pub created_at: String,
    /// Whether the account is terminated. Does not include non-termination bans.
    #[serde(alias = "isBanned")]
    pub is_terminated: bool,
    #[serde(alias = "hasVerifiedBadge")]
    pub has_verified_badge: bool,
}

/// The details of a user. Fetched from <https://users.roblox.com/v1/usernames/users>.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct UsernameUserDetails {
    pub requested_username: String,
    #[serde(alias = "name")]
    pub username: String,
    #[serde(alias = "displayName")]
    pub display_name: String,
    pub id: u64,
    #[serde(alias = "hasVerifiedBadge")]
    pub has_verified_badge: bool,
}


// The presence of a user. Fetched from <https://presence.roblox.com/v1/presence/users>.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserPresenceResponse {
    pub user_presences: Vec<request_types::UserPresenceRaw>,
}

impl Client {
    /// Grabs information about the user from <https://catalog.roblox.com/v1/catalog/items/details> using the
    /// Roblosecurity inside the client.
    ///
    /// This is only for internal use. Use [`Client::user_id`], [`Client::username`], and [`Client::display_name`] instead.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    pub(crate) async fn user_information_internal(
        &self,
    ) -> Result<ClientUserInformation, RoboatError> {
        let cookie = self.cookie_string()?;

        let request_result = self
            .reqwest_client
            .get(AUTHENTICATED_USER_DETAILS_API)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let user_information = Self::parse_to_raw::<ClientUserInformation>(response).await?;

        // Cache results.
        self.set_user_information(user_information.clone()).await;

        Ok(user_information)
    }

    /// Searches for a user using <https://users.roblox.com/v1/users/search>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * HOWEVER, if a valid roblosecurity is not provided then there will be a very low rate limit.
    /// * The cursors in this response are not used as using them is currently broken.
    /// * Limits are not used for the same reason (the endpoint does not respect them).
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    /// const KEYWORD: &str = "linkmon";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let keyword = KEYWORD.to_string();
    /// let users = client.user_search(keyword).await?;
    ///
    /// println!("Found {} users.", users.len());
    ///
    /// for user in users {
    ///     println!("{}: {}", user.username, user.user_id);
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn user_search(&self, keyword: String) -> Result<Vec<User>, RoboatError> {
        let formatted_url = format!("{}?keyword={}", USERS_SEARCH_API, keyword);

        let cookie_string = self.cookie_string().unwrap_or(HeaderValue::from_static(""));

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie_string)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<request_types::UserSearchResponse>(response).await?;

        let mut users = Vec::new();

        for user in raw.data {
            let user_data = User {
                user_id: user.id,
                username: user.name,
                display_name: user.display_name,
            };

            users.push(user_data);
        }

        Ok(users)
    }


    // Fetches user presence using <https://presence.roblox.com/v1/presence/users>.
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * HOWEVER, if a valid roblosecurity is not provided then there will be a very low rate limit.
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const USER_IDS = vec![2207291, 123];
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let user_presences = client.user_presence(USER_IDS).await?;
    ///
    /// for user_presence in user_presences.user_presences {
    ///     println!("User ID: {}", user_presence.user_id.unwrap());
    ///     println!("Last Online: {}", user_presence.last_online);
    ///     println!("Last Location: {}", user_presence.last_location.unwrap_or("None".to_string()));
    ///     println!("Place ID: {}", user_presence.place_id.unwrap_or(0));
    ///     println!("Root Place ID: {}", user_presence.root_place_id.unwrap_or(0));
    ///     println!("Game ID: {}", user_presence.game_id.unwrap_or("None".to_string()));
    ///     println!("Universe ID: {}", user_presence.universe_id.unwrap_or(0));
    ///     println!("Invisible Mod Expiry: {}", user_presence.invisible_mod_expiry.unwrap_or("None".to_string()));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn user_presence(&self, user_ids: Vec<u64>) -> Result<UserPresenceResponse, RoboatError> {
        let cookie_string = self.cookie_string().unwrap_or(HeaderValue::from_static(""));

        let json = serde_json::json!({ "userIds": user_ids });
        let request_result = self
            .reqwest_client
            .post("https://presence.roblox.com/v1/presence/users")
            .header(header::COOKIE, cookie_string)
            .json(&json)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<UserPresenceResponse>(response).await?;
        Ok(raw)
    }

    /// Fetches user details using <https://users.roblox.com/v1/users/{user_id}>.
    ///
    /// For bulk fetching, it is recommended to use [`Client::username_user_details`]
    /// instead. This is because more users can be fetched at once using that endpoint.
    /// The only downside of this is that you can only search using usernames instead
    /// of user IDs.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const USER_ID: u64 = 2207291;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let user_details = client.user_details(USER_ID).await?;
    ///
    /// println!("Username: {}", user_details.username);
    /// println!("Display Name: {}", user_details.display_name);
    /// println!("Year Created: {}", user_details.created_at.chars().take(4).collect::<String>());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn user_details(&self, user_id: u64) -> Result<UserDetails, RoboatError> {
        let formatted_url = USER_DETAILS_API.replace("{user_id}", &user_id.to_string());

        let request_result = self.reqwest_client.get(formatted_url).send().await;

        let response = Self::validate_request_result(request_result).await?;
        let user_details = Self::parse_to_raw::<UserDetails>(response).await?;

        Ok(user_details)
    }

    /// Fetches user details using <https://users.roblox.com/v1/usernames/users>.
    ///
    /// This endpoint uses a post request instead of a get request so
    /// it can be used to retrieve information about multiple users at once.
    ///
    /// To fetch a single user, or to fetch a user using a user ID instead of a username,
    /// use [`Client::user_details`] instead.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * This is virtually the same as [`Client::user_details`] except that it can
    /// fetch multiple users at once, and it searches using usernames instead of user IDs.
    /// * The usernames are not case sensitive.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const USERNAME: &str = "Builderman";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let users = vec![USERNAME.to_owned()];
    /// let all_username_user_details = client.username_user_details(users, true).await?;
    /// let username_user_details = all_username_user_details.first().ok_or("User not found")?;
    ///
    /// println!("Username: {}", username_user_details.username);
    /// println!("Display Name: {}", username_user_details.display_name);
    /// println!("ID: {}", username_user_details.id);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn username_user_details(
        &self,
        usernames: Vec<String>,
        exclude_banned_users: bool,
    ) -> Result<Vec<UsernameUserDetails>, RoboatError> {
        let request_result = self
            .reqwest_client
            .post(USER_FROM_USERNAME_API)
            .json(&request_types::UsernameUserDetailsRequest {
                usernames,
                exclude_banned_users,
            })
        .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let raw =
            Self::parse_to_raw::<request_types::UsernameUserDetailsResponse>(response).await?;

        let users = raw
            .data
            .into_iter()
            .map(|user| UsernameUserDetails {
                requested_username: user.requested_username,
                username: user.name,
                display_name: user.display_name,
                id: user.id,
                has_verified_badge: user.has_verified_badge,
            })
        .collect();
        Ok(users)
    }
}

