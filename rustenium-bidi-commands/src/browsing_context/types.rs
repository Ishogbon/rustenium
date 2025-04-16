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
	pub children: Option<InfoList>,
	#[serde(rename = "clientWindow")]
	pub client_window: ClientWindow,
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "originalOpener")]
	pub original_opener: Option<BrowsingContext>,
	#[serde(rename = "url")]
	pub url: String,
	#[serde(rename = "userContext")]
	pub user_context: UserContext,
	#[serde(rename = "parent")]
	pub parent: Option<BrowsingContext>,
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
enum AccessibilityLocatorType {
	#[serde(rename = "accessibility")]
	Aceessibility,
}

#[derive(Debug, Serialize, Deserialize)]
enum CssLocatorType {
	#[serde(rename = "css")]
	Css,
}

#[derive(Debug, Serialize, Deserialize)]
enum ContextLocatorType {
	#[serde(rename = "context")]
	Context,
}

#[derive(Debug, Serialize, Deserialize)]
enum InnerTextLocatorType {
	#[serde(rename = "innerText")]
	InnerText,
}

#[derive(Debug, Serialize, Deserialize)]
enum XPathLocatorType {
	#[serde(rename = "xpath")]
	XPath,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityLocatorValue {
	pub name: Option<String>,
	pub role: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityLocator {
	#[serde(rename = "type")]
	pub r#type: AccessibilityLocatorType,
	#[serde(rename = "value")]
	pub value: AccessibilityLocatorValue,
	#[serde(rename = "name")]
	pub name: Option<String>,
	#[serde(rename = "role")]
	pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CssLocator {
	#[serde(rename = "type")]
	pub r#type: CssLocatorType,
	#[serde(rename = "value")]
	pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextLocatorValue {
	pub context: BrowsingContext,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextLocator {
	#[serde(rename = "type")]
	pub r#type: ContextLocatorType,
	#[serde(rename = "value")]
	pub value: ContextLocatorValue,
	#[serde(rename = "context")]
	pub context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerTextLocator {
	#[serde(rename = "type")]
	pub r#type: InnerTextLocatorType,
	#[serde(rename = "value")]
	pub value: String,
	#[serde(rename = "ignoreCase")]
	pub ignore_case: Option<bool>,
	#[serde(rename = "matchType")]
	pub match_type: Option<String>,
	#[serde(rename = "maxDepth")]
	pub max_depth: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XPathLocator {
	#[serde(rename = "type")]
	pub r#type: XPathLocatorType,
	#[serde(rename = "value")]
	pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseNavigationInfo {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "navigation")]
	pub navigation: Option<Navigation>,
	#[serde(rename = "timestamp")]
	pub timestamp: u32,
	#[serde(rename = "url")]
	pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationInfo {
	#[serde(flatten)]
	pub base: BaseNavigationInfo,
}
