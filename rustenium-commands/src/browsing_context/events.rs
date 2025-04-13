use serde::{Serialize, Deserialize};
use super::types::{BaseNavigationInfo, BrowsingContext, Info, NavigationInfo, UserPromptType};

use super::types::UserPromptHandlerType;
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
enum ContextCreatedMethod {
	#[serde(rename = "browsingContext.contextCreated")]
	BrowsingContextContextCreated,
}

#[derive(Debug, Serialize, Deserialize)]
enum ContextDestroyedMethod {
	#[serde(rename = "browsingContext.contextDestroyed")]
	BrowsingContextContextDestroyed,
}

#[derive(Debug, Serialize, Deserialize)]
enum NavigationStartedMethod {
	#[serde(rename = "browsingContext.navigationStarted")]
	BrowsingContextNavigationStarted,
}

#[derive(Debug, Serialize, Deserialize)]
enum FragmentNavigatedMethod {
	#[serde(rename = "browsingContext.fragmentNavigated")]
	BrowsingContextFragmentNavigated,
}

#[derive(Debug, Serialize, Deserialize)]
enum HistoryUpdatedMethod {
	#[serde(rename = "browsingContext.historyUpdated")]
	BrowsingContextHistoryUpdated,
}

#[derive(Debug, Serialize, Deserialize)]
enum DomContentLoadedMethod {
	#[serde(rename = "browsingContext.domContentLoaded")]
	BrowsingContextDomContentLoaded,
}

#[derive(Debug, Serialize, Deserialize)]
enum LoadMethod {
	#[serde(rename = "browsingContext.load")]
	BrowsingContextLoad,
}

#[derive(Debug, Serialize, Deserialize)]
enum DownloadWillBeginMethod {
	#[serde(rename = "browsingContext.downloadWillBegin")]
	BrowsingContextDownloadWillBegin,
}

#[derive(Debug, Serialize, Deserialize)]
enum NavigationAbortedMethod {
	#[serde(rename = "browsingContext.navigationAborted")]
	BrowsingContextNavigationAborted,
}

#[derive(Debug, Serialize, Deserialize)]
enum NavigationCommittedMethod {
	#[serde(rename = "browsingContext.navigationCommitted")]
	BrowsingContextNavigationCommitted,
}

#[derive(Debug, Serialize, Deserialize)]
enum NavigationFailedMethod {
	#[serde(rename = "browsingContext.navigationFailed")]
	BrowsingContextNavigationFailed,
}

#[derive(Debug, Serialize, Deserialize)]
enum UserPromptClosedMethod {
	#[serde(rename = "browsingContext.userPromptClosed")]
	BrowsingContextUserPromptClosed,
}

#[derive(Debug, Serialize, Deserialize)]
enum UserPromptOpenedMethod {
	#[serde(rename = "browsingContext.userPromptOpened")]
	BrowsingContextUserPromptOpened,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextCreated {
	#[serde(rename = "method")]
	method: ContextCreatedMethod,
	#[serde(rename = "params")]
	params: Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextDestroyed {
	#[serde(rename = "method")]
	method: ContextDestroyedMethod,
	#[serde(rename = "params")]
	params: Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationStarted {
	#[serde(rename = "method")]
	method: NavigationStartedMethod,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FragmentNavigated {
	#[serde(rename = "method")]
	method: FragmentNavigatedMethod,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryUpdated {
	#[serde(rename = "method")]
	method: HistoryUpdatedMethod,
	#[serde(rename = "params")]
	params: HistoryUpdatedParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryUpdatedParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "url")]
	url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomContentLoaded {
	#[serde(rename = "method")]
	method: DomContentLoadedMethod,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Load {
	#[serde(rename = "method")]
	method: LoadMethod,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadWillBegin {
	#[serde(rename = "method")]
	method: DownloadWillBeginMethod,
	#[serde(rename = "params")]
	params: DownloadWillBeginParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadWillBeginParams {
	#[serde(rename = "suggestedFilename")]
	suggested_filename: String,
	base_navigation_info: BaseNavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationAborted {
	#[serde(rename = "method")]
	method: NavigationAbortedMethod,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationCommitted {
	#[serde(rename = "method")]
	method: NavigationCommittedMethod,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationFailed {
	#[serde(rename = "method")]
	method: NavigationFailedMethod,
	#[serde(rename = "params")]
	params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptClosed {
	#[serde(rename = "method")]
	method: UserPromptClosedMethod,
	#[serde(rename = "params")]
	params: UserPromptClosedParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptClosedParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "accepted")]
	accepted: bool,
	#[serde(rename = "type")]
	r#type: UserPromptType,
	#[serde(rename = "userText")]
	user_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptOpened {
	#[serde(rename = "method")]
	method: UserPromptOpenedMethod,
	#[serde(rename = "params")]
	params: UserPromptOpenedParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptOpenedParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "handler")]
	handler: UserPromptHandlerType,
	#[serde(rename = "message")]
	message: String,
	#[serde(rename = "type")]
	r#type: UserPromptType,
	#[serde(rename = "defaultValue")]
	default_value: Option<String>,
}

