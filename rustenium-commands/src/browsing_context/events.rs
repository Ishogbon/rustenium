pub enum BrowsingContextEvent {
	ContextCreated(ContextCreated),
	ContextDestroyed(ContextDestroyed),
	DomContentLoaded(DomContentLoaded),
	DownloadWillBegin(DownloadWillBegin),
	FragmentNavigated(FragmentNavigated),
	HistoryUpdated(HistoryUpdated),
	Load(Load),
	NavigationAborted(NavigationAborted),
	NavigationCommitted(NavigationCommitted),
	NavigationFailed(NavigationFailed),
	NavigationStarted(NavigationStarted),
	UserPromptClosed(UserPromptClosed),
	UserPromptOpened(UserPromptOpened),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextCreated {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextDestroyed {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationStarted {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FragmentNavigated {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryUpdated {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: HistoryUpdatedParameters,
}

pub struct HistoryUpdatedParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "url")]
	url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomContentLoaded {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Load {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadWillBegin {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: DownloadWillBeginParams,
}

pub struct DownloadWillBeginParams {
	#[serde(rename = "suggestedFilename")]
	suggested_filename: String,
	BaseNavigationInfo(BaseNavigationInfo),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationAborted {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationCommitted {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationFailed {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptClosed {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: UserPromptClosedParameters,
}

pub struct UserPromptClosedParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "accepted")]
	accepted: bool,
	#[serde(rename = "type")]
	type_: UserPromptType,
	#[serde(rename = "userText")]
	user_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptOpened {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: UserPromptOpenedParameters,
}

pub struct UserPromptOpenedParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "handler")]
	handler: UserPromptHandlerType,
	#[serde(rename = "message")]
	message: String,
	#[serde(rename = "type")]
	type_: UserPromptType,
	#[serde(rename = "defaultValue")]
	default_value: Option<String>,
}

