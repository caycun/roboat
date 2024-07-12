use reqwest::header::{self, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::{Client, RoboatError, User};

mod request_types;

const FRIENDS_LIST: &str = "https://friends.roblox.com/v1/users/{user_id}/friends";
const FRIEND_REQUESTS: &str = "https://friends.roblox.com/v1/my/friends/requests";
const PENDING_FRIEND_REQUESTS: &str = "https://friends.roblox.com/v1/user/friend-requests/count";

/// Model, representing user information that also contains select presence information
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct FriendsUserInformation {
    #[serde(alias = "id")]
    pub user_id: u64,

    #[serde(alias = "externalAppDisplayName")]
    pub external_app_display_name: Option<String>,

    #[serde(alias = "name")]
    pub username: String,

    #[serde(alias = "displayName")]
    pub display_name: String,

    /// Whether the user is online.
    #[serde(alias = "isOnline")]
    pub is_online: bool,

    // TODO: make enum from it
    /// Where the user is online. ['Offline' = 0, 'Online' = 1, 'InGame' = 2, 'InStudio' = 3, 'Invisible' = 4]
    ///
    /// Notes:
    ///  * `None`, when user isn't online
    #[serde(alias = "presenceType")]
    pub presence_type: Option<i32>,

    /// Whether the user is deleted.
    #[serde(alias = "isDeleted")]
    pub is_deleted: bool,

    #[serde(alias = "isBanned")]
    pub is_banned: bool,

    /// Frequents value for the user.
    #[serde(alias = "friendFrequentScore")]
    pub friend_frequent_score: i64,

    /// Frequents rank for the user.
    #[serde(alias = "friendFrequentRank")]
    pub friend_frequent_rank: i64,

    /// The user's verified badge status.
    #[serde(alias = "hasVerifiedBadge")]
    pub has_verified_badge: bool,

    pub description: Option<String>,
    pub created: String,
}

/// Model, representing a friend request.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct FriendRequestUserInformation {
    #[serde(alias = "friendRequest")]
    pub friend_request: FriendRequest,

    #[serde(alias = "mutualFriendsList")]
    pub mutual_friends_list: Vec<String>,

    #[serde(alias = "hasVerifiedBadge")]
    pub has_verified_badge: bool,

    pub description: Option<String>,

    pub created: String,

    #[serde(alias = "isBanned")]
    pub is_banned: bool,

    #[serde(alias = "externalAppDisplayName")]
    pub external_app_display_name: Option<String>,

    #[serde(alias = "id")]
    pub user_id: u64,

    #[serde(alias = "name")]
    pub username: String,

    #[serde(alias = "displayName")]
    pub display_name: String
}

/// Model, representing a friend request.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct FriendRequest {
    /// When the friend request was sent.
    #[serde(alias = "senderId")]
    sender_id: u64,

    /// The sender user Id.
    #[serde(alias = "sourceUniverseId")]
    source_universe_id: u64,

    /// The source universe Id which the request was sent in.
    #[serde(alias = "sentAt")]
    sent_at: String,

    /// The origin source type associated with the friend request.
    /// ['Unknown' = 0, 'PlayerSearch' = 1, 'QrCode' = 2, 'InGame' = 3, 'UserProfile' = 4, 'QqContactImporter' = 5, 'WeChatContactImporter' = 6, 'ProfileShare' = 7, 'PhoneContactImporter' = 8, 'FriendRecommendations' = 9]
    #[serde(alias = "originSourceType")]
    origin_source_type: String,

    /// The contact name associated with the friend request.
    #[serde(alias = "contactName")]
    contact_name: Option<String>,
}

impl Client {
    /// Get list of all friends for the specified user using <https://friends.roblox.com/v1/users/{userId}/friends>.
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
    /// const ROBLOSECURITY: &str = "roblosecurity";
    /// const KEYWORD: &str = "linkmon";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let keyword = KEYWORD.to_string();
    /// let users = client.friends_list(keyword).await?;
    ///
    /// println!("Found {} friends.", users.len());
    ///
    /// for user in users {
    ///     println!("{}: {}", user.username, user.user_id);
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn friends_list(&self, user_id: u64) -> Result<Vec<FriendsUserInformation>, RoboatError> {
        let formatted_url = FRIENDS_LIST.replace("{user_id}", &user_id.to_string());

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;

        let raw = Self::parse_to_raw::<request_types::FriendsListResponse>(response).await?;
        Ok(raw.data)
    }

    // TODO: add cursor argument or get all requests at one
    /// Get list of friend requests using <https://friends.roblox.com/v1/my/friends/requests>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    pub async fn friend_requests(
        &self,
        cursor: Option<String>,
    ) -> Result<(Vec<FriendRequestUserInformation>, Option<String>), RoboatError> {
        let cookie = self.cookie_string()?;
        let formatted_url = format!("{}?limit={}", FRIEND_REQUESTS, 10);

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;

        let raw = Self::parse_to_raw::<request_types::FriendRequestsResponse>(response).await?;
        Ok((raw.data, raw.next_page_cursor))
    }

    /// Get count of pending friend requests using <https://friends.roblox.com/v1/user/friend-requests/count>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    pub async fn pending_friend_requests(
        &self,
    ) -> Result<u64, RoboatError> {
        let cookie = self.cookie_string()?;
        let formatted_url = PENDING_FRIEND_REQUESTS;

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;

        let raw = Self::parse_to_raw::<request_types::PendingFriendRequestsResponse>(response).await?;

        Ok(raw.count)
    }
}
