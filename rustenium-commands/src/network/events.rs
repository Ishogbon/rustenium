pub enum NetworkEvent {
	AuthRequired(AuthRequired),
	BeforeRequestSent(BeforeRequestSent),
	FetchError(FetchError),
	ResponseCompleted(ResponseCompleted),
	ResponseStarted(ResponseStarted),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequired {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: AuthRequiredParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeforeRequestSent {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: BeforeRequestSentParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchError {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: FetchErrorParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCompleted {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: ResponseCompletedParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseStarted {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: ResponseStartedParameters,
}

