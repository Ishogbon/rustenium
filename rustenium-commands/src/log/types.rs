pub enum Entry {
	GenericLogEntry(GenericLogEntry),
	ConsoleLogEntry(ConsoleLogEntry),
	JavascriptLogEntry(JavascriptLogEntry),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseLogEntry {
	#[serde(rename = "level")]
	level: Level,
	#[serde(rename = "source")]
	source: Source,
	#[serde(rename = "text")]
	text: Option<String>,
	#[serde(rename = "timestamp")]
	timestamp: u32,
	#[serde(rename = "stackTrace")]
	stack_trace: Option<StackTrace>,
}

