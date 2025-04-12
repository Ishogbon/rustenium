use serde::{Deserialize, Serialize};
use crate::browser::types::{ClientWindow, UserContext};

pub type BrowsingContext = String;
pub type InfoList = Vec<Info>;
pub type Navigation = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserPromptType {
	Alert,
	Beforeunload,
	Confirm,
	Prompt,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserPromptHandlerType {
	Accept,
	Dismiss,
	Ignore,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
	#[serde(rename = "children")]
	children: Option<InfoList>,
	#[serde(rename = "clientWindow")]
	client_window: ClientWindow,
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "originalOpener")]
	original_opener: Option<BrowsingContext>,
	#[serde(rename = "url")]
	url: String,
	#[serde(rename = "userContext")]
	user_context: UserContext,
	#[serde(rename = "parent")]
	parent: Option<BrowsingContext>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Locator {
	AccessibilityLocator(AccessibilityLocator),
	CssLocator(CssLocator),
	ContextLocator(ContextLocator),
	InnerTextLocator(InnerTextLocator),
	XPathLocator(XPathLocator),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityLocatorValue {
	name: Option<String>,
	role: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityLocator {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "value")]
	value: AccessibilityLocatorValue,
	#[serde(rename = "name")]
	name: Option<String>,
	#[serde(rename = "role")]
	role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CssLocator {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextLocatorValue {
	context: BrowsingContext,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextLocator {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "value")]
	value: ContextLocatorValue,
	#[serde(rename = "context")]
	context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerTextLocator {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "value")]
	value: String,
	#[serde(rename = "ignoreCase")]
	ignore_case: Option<bool>,
	#[serde(rename = "matchType")]
	match_type: Option<String>,
	#[serde(rename = "maxDepth")]
	max_depth: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XPathLocator {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseNavigationInfo {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "navigation")]
	navigation: Option<Navigation>,
	#[serde(rename = "timestamp")]
	timestamp: u32,
	#[serde(rename = "url")]
	url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationInfo {
	#[serde(flatten)]
	base: BaseNavigationInfo,
}
