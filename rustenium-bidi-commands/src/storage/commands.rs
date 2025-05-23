use serde::{Deserialize, Serialize};

use crate::{browsing_context::types::BrowsingContext, network::types::{BytesValue, Cookie, SameSite}};

use super::types::PartitionKey;


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StorageCommand {
	DeleteCookies(DeleteCookies),
	GetCookies(GetCookies),
	SetCookie(SetCookie),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StorageResult {
	DeleteCookiesResult(DeleteCookiesResult),
	GetCookiesResult(GetCookiesResult),
	SetCookieResult(SetCookieResult),
}

#[derive(Debug, Serialize, Deserialize)]
enum GetCookiesMethod {
	#[serde(rename = "storage.getCookies")]
	StorageGetCookies,
}

#[derive(Debug, Serialize, Deserialize)]
enum SetCookieMethod {
	#[serde(rename = "storage.setCookie")]
	StorageSetCookie,
}

#[derive(Debug, Serialize, Deserialize)]
enum DeleteCookiesMethod {
	#[serde(rename = "storage.deleteCookies")]
	StorageDeleteCookies,
}

#[derive(Debug, Serialize, Deserialize)]
enum BrowsingContextPartitionDescriptorType {
	#[serde(rename = "context")]
	Context,
}

#[derive(Debug, Serialize, Deserialize)]
enum StorageKeyPartitionDescriptorType {
	#[serde(rename = "storageKey")]
	StorageKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCookies {
	#[serde(rename = "method")]
	pub method: GetCookiesMethod,
	#[serde(rename = "params")]
	pub params: GetCookiesParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CookieFilter {
	#[serde(skip_serializing_if = "Option::is_none", rename = "name")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "value")]
	pub value: Option<BytesValue>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "domain")]
	pub domain: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "path")]
	pub path: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "size")]
	pub size: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "httpOnly")]
	pub http_only: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "secure")]
	pub secure: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "sameSite")]
	pub same_site: Option<SameSite>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "expiry")]
	pub expiry: Option<u32>,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrowsingContextPartitionDescriptor {
	#[serde(rename = "type")]
	pub r#type: BrowsingContextPartitionDescriptorType,
	#[serde(rename = "context")]
	pub context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageKeyPartitionDescriptor {
	#[serde(rename = "type")]
	pub r#type: StorageKeyPartitionDescriptorType,
	#[serde(skip_serializing_if = "Option::is_none", rename = "userContext")]
	pub user_context: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "sourceOrigin")]
	pub source_origin: Option<String>,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PartitionDescriptor {
	BrowsingContextPartitionDescriptor(BrowsingContextPartitionDescriptor),
	StorageKeyPartitionDescriptor(StorageKeyPartitionDescriptor),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCookiesParameters {
	#[serde(skip_serializing_if = "Option::is_none", rename = "filter")]
	pub filter: Option<CookieFilter>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "partition")]
	pub partition: Option<PartitionDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCookie {
	#[serde(rename = "method")]
	pub method: SetCookieMethod,
	#[serde(rename = "params")]
	pub params: SetCookieParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialCookie {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "value")]
	pub value: BytesValue,
	#[serde(rename = "domain")]
	pub domain: String,
	#[serde(skip_serializing_if = "Option::is_none", rename = "path")]
	pub path: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "httpOnly")]
	pub http_only: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "secure")]
	pub secure: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "sameSite")]
	pub same_site: Option<SameSite>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "expiry")]
	pub expiry: Option<u32>,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCookieParameters {
	#[serde(rename = "cookie")]
	pub cookie: PartialCookie,
	#[serde(skip_serializing_if = "Option::is_none", rename = "partition")]
	pub partition: Option<PartitionDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCookies {
	#[serde(rename = "method")]
	pub method: DeleteCookiesMethod,
	#[serde(rename = "params")]
	pub params: DeleteCookiesParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCookiesParameters {
	#[serde(skip_serializing_if = "Option::is_none", rename = "filter")]
	pub filter: Option<CookieFilter>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "partition")]
	pub partition: Option<PartitionDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCookiesResult {
	#[serde(rename = "partitionKey")]
	pub partition_key: PartitionKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCookiesResult {
	#[serde(rename = "cookies")]
	pub cookies: Vec<Cookie>,

	#[serde(rename = "partitionKey")]
	pub partition_key: PartitionKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCookieResult {
	#[serde(rename = "partitionKey")]
	pub partition_key: PartitionKey,
}

