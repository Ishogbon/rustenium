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
enum AddInterceptMethod {
	#[serde(rename = "network.addIntercept")]
	NetworkAddIntercept,
}

#[derive(Debug, Serialize, Deserialize)]
enum ContinueRequestMethod {
	#[serde(rename = "network.continueRequest")]
	NetworkContinueRequest,
}

#[derive(Debug, Serialize, Deserialize)]
enum ContinueResponseMethod {
	#[serde(rename = "network.continueResponse")]
	NetworkContinueResponse,
}

#[derive(Debug, Serialize, Deserialize)]
enum FailRequestMethod {
	#[serde(rename = "network.failRequest")]
	NetworkFailRequest,
}

#[derive(Debug, Serialize, Deserialize)]
enum ProvideResponseMethod {
	#[serde(rename = "network.provideResponse")]
	NetworkProvideResponse,
}

#[derive(Debug, Serialize, Deserialize)]
enum RemoveInterceptMethod {
	#[serde(rename = "network.removeIntercept")]
	NetworkRemoveIntercept,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddIntercept {
	#[serde(rename = "method")]
	pub method: AddInterceptMethod,
	#[serde(rename = "params")]
	pub params: AddInterceptParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddInterceptParameters {
	#[serde(rename = "phases")]
	pub phases: Vec<InterceptPhase>,
	#[serde(rename = "contexts")]
	pub contexts: Option<Vec<BrowsingContext>>,
	#[serde(rename = "urlPatterns")]
	pub url_patterns: Option<Vec<UrlPattern>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueRequest {
	#[serde(rename = "method")]
	pub method: ContinueRequestMethod,
	#[serde(rename = "params")]
	pub params: ContinueRequestParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueRequestParameters {
	#[serde(rename = "request")]
	pub request: Request,
	#[serde(rename = "body")]
	pub body: BytesValue,
	#[serde(rename = "cookies")]
	pub cookies: Vec<CookieHeader>,
	#[serde(rename = "headers")]
	pub headers: Vec<Header>,
	#[serde(rename = "method")]
	pub method: String,
	#[serde(rename = "url")]
	pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueResponse {
	#[serde(rename = "method")]
	pub method: ContinueResponseMethod,
	#[serde(rename = "params")]
	pub params: ContinueResponseParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueResponseParameters {
	#[serde(rename = "request")]
	pub request: Request,
	#[serde(rename = "cookies")]
	pub cookies: Vec<SetCookieHeader>,
	#[serde(rename = "credentials")]
	pub credentials: AuthCredentials,
	#[serde(rename = "headers")]
	pub headers: Vec<Header>,
	#[serde(rename = "reasonPhrase")]
	pub reason_phrase: String,
	#[serde(rename = "statusCode")]
	pub status_code: u32,
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
	pub method: FailRequestMethod,
	#[serde(rename = "params")]
	pub params: FailRequestParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailRequestParameters {
	#[serde(rename = "request")]
	pub request: Request,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvideResponse {
	#[serde(rename = "method")]
	pub method: ProvideResponseMethod,
	#[serde(rename = "params")]
	pub params: ProvideResponseParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvideResponseParameters {
	#[serde(rename = "request")]
	pub request: Request,
	#[serde(rename = "body")]
	pub body: BytesValue,
	#[serde(rename = "cookies")]
	pub cookies: Vec<SetCookieHeader>,
	#[serde(rename = "headers")]
	pub headers: Vec<Header>,
	#[serde(rename = "reasonPhrase")]
	pub reason_phrase: String,
	#[serde(rename = "statusCode")]
	pub status_code: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveIntercept {
	#[serde(rename = "method")]
	pub method: RemoveInterceptMethod,
	#[serde(rename = "params")]
	pub params: RemoveInterceptParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveInterceptParameters {
	#[serde(rename = "intercept")]
	pub intercept: Intercept,
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
