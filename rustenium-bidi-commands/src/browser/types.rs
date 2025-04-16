use serde::{Deserialize, Serialize};

pub type ClientWindow = String;
pub type UserContext = String;

pub type CreateUserContextResult = UserContextInfo;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClientWindowState {
	Fullscreen,
	Maximized,
	Minimized,
	Normal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClientWindowInfoState {
	Fullscreen,
	Maximized,
	Minimized,
	Normal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientWindowInfo {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "clientWindow")]
	pub client_window: ClientWindow,
	#[serde(rename = "height")]
	pub height: u32,
	#[serde(rename = "state")]
	pub state: ClientWindowInfoState,
	#[serde(rename = "width")]
	pub width: u32,
	#[serde(rename = "x")]
	pub x: i32,
	#[serde(rename = "y")]
	pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserContextInfo {
	#[serde(rename = "userContext")]
	pub user_context: UserContext,
}

