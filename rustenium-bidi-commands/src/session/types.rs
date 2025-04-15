use serde::{Serialize, Deserialize};

use crate::{browser::types::UserContext, browsing_context::types::{BrowsingContext, UserPromptHandlerType}};

pub type Subscription = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct CapabilitiesRequest {
	#[serde(rename = "alwaysMatch")]
	always_match: Option<CapabilityRequest>,
	#[serde(rename = "firstMatch")]
	first_match: Option<Vec<CapabilityRequest>>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CapabilityRequest {
	#[serde(rename = "acceptInsecureCerts")]
	accept_insecure_certs: Option<bool>,
	#[serde(rename = "browserName")]
	browser_name: Option<String>,
	#[serde(rename = "browserVersion")]
	browser_version: Option<String>,
	#[serde(rename = "platformName")]
	platform_name: Option<String>,
	#[serde(rename = "proxy")]
	proxy: Option<ProxyConfiguration>,
	#[serde(rename = "unhandledPromptBehavior")]
	unhandled_prompt_behavior: Option<UserPromptHandler>,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
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
	proxy_type: AutodetectProxyConfigurationType,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectProxyConfiguration {
	#[serde(rename = "proxyType")]
	proxy_type: DirectProxyConfigurationType,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManualProxyConfiguration {
	#[serde(rename = "proxyType")]
	proxy_type: ManualProxyConfigurationType,
	#[serde(rename = "ftpProxy")]
	ftp_proxy: Option<String>,
	#[serde(rename = "httpProxy")]
	http_proxy: Option<String>,
	#[serde(rename = "sslProxy")]
	ssl_proxy: Option<String>,
	#[serde(rename = "SocksProxyConfiguration")]
	socks_proxy_configuration: Option<SocksProxyConfiguration>,
	#[serde(rename = "noProxy")]
	no_proxy: Option<Vec<String>>,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocksProxyConfiguration {
	#[serde(rename = "socksProxy")]
	socks_proxy: String,
	#[serde(rename = "socksVersion")]
	socks_version: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PacProxyConfiguration {
	#[serde(rename = "proxyType")]
	proxy_type: PacProxyConfigurationType,
	#[serde(rename = "proxyAutoconfigUrl")]
	proxy_autoconfig_url: String,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemProxyConfiguration {
	#[serde(rename = "proxyType")]
	proxy_type: SystemProxyConfigurationType,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptHandler {
	#[serde(rename = "alert")]
	alert: Option<UserPromptHandlerType>,
	#[serde(rename = "beforeUnload")]
	before_unload: Option<UserPromptHandlerType>,
	#[serde(rename = "confirm")]
	confirm: Option<UserPromptHandlerType>,
	#[serde(rename = "default")]
	default: Option<UserPromptHandlerType>,
	#[serde(rename = "file")]
	file: Option<UserPromptHandlerType>,
	#[serde(rename = "prompt")]
	prompt: Option<UserPromptHandlerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionRequest {
	#[serde(rename = "events")]
	events: Vec<String>,
	#[serde(rename = "contexts")]
	contexts: Option<Vec<BrowsingContext>>,
	#[serde(rename = "userContexts")]
	user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsubscribeByIDRequest {
	#[serde(rename = "subscriptions")]
	subscriptions: Vec<Subscription>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UnsubscribeByAttributesRequest {
	#[serde(rename = "events")]
	events: Vec<String>,
	#[serde(rename = "contexts")]
	contexts: Option<Vec<BrowsingContext>>,
}

