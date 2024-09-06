use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UserSearchResponse {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: String,
    pub data: Vec<UserSearchUserInformationRaw>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UserSearchUserInformationRaw {
    pub id: u64,
    pub name: String,
    pub has_verified_badge: bool,
    pub previous_usernames: Vec<String>,
    pub display_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UsernameUserDetailsRequest {
    pub usernames: Vec<String>,
    pub exclude_banned_users: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UsernameUserInformationRaw {
    pub requested_username: String,
    pub has_verified_badge: bool,
    pub id: u64,
    pub name: String,
    pub display_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UsernameUserDetailsResponse {
    pub data: Vec<UsernameUserInformationRaw>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UserPresenceRequest {
    pub user_ids: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserPresenceRaw {
    pub user_presence_type: i32,
    pub last_location: Option<String>,
    pub place_id: Option<u64>,
    pub root_place_id: Option<u64>,
    pub game_id: Option<String>,
    pub universe_id: Option<u64>,
    pub user_id: Option<u64>,
    pub last_online: String,
    pub invisible_mod_expiry: Option<String>,
} 


