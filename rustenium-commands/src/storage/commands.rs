pub enum StorageCommand {
	DeleteCookies(DeleteCookies),
	GetCookies(GetCookies),
	SetCookie(SetCookie),
}

pub enum StorageResult {
	DeleteCookiesResult(DeleteCookiesResult),
	GetCookiesResult(GetCookiesResult),
	SetCookieResult(SetCookieResult),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCookies {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: GetCookiesParameters,
}

pub struct CookieFilter {
	#[serde(rename = "name")]
	name: Option<String>,
	#[serde(rename = "value")]
	value: Option<BytesValue>,
	#[serde(rename = "domain")]
	domain: Option<String>,
	#[serde(rename = "path")]
	path: Option<String>,
	#[serde(rename = "size")]
	size: Option<u32>,
	#[serde(rename = "httpOnly")]
	http_only: Option<bool>,
	#[serde(rename = "secure")]
	secure: Option<bool>,
	#[serde(rename = "sameSite")]
	same_site: Option<SameSite>,
	#[serde(rename = "expiry")]
	expiry: Option<u32>,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

pub struct BrowsingContextPartitionDescriptor {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "context")]
	context: BrowsingContext,
}

pub struct StorageKeyPartitionDescriptor {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "userContext")]
	user_context: Option<String>,
	#[serde(rename = "sourceOrigin")]
	source_origin: Option<String>,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

pub enum PartitionDescriptor {
	BrowsingContextPartitionDescriptor(BrowsingContextPartitionDescriptor),
	StorageKeyPartitionDescriptor(StorageKeyPartitionDescriptor),
}

pub struct GetCookiesParameters {
	#[serde(rename = "filter")]
	filter: Option<CookieFilter>,
	#[serde(rename = "partition")]
	partition: Option<PartitionDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCookie {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: SetCookieParameters,
}

pub struct PartialCookie {
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "value")]
	value: BytesValue,
	#[serde(rename = "domain")]
	domain: String,
	#[serde(rename = "path")]
	path: Option<String>,
	#[serde(rename = "httpOnly")]
	http_only: Option<bool>,
	#[serde(rename = "secure")]
	secure: Option<bool>,
	#[serde(rename = "sameSite")]
	same_site: Option<SameSite>,
	#[serde(rename = "expiry")]
	expiry: Option<u32>,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

pub struct SetCookieParameters {
	#[serde(rename = "cookie")]
	cookie: PartialCookie,
	#[serde(rename = "partition")]
	partition: Option<PartitionDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCookies {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: DeleteCookiesParameters,
}

pub struct DeleteCookiesParameters {
	#[serde(rename = "filter")]
	filter: Option<CookieFilter>,
	#[serde(rename = "partition")]
	partition: Option<PartitionDescriptor>,
}

