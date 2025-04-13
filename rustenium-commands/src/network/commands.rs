use serde::{Serialize, Deserialize};

use crate::browsing_context::types::BrowsingContext;

use super::types::{BytesValue, CookieHeader, Header, Intercept, Request, SetCookieHeader, UrlPattern};

pub enum NetworkCommand {
	AddIntercept(AddIntercept),
	ContinueRequest(ContinueRequest),
	ContinueResponse(ContinueResponse),
	ContinueWithAuth(ContinueWithAuth),
	FailRequest(FailRequest),
	ProvideResponse(ProvideResponse),
	RemoveIntercept(RemoveIntercept),
	SetCacheBehavior(SetCacheBehavior),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddIntercept {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: AddInterceptParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddInterceptParameters {
	#[serde(rename = "phases")]
	phases: Vec<InterceptPhase>,
	#[serde(rename = "contexts")]
	contexts: Option<Vec<BrowsingContext>>,
	#[serde(rename = "urlPatterns")]
	url_patterns: Option<Vec<UrlPattern>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueRequest {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: ContinueRequestParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueRequestParameters {
	#[serde(rename = "request")]
	request: Request,
	#[serde(rename = "body")]
	body: BytesValue,
	#[serde(rename = "cookies")]
	cookies: Vec<CookieHeader>,
	#[serde(rename = "headers")]
	headers: Vec<Header>,
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "url")]
	url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueResponse {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: ContinueResponseParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueResponseParameters {
	#[serde(rename = "request")]
	request: Request,
	#[serde(rename = "cookies")]
	cookies: Vec<SetCookieHeader>,
	#[serde(rename = "credentials")]
	credentials: AuthCredentials,
	#[serde(rename = "headers")]
	headers: Vec<Header>,
	#[serde(rename = "reasonPhrase")]
	reason_phrase: String,
	#[serde(rename = "statusCode")]
	status_code: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContinueWithAuthMethod {
	#[serde(rename = "network.continueWithAuth")]
	NetworkContinueWithAuth,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContinueWithAuthCredentialsAction {
	ProvideCredentials
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContinueWithAuthNoCredentialsAction {
	Default,
	Cancel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueWithAuthCredentials {
	#[serde(rename = "action")]
	pub action: ContinueWithAuthCredentialsAction,

	#[serde(rename = "credentials")]
	pub credentials: AuthCredentials,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueWithAuthNoCredentials {
	#[serde(rename = "action")]
	pub action: ContinueWithAuthNoCredentialsAction,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContinueWithAuthParametersExtension {
	ContinueWithAuthCredentials(ContinueWithAuthCredentials),
	ContinueWithAuthNoCredential(ContinueWithAuthNoCredentials),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueWithAuthParameters {
	#[serde(rename = "request")]
	pub request: Request,

	#[serde(flatten)]
	pub extension: ContinueWithAuthParametersExtension,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueWithAuth {
	#[serde(rename = "method")]
	pub method: ContinueWithAuthMethod,

	#[serde(rename = "params")]
	pub params: ContinueWithAuthParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailRequest {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: FailRequestParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailRequestParameters {
	#[serde(rename = "request")]
	request: Request,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvideResponse {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: ProvideResponseParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvideResponseParameters {
	#[serde(rename = "request")]
	request: Request,
	#[serde(rename = "body")]
	body: BytesValue,
	#[serde(rename = "cookies")]
	cookies: Vec<SetCookieHeader>,
	#[serde(rename = "headers")]
	headers: Vec<Header>,
	#[serde(rename = "reasonPhrase")]
	reason_phrase: String,
	#[serde(rename = "statusCode")]
	status_code: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveIntercept {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: RemoveInterceptParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveInterceptParameters {
	#[serde(rename = "intercept")]
	intercept: Intercept,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SetCacheBehaviorMethod {
	#[serde(rename = "network.setCacheBehavior")]
	NetworkSetCacheBehavior,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CacheBehavior {
	Default,
	Bypass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCacheBehaviorParameters {
	#[serde(rename = "cacheBehavior")]
	pub cache_behavior: CacheBehavior,

	#[serde(rename = "contexts")]
	pub contexts: Option<Vec<BrowsingContext>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCacheBehavior {
	#[serde(rename = "method")]
	pub method: SetCacheBehaviorMethod,

	#[serde(rename = "params")]
	pub params: SetCacheBehaviorParameters,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NetworkInterceptPhase {
	BeforeRequestSent,
	ResponseStarted,
	AuthRequired,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthCredentialsType {
	#[serde(rename = "password")]
	Password,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCredentials {
	#[serde(rename = "type")]
	pub r#type: AuthCredentialsType,

	#[serde(rename = "username")]
	pub username: String,

	#[serde(rename = "password")]
	pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InterceptPhase {
	BeforeRequestSent,
	ResponseStarted,
	AuthRequired,
}
