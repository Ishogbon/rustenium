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
#[serde(untagged)]
pub enum ClientWindowState {
	ClientWindowNamedState(ClientWindowNamedState),
	ClientWindowRectState(ClientWindowRectState),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserContextsResult {
	user_contexts: Vec<UserContextInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CloseMethod {
	#[serde(rename = "browser.close")]
	BrowserClose,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Close {
	#[serde(rename = "method")]
	method: CloseMethod,
	#[serde(rename = "params")]
	params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
enum CreateUserContextMethod {
	#[serde(rename = "browser.createUserContext")]
	BrowserCreateUserContext,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserContext {
	#[serde(rename = "method")]
	method: CreateUserContextMethod,
	#[serde(rename = "params")]
	params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
enum GetClientWindowsMethod {
	#[serde(rename = "browser.getClientWindows")]
	BrowserGetClientWindows,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetClientWindows {
	#[serde(rename = "method")]
	method: GetClientWindowsMethod,
	#[serde(rename = "params")]
	params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
enum GetUserContextsMethod {
	#[serde(rename = "browser.getUserContexts")]
	BrowserGetUserContexts,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetUserContexts {
	#[serde(rename = "method")]
	method: GetUserContextsMethod,
	#[serde(rename = "params")]
	params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
enum RemoveUserContextMethod {
	#[serde(rename = "browser.removeUserContext")]
	BrowserRemoveUserContext,
}

#[derive(Debug, Serialize, Deserialize)]
enum SetClientWindowStateMethod {
	#[serde(rename = "browser.setClientWindowState")]
	BrowserSetClientWindowState,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ClientWindowNamedStateType {
	Fullscreen,
	Maximized,
	Minimized,
}

#[derive(Debug, Serialize, Deserialize)]
enum ClientWindowRectStateType {
	#[serde(rename = "normal")]
	Normal,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientWindowNamedState {
	#[serde(rename = "state")]
	state: ClientWindowNamedStateType,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientWindowRectState {
	#[serde(rename = "state")]
	state: ClientWindowRectStateType,
	#[serde(rename = "width")]
	width: Option<u32>,
	#[serde(rename = "height")]
	height: Option<u32>,
	#[serde(rename = "x")]
	x: Option<i32>,
	#[serde(rename = "y")]
	y: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveUserContextParameters {
	#[serde(rename = "userContext")]
	user_context: UserContext,
}

#[derive(Debug, Serialize, Deserialize)]
struct SetClientWindowStateParameters {
	#[serde(rename = "clientWindow")]
	client_window: ClientWindow,
	#[serde(flatten)]
	extesnsion: ClientWindowState,
}

#[derive(Debug, Serialize, Deserialize)]
struct RemoveUserContext {
	#[serde(rename = "method")]
	method: RemoveUserContextMethod,
	#[serde(rename = "params")]
	params: RemoveUserContextParameters,
}

#[derive(Debug, Serialize, Deserialize)]
struct SetClientWindowState {
	#[serde(rename = "method")]
	method: SetClientWindowStateMethod,
	#[serde(rename = "params")]
	params: SetClientWindowStateParameters,
}

