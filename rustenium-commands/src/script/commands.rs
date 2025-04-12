use serde::{Serialize, Deserialize};
use crate::script::types::{ChannelValue, EvaluateResult, Handle, LocalValue, PreloadScript, RealmInfo, RealmType, ResultOwnership, SerializationOptions, Target};
use crate::browser::types::UserContext;
use crate::browsing_context::types::BrowsingContext;
pub enum ScriptCommand {
	AddPreloadScript(AddPreloadScript),
	CallFunction(CallFunction),
	Disown(Disown),
	Evaluate(Evaluate),
	GetRealms(GetRealms),
	RemovePreloadScript(RemovePreloadScript),
}

pub enum ScriptResult {
	AddPreloadScriptResult(AddPreloadScriptResult),
	EvaluateResult(EvaluateResult),
	GetRealmsResult(GetRealmsResult),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPreloadScript {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: AddPreloadScriptParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AddPreloadScriptParameters {
	#[serde(rename = "functionDeclaration")]
	function_declaration: String,
	#[serde(rename = "arguments")]
	arguments: Option<Vec<ChannelValue>>,
	#[serde(rename = "contexts")]
	contexts: Option<Vec<BrowsingContext>>,
	#[serde(rename = "userContexts")]
	user_contexts: Option<Vec<UserContext>>,
	#[serde(rename = "sandbox")]
	sandbox: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPreloadScriptResult {
	#[serde(rename = "script")]
	script: PreloadScript,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRealmsResult {
	#[serde(rename = "realms")]
	realms: Vec<RealmInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disown {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: DisownParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisownParameters {
	#[serde(rename = "handles")]
	handles: Vec<Handle>,
	#[serde(rename = "target")]
	target: Target,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallFunction {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: CallFunctionParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallFunctionParameters {
	#[serde(rename = "functionDeclaration")]
	function_declaration: String,
	#[serde(rename = "awaitPromise")]
	await_promise: bool,
	#[serde(rename = "target")]
	target: Target,
	#[serde(rename = "arguments")]
	arguments: Option<Vec<LocalValue>>,
	#[serde(rename = "resultOwnership")]
	result_ownership: Option<ResultOwnership>,
	#[serde(rename = "serializationOptions")]
	serialization_options: Option<SerializationOptions>,
	#[serde(rename = "this")]
	this: Option<LocalValue>,
	#[serde(rename = "userActivation", default = "default_user_activation")]
	user_activation: bool,
}

fn default_user_activation() -> bool {
	false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Evaluate {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: EvaluateParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateParameters {
	#[serde(rename = "expression")]
	expression: String,
	#[serde(rename = "target")]
	target: Target,
	#[serde(rename = "awaitPromise")]
	await_promise: bool,
	#[serde(rename = "resultOwnership")]
	result_ownership: Option<ResultOwnership>,
	#[serde(rename = "serializationOptions")]
	serialization_options: Option<SerializationOptions>,
	#[serde(rename = "userActivation", default = "default_user_activation")]
	user_activation: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRealms {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: GetRealmsParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRealmsParameters {
	#[serde(rename = "context")]
	context: Option<BrowsingContext>,
	#[serde(rename = "type")]
	r#type: Option<RealmType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovePreloadScript {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: RemovePreloadScriptParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RemovePreloadScriptParameters {
	#[serde(rename = "script")]
	script: PreloadScript,
}

