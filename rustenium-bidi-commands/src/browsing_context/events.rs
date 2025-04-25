use serde::{Serialize, Deserialize};
use super::types::{BaseNavigationInfo, BrowsingContext, Info, NavigationInfo, UserPromptType};

use super::types::UserPromptHandlerType;

#[derive(Debug, Serialize, Deserialize)]
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
	pub method: ContextCreatedMethod,
	#[serde(rename = "params")]
	pub params: Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextDestroyed {
	#[serde(rename = "method")]
	pub method: ContextDestroyedMethod,
	#[serde(rename = "params")]
	pub params: Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationStarted {
	#[serde(rename = "method")]
	pub method: NavigationStartedMethod,
	#[serde(rename = "params")]
	pub params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FragmentNavigated {
	#[serde(rename = "method")]
	pub method: FragmentNavigatedMethod,
	#[serde(rename = "params")]
	pub params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryUpdated {
	#[serde(rename = "method")]
	pub method: HistoryUpdatedMethod,
	#[serde(rename = "params")]
	pub params: HistoryUpdatedParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryUpdatedParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "url")]
	pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomContentLoaded {
	#[serde(rename = "method")]
	pub method: DomContentLoadedMethod,
	#[serde(rename = "params")]
	pub params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Load {
	#[serde(rename = "method")]
	pub method: LoadMethod,
	#[serde(rename = "params")]
	pub params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadWillBegin {
	#[serde(rename = "method")]
	pub method: DownloadWillBeginMethod,
	#[serde(rename = "params")]
	pub params: DownloadWillBeginParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadWillBeginParams {
	#[serde(rename = "suggestedFilename")]
	pub suggested_filename: String,
	pub base_navigation_info: BaseNavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationAborted {
	#[serde(rename = "method")]
	pub method: NavigationAbortedMethod,
	#[serde(rename = "params")]
	pub params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationCommitted {
	#[serde(rename = "method")]
	pub method: NavigationCommittedMethod,
	#[serde(rename = "params")]
	pub params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationFailed {
	#[serde(rename = "method")]
	pub method: NavigationFailedMethod,
	#[serde(rename = "params")]
	pub params: NavigationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptClosed {
	#[serde(rename = "method")]
	pub method: UserPromptClosedMethod,
	#[serde(rename = "params")]
	pub params: UserPromptClosedParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptClosedParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "accepted")]
	pub accepted: bool,
	#[serde(rename = "type")]
	pub r#type: UserPromptType,
	#[serde(rename = "userText")]
	pub user_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptOpened {
	#[serde(rename = "method")]
	pub method: UserPromptOpenedMethod,
	#[serde(rename = "params")]
	pub params: UserPromptOpenedParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptOpenedParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "handler")]
	pub handler: UserPromptHandlerType,
	#[serde(rename = "message")]
	pub message: String,
	#[serde(rename = "type")]
	pub r#type: UserPromptType,
	#[serde(rename = "defaultValue")]
	pub default_value: Option<String>,
}

