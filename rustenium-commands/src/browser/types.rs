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
	active: bool,
	#[serde(rename = "clientWindow")]
	client_window: ClientWindow,
	#[serde(rename = "height")]
	height: u32,
	#[serde(rename = "state")]
	state: ClientWindowInfoState,
	#[serde(rename = "width")]
	width: u32,
	#[serde(rename = "x")]
	x: i32,
	#[serde(rename = "y")]
	y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserContextInfo {
	#[serde(rename = "userContext")]
	user_context: UserContext,
}

