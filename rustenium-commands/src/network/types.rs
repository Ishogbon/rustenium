use serde::{Deserialize, Serialize};

use crate::{browsing_context::types::{BrowsingContext, Navigation}, script::types::StackTrace};

pub type Intercept = String;
pub type Request = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthChallenge {
	#[serde(rename = "scheme")]
	scheme: String,
	#[serde(rename = "realm")]
	realm: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCredentials {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "username")]
	username: String,
	#[serde(rename = "password")]
	password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseParameters {
	#[serde(rename = "context")]
	pub context: Option<BrowsingContext>,
	#[serde(rename = "isBlocked")]
	pub is_blocked: bool,
	#[serde(rename = "navigation")]
	pub navigation: Option<Navigation>,
	#[serde(rename = "redirectCount")]
	pub redirect_count: u32,
	#[serde(rename = "request")]
	pub request: RequestData,
	#[serde(rename = "timestamp")]
	pub timestamp: u32,
	#[serde(rename = "intercepts")]
	pub intercepts: Option<Vec<Intercept>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringValue {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Base64Value {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BytesValue {
	String(StringValue),
	Base64(Base64Value),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cookie {
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "value")]
	value: BytesValue,
	#[serde(rename = "domain")]
	domain: String,
	#[serde(rename = "path")]
	path: String,
	#[serde(rename = "size")]
	size: u32,
	#[serde(rename = "httpOnly")]
	http_only: bool,
	#[serde(rename = "secure")]
	secure: bool,
	#[serde(rename = "sameSite")]
	same_site: SameSite,
	#[serde(rename = "expiry")]
	expiry: Option<u32>,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CookieHeader {
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "value")]
	value: BytesValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchTimingInfo {
	#[serde(rename = "timeOrigin")]
	time_origin: f64,
	#[serde(rename = "requestTime")]
	request_time: f64,
	#[serde(rename = "redirectStart")]
	redirect_start: f64,
	#[serde(rename = "redirectEnd")]
	redirect_end: f64,
	#[serde(rename = "fetchStart")]
	fetch_start: f64,
	#[serde(rename = "dnsStart")]
	dns_start: f64,
	#[serde(rename = "dnsEnd")]
	dns_end: f64,
	#[serde(rename = "connectStart")]
	connect_start: f64,
	#[serde(rename = "connectEnd")]
	connect_end: f64,
	#[serde(rename = "tlsStart")]
	tls_start: f64,
	#[serde(rename = "requestStart")]
	request_start: f64,
	#[serde(rename = "responseStart")]
	response_start: f64,
	#[serde(rename = "responseEnd")]
	response_end: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "value")]
	value: BytesValue,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InitiatorType {
	Parser,
	Script,
	Preflight,
	Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Initiator {
	#[serde(rename = "columnNumber")]
	pub column_number: Option<u32>,
	#[serde(rename = "lineNumber")]
	pub line_number: Option<u32>,
	#[serde(rename = "request")]
	pub request: Option<Request>,
	#[serde(rename = "stackTrace")]
	pub stack_trace: Option<StackTrace>,
	#[serde(rename = "type")]
	pub r#type: Option<InitiatorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestData {
	#[serde(rename = "request")]
	pub request: Request,
	#[serde(rename = "url")]
	pub url: String,
	#[serde(rename = "method")]
	pub method: String,
	#[serde(rename = "headers")]
	pub headers: Vec<Header>,
	#[serde(rename = "cookies")]
	pub cookies: Vec<Cookie>,
	#[serde(rename = "headersSize")]
	pub headers_size: u32,
	#[serde(rename = "bodySize")]
	pub body_size: Option<u32>,
	#[serde(rename = "destination")]
	pub destination: String,
	#[serde(rename = "initiatorType")]
	pub initiator_type: Option<String>,
	#[serde(rename = "timings")]
	pub timings: FetchTimingInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseContent {
	#[serde(rename = "size")]
	size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
	#[serde(rename = "url")]
	pub url: String,
	#[serde(rename = "protocol")]
	pub protocol: String,
	#[serde(rename = "status")]
	pub status: u32,
	#[serde(rename = "statusText")]
	pub status_text: String,
	#[serde(rename = "fromCache")]
	pub from_cache: bool,
	#[serde(rename = "headers")]
	pub headers: Vec<Header>,
	#[serde(rename = "mimeType")]
	pub mime_type: String,
	#[serde(rename = "bytesReceived")]
	pub bytes_received: u32,
	#[serde(rename = "headersSize")]
	pub headers_size: Option<u32>,
	#[serde(rename = "bodySize")]
	pub body_size: Option<u32>,
	#[serde(rename = "content")]
	pub content: ResponseContent,
	#[serde(rename = "authChallenges")]
	pub auth_challenges: Option<Vec<AuthChallenge>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCookieHeader {
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "value")]
	value: BytesValue,
	#[serde(rename = "domain")]
	domain: Option<String>,
	#[serde(rename = "httpOnly")]
	http_only: Option<bool>,
	#[serde(rename = "expiry")]
	expiry: Option<String>,
	#[serde(rename = "maxAge")]
	max_age: Option<i32>,
	#[serde(rename = "path")]
	path: Option<String>,
	#[serde(rename = "sameSite")]
	same_site: Option<SameSite>,
	#[serde(rename = "secure")]
	secure: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UrlPattern {
	UrlPatternPattern(UrlPatternPattern),
	UrlPatternString(UrlPatternString),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlPatternPattern {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "protocol")]
	protocol: String,
	#[serde(rename = "hostname")]
	hostname: String,
	#[serde(rename = "port")]
	port: String,
	#[serde(rename = "pathname")]
	pathname: String,
	#[serde(rename = "search")]
	search: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlPatternString {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "pattern")]
	pattern: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SameSite {
	Strict,
	Lax,
	None,
}

