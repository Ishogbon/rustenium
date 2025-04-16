use serde::{Serialize, Deserialize};

use crate::{browser::types::UserContext, browsing_context::types::{BrowsingContext, UserPromptHandlerType}};

pub type Subscription = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct CapabilitiesRequest {
	#[serde(rename = "alwaysMatch")]
	pub always_match: Option<CapabilityRequest>,
	#[serde(rename = "firstMatch")]
	pub first_match: Option<Vec<CapabilityRequest>>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CapabilityRequest {
	#[serde(rename = "acceptInsecureCerts")]
	pub accept_insecure_certs: Option<bool>,
	#[serde(rename = "browserName")]
	pub browser_name: Option<String>,
	#[serde(rename = "browserVersion")]
	pub browser_version: Option<String>,
	#[serde(rename = "platformName")]
	pub platform_name: Option<String>,
	#[serde(rename = "proxy")]
	pub proxy: Option<ProxyConfiguration>,
	#[serde(rename = "unhandledPromptBehavior")]
	pub unhandled_prompt_behavior: Option<UserPromptHandler>,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProxyConfiguration {
	AutodetectProxyConfiguration(AutodetectProxyConfiguration),
	DirectProxyConfiguration(DirectProxyConfiguration),
	ManualProxyConfiguration(ManualProxyConfiguration),
	PacProxyConfiguration(PacProxyConfiguration),
	SystemProxyConfiguration(SystemProxyConfiguration),
}


#[derive(Debug, Serialize, Deserialize)]
pub enum AutodetectProxyConfigurationType {
	#[serde(rename = "autodetect")]
	Autodetect,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DirectProxyConfigurationType {
	#[serde(rename = "direct")]
	Direct,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ManualProxyConfigurationType {
	#[serde(rename = "manual")]
	Manual,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PacProxyConfigurationType {
	#[serde(rename = "pac")]
	Pac,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SystemProxyConfigurationType {
	#[serde(rename = "system")]
	System,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AutodetectProxyConfiguration {
	#[serde(rename = "proxyType")]
	pub proxy_type: AutodetectProxyConfigurationType,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectProxyConfiguration {
	#[serde(rename = "proxyType")]
	pub proxy_type: DirectProxyConfigurationType,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManualProxyConfiguration {
	#[serde(rename = "proxyType")]
	pub proxy_type: ManualProxyConfigurationType,
	#[serde(rename = "ftpProxy")]
	pub ftp_proxy: Option<String>,
	#[serde(rename = "httpProxy")]
	pub http_proxy: Option<String>,
	#[serde(rename = "sslProxy")]
	pub ssl_proxy: Option<String>,
	#[serde(rename = "SocksProxyConfiguration")]
	pub socks_proxy_configuration: Option<SocksProxyConfiguration>,
	#[serde(rename = "noProxy")]
	pub no_proxy: Option<Vec<String>>,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocksProxyConfiguration {
	#[serde(rename = "socksProxy")]
	pub socks_proxy: String,
	#[serde(rename = "socksVersion")]
	pub socks_version: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PacProxyConfiguration {
	#[serde(rename = "proxyType")]
	pub proxy_type: PacProxyConfigurationType,
	#[serde(rename = "proxyAutoconfigUrl")]
	pub proxy_autoconfig_url: String,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemProxyConfiguration {
	#[serde(rename = "proxyType")]
	pub proxy_type: SystemProxyConfigurationType,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptHandler {
	#[serde(rename = "alert")]
	pub alert: Option<UserPromptHandlerType>,
	#[serde(rename = "beforeUnload")]
	pub before_unload: Option<UserPromptHandlerType>,
	#[serde(rename = "confirm")]
	pub confirm: Option<UserPromptHandlerType>,
	#[serde(rename = "default")]
	pub default: Option<UserPromptHandlerType>,
	#[serde(rename = "file")]
	pub file: Option<UserPromptHandlerType>,
	#[serde(rename = "prompt")]
	pub prompt: Option<UserPromptHandlerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionRequest {
	#[serde(rename = "events")]
	pub events: Vec<String>,
	#[serde(rename = "contexts")]
	pub contexts: Option<Vec<BrowsingContext>>,
	#[serde(rename = "userContexts")]
	pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsubscribeByIDRequest {
	#[serde(rename = "subscriptions")]
	pub subscriptions: Vec<Subscription>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UnsubscribeByAttributesRequest {
	#[serde(rename = "events")]
	pub events: Vec<String>,
	#[serde(rename = "contexts")]
	pub contexts: Option<Vec<BrowsingContext>>,
}

