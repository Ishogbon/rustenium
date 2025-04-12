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
pub struct ContinueWithAuth {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: ContinueWithAuthParameters,
}

pub struct ContinueWithAuthParameters {
	#[serde(rename = "request")]
	request: Request,
	ContinueWithAuthCredentials(ContinueWithAuthCredentials),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueWithAuthCredentials {
	#[serde(rename = "action")]
	action: String,
	#[serde(rename = "credentials")]
	credentials: AuthCredentials,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueWithAuthNoCredentials {
	#[serde(rename = "action")]
	action: String | String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailRequest {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: FailRequestParameters,
}

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

pub struct RemoveInterceptParameters {
	#[serde(rename = "intercept")]
	intercept: Intercept,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCacheBehavior {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: SetCacheBehaviorParameters,
}

pub struct SetCacheBehaviorParameters {
	#[serde(rename = "cacheBehavior")]
	cache_behavior: String | String,
	#[serde(rename = "contexts")]
	contexts: Option<Vec<BrowsingContext>>,
}

