use serde::{Serialize, Deserialize};
use serde_cbor;
use crate::browser::types::{ClientWindow, UserContext, UserContextInfo};
pub type CreateUserContextResult = UserContextInfo;

pub enum BrowserCommand {
	Close(Close),
	CreateUserContext(CreateUserContext),
	GetClientWindows(GetClientWindows),
	GetUserContexts(GetUserContexts),
	RemoveUserContext(RemoveUserContext),
	SetClientWindowState(SetClientWindowState),
}

pub enum BrowserResult {
	CreateUserContextResult(CreateUserContextResult),
	GetUserContextsResult(GetUserContextsResult),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientWindowState {
	Named(ClientWindowNamedState),
	Rect(ClientWindowRectState),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserContextsResult {
	user_contexts: Vec<UserContextInfo>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Close {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserContext {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClientWindows {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserContexts {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveUserContext {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: RemoveUserContextParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveUserContextParameters {
	#[serde(rename = "userContext")]
	user_context: UserContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetClientWindowState {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: SetClientWindowStateParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SetClientWindowStateParameters {
	#[serde(rename = "clientWindow")]
	client_window: ClientWindow,
	#[serde(flatten)]
	state: ClientWindowState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientWindowNamedState {
	#[serde(rename = "state")]
	state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientWindowRectState {
	#[serde(rename = "state")]
	state: String,
	#[serde(rename = "width")]
	width: Option<u32>,
	#[serde(rename = "height")]
	height: Option<u32>,
	#[serde(rename = "x")]
	x: Option<i32>,
	#[serde(rename = "y")]
	y: Option<i32>,
}

