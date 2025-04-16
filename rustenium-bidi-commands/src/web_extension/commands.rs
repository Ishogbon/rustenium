use serde::{Deserialize, Serialize};

pub type Extension = String;

pub enum WebExtensionCommand {
	Install(Install),
	Uninstall(Uninstall),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallResult {
	#[serde(rename = "extension")]
	pub extension: Extension,
}

#[derive(Debug, Serialize, Deserialize)]
enum InstallMethod {
	#[serde(rename = "webExtension.install")]
	WebExtensionInstall,
}

#[derive(Debug, Serialize, Deserialize)]
enum ExtensionPathType {
	#[serde(rename = "path")]
	Path,
}

#[derive(Debug, Serialize, Deserialize)]
enum ExtensionArchivePathType {
	#[serde(rename = "archivePath")]
	ArchivePath,
}

#[derive(Debug, Serialize, Deserialize)]
enum ExtensionBase64EncodedType {
	#[serde(rename = "base64")]
	Base64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Install {
	#[serde(rename = "method")]
	pub method: InstallMethod,
	#[serde(rename = "params")]
	pub params: InstallParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallParameters {
	#[serde(rename = "extensionData")]
	pub extension_data: ExtensionData,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExtensionData {
	ExtensionArchivePath(ExtensionArchivePath),
	ExtensionBase64Encoded(ExtensionBase64Encoded),
	ExtensionPath(ExtensionPath),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionPath {
	#[serde(rename = "type")]
	pub r#type: ExtensionPathType,
	#[serde(rename = "path")]
	pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionArchivePath {
	#[serde(rename = "type")]
	pub r#type: ExtensionArchivePathType,
	#[serde(rename = "path")]
	pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionBase64Encoded {
	#[serde(rename = "type")]
	pub r#type: ExtensionBase64EncodedType,
	#[serde(rename = "value")]
	pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum UninstallMethod {
	#[serde(rename = "webExtension.uninstall")]
	WebExtensionUninstall,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Uninstall {
	#[serde(rename = "method")]
	pub method: UninstallMethod,
	#[serde(rename = "params")]
	pub params: UninstallParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UninstallParameters {
	#[serde(rename = "extension")]
	pub extension: Extension,
}
