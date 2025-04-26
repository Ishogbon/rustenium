use serde::{Serialize, Deserialize};
use serde_cbor;
use crate::browser::types::{ClientWindow, UserContext, UserContextInfo};
pub type CreateUserContextResult = UserContextInfo;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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
#[serde(untagged)]
pub enum ClientWindowState {
	ClientWindowNamedState(ClientWindowNamedState),
	ClientWindowRectState(ClientWindowRectState),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserContextsResult {
	pub user_contexts: Vec<UserContextInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CloseMethod {
	#[serde(rename = "browser.close")]
	BrowserClose,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Close {
	#[serde(rename = "method")]
	pub method: CloseMethod,
	#[serde(rename = "params")]
	pub params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CreateUserContextMethod {
	#[serde(rename = "browser.createUserContext")]
	BrowserCreateUserContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserContext {
	#[serde(rename = "method")]
	pub method: CreateUserContextMethod,
	#[serde(rename = "params")]
	pub params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GetClientWindowsMethod {
	#[serde(rename = "browser.getClientWindows")]
	BrowserGetClientWindows,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClientWindows {
	#[serde(rename = "method")]
	pub method: GetClientWindowsMethod,
	#[serde(rename = "params")]
	pub params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GetUserContextsMethod {
	#[serde(rename = "browser.getUserContexts")]
	BrowserGetUserContexts,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserContexts {
	#[serde(rename = "method")]
	pub method: GetUserContextsMethod,
	#[serde(rename = "params")]
	pub params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RemoveUserContextMethod {
	#[serde(rename = "browser.removeUserContext")]
	BrowserRemoveUserContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SetClientWindowStateMethod {
	#[serde(rename = "browser.setClientWindowState")]
	BrowserSetClientWindowState,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClientWindowNamedStateType {
	Fullscreen,
	Maximized,
	Minimized,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientWindowRectStateType {
	#[serde(rename = "normal")]
	Normal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientWindowNamedState {
	#[serde(rename = "state")]
	pub state: ClientWindowNamedStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientWindowRectState {
	#[serde(rename = "state")]
	pub state: ClientWindowRectStateType,
	#[serde(skip_serializing_if = "Option::is_none", rename = "width")]
	pub width: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "height")]
	pub height: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "x")]
	pub x: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "y")]
	pub y: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveUserContextParameters {
	#[serde(rename = "userContext")]
	pub user_context: UserContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetClientWindowStateParameters {
	#[serde(rename = "clientWindow")]
	pub client_window: ClientWindow,
	#[serde(flatten)]
	pub extesnsion: ClientWindowState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveUserContext {
	#[serde(rename = "method")]
	pub method: RemoveUserContextMethod,
	#[serde(rename = "params")]
	pub params: RemoveUserContextParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetClientWindowState {
	#[serde(rename = "method")]
	pub method: SetClientWindowStateMethod,
	#[serde(rename = "params")]
	pub params: SetClientWindowStateParameters,
}

