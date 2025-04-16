use serde::{Serialize, Deserialize};

use crate::script::types::{RemoteValue, Source, StackTrace};	

#[derive(Debug, Serialize, Deserialize)]
pub enum Entry {
	GenericLogEntry(GenericLogEntry),
	ConsoleLogEntry(ConsoleLogEntry),
	JavascriptLogEntry(JavascriptLogEntry),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseLogEntry {
	#[serde(rename = "level")]
	pub level: Level,
	#[serde(rename = "source")]
	pub source: Source,
	#[serde(rename = "text")]
	pub text: Option<String>,
	#[serde(rename = "timestamp")]
	pub timestamp: u32,
	#[serde(rename = "stackTrace")]
	pub stack_trace: Option<StackTrace>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
	Debug,
	Info,
	Warn,
	Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericLogEntry {
	#[serde(flatten)]
	pub base: BaseLogEntry,

	#[serde(rename = "type")]
	pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum ConsoleLogEntryType {
	#[serde(rename = "console")]
	Console,
}

#[derive(Debug, Serialize, Deserialize)]
enum JavascriptLogEntryType {
	#[serde(rename = "javascript")]
	Javascript,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsoleLogEntry {
	#[serde(flatten)]
	pub base: BaseLogEntry,

	#[serde(rename = "type")]
	pub r#type: ConsoleLogEntryType,

	#[serde(rename = "method")]
	pub method: String,

	#[serde(rename = "args")]
	pub args: Vec<RemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JavascriptLogEntry {
	#[serde(flatten)]
	pub base: BaseLogEntry,

	#[serde(rename = "type")]
	pub r#type: JavascriptLogEntryType,
}

