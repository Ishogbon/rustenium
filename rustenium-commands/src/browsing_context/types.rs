pub enum Info {
	#[serde(rename = "children")]
	children: InfoList | None,
	#[serde(rename = "clientWindow")]
	client_window: ClientWindow,
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "originalOpener")]
	original_opener: BrowsingContext | None,
	#[serde(rename = "url")]
	url: String,
	#[serde(rename = "userContext")]
	user_context: UserContext,
	#[serde(rename = "parent")]
	parent: Option<BrowsingContext | None>,
}

pub enum Locator {
	AccessibilityLocator(AccessibilityLocator),
	CssLocator(CssLocator),
	ContextLocator(ContextLocator),
	InnerTextLocator(InnerTextLocator),
	XPathLocator(XPathLocator),
}

pub struct AccessibilityLocator {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: ,
	#[serde(rename = "name")]
	name: Option<String>,
	#[serde(rename = "role")]
	role: Option<String>,
}

pub struct CssLocator {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: String,
}

pub struct ContextLocator {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: ,
	#[serde(rename = "context")]
	context: BrowsingContext,
}

pub struct InnerTextLocator {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: String,
	#[serde(rename = "ignoreCase")]
	ignore_case: Option<bool>,
	#[serde(rename = "matchType")]
	match_type: Option<String | String>,
	#[serde(rename = "maxDepth")]
	max_depth: Option<u32>,
}

pub struct XPathLocator {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseNavigationInfo {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "navigation")]
	navigation: Navigation | None,
	#[serde(rename = "timestamp")]
	timestamp: u32,
	#[serde(rename = "url")]
	url: String,
}

