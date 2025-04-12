pub struct AuthChallenge {
	#[serde(rename = "scheme")]
	scheme: String,
	#[serde(rename = "realm")]
	realm: String,
}

pub struct AuthCredentials {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "username")]
	username: String,
	#[serde(rename = "password")]
	password: String,
}

pub enum BaseParameters {
	#[serde(rename = "context")]
	context: BrowsingContext | None,
	#[serde(rename = "isBlocked")]
	is_blocked: bool,
	#[serde(rename = "navigation")]
	navigation: Navigation | None,
	#[serde(rename = "redirectCount")]
	redirect_count: u32,
	#[serde(rename = "request")]
	request: RequestData,
	#[serde(rename = "timestamp")]
	timestamp: u32,
	#[serde(rename = "intercepts")]
	intercepts: Option<Vec<Intercept>>,
}

pub struct StringValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: String,
}

pub struct Base64Value {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: String,
}

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

pub struct CookieHeader {
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "value")]
	value: BytesValue,
}

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
	#[serde(rename = "tlsEnd")]
	tls_end: ,
	#[serde(rename = "requestStart")]
	request_start: f64,
	#[serde(rename = "responseStart")]
	response_start: f64,
	#[serde(rename = "responseHeadersEnd")]
	response_headers_end: ,
	#[serde(rename = "responseEnd")]
	response_end: f64,
}

pub struct Header {
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "value")]
	value: BytesValue,
}

pub struct Initiator {
	#[serde(rename = "columnNumber")]
	column_number: Option<u32>,
	#[serde(rename = "linpub enumber")]
	line_number: Option<u32>,
	#[serde(rename = "request")]
	request: Option<Request>,
	#[serde(rename = "stackTrace")]
	stack_trace: Option<StackTrace>,
	#[serde(rename = "type")]
	type_: Option<String | String | String | String>,
}

pub struct RequestData {
	#[serde(rename = "request")]
	request: Request,
	#[serde(rename = "url")]
	url: String,
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "headers")]
	headers: Vec<Header>,
	#[serde(rename = "cookies")]
	cookies: Vec<Cookie>,
	#[serde(rename = "headersSize")]
	headers_size: u32,
	#[serde(rename = "bodySize")]
	body_size: u32 | None,
	#[serde(rename = "destination")]
	destination: String,
	#[serde(rename = "initiatorType")]
	initiator_type: String | None,
	#[serde(rename = "timings")]
	timings: FetchTimingInfo,
}

pub struct ResponseContent {
	#[serde(rename = "size")]
	size: u32,
}

pub struct ResponseData {
	#[serde(rename = "url")]
	url: String,
	#[serde(rename = "protocol")]
	protocol: String,
	#[serde(rename = "status")]
	status: u32,
	#[serde(rename = "statusText")]
	status_text: String,
	#[serde(rename = "fromCache")]
	from_cache: bool,
	#[serde(rename = "headers")]
	headers: Vec<Header>,
	#[serde(rename = "mimeType")]
	mime_type: String,
	#[serde(rename = "bytesReceived")]
	bytes_received: u32,
	#[serde(rename = "headersSize")]
	headers_size: u32 | None,
	#[serde(rename = "bodySize")]
	body_size: u32 | None,
	#[serde(rename = "content")]
	content: ResponseContent,
	#[serde(rename = "authChallenges")]
	auth_challenges: Vec<AuthChallenge>,
}

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

pub enum UrlPattern {
	UrlPatternPattern(UrlPatternPattern),
	UrlPatternString(UrlPatternString),
}

pub struct UrlPatternPattern {
	#[serde(rename = "type")]
	type_: String,
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

pub struct UrlPatternString {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "pattern")]
	pattern: String,
}

