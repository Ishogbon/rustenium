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
enum AddPreloadScriptMethod {
	#[serde(rename = "script.addPreloadScript")]
	ScriptAddPreloadScript,
}

#[derive(Debug, Serialize, Deserialize)]
enum DisownMethod {
	#[serde(rename = "script.disown")]
	ScriptDisown,
}

#[derive(Debug, Serialize, Deserialize)]
enum CallFunctionMethod {
	#[serde(rename = "script.callFunction")]
	ScriptCallFunction,
}

#[derive(Debug, Serialize, Deserialize)]
enum EvaluateMethod {
	#[serde(rename = "script.evaluate")]
	ScriptEvaluate,
}

#[derive(Debug, Serialize, Deserialize)]
enum GetRealmsMethod {
	#[serde(rename = "script.getRealms")]
	ScriptGetRealms,
}

#[derive(Debug, Serialize, Deserialize)]
enum RemovePreloadScriptMethod {
	#[serde(rename = "script.removePreloadScript")]
	ScriptRemovePreloadScript,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPreloadScript {
	#[serde(rename = "method")]
	pub method: AddPreloadScriptMethod,
	#[serde(rename = "params")]
	pub params: AddPreloadScriptParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPreloadScriptParameters {
	#[serde(rename = "functionDeclaration")]
	pub function_declaration: String,
	#[serde(rename = "arguments")]
	pub arguments: Option<Vec<ChannelValue>>,
	#[serde(rename = "contexts")]
	pub contexts: Option<Vec<BrowsingContext>>,
	#[serde(rename = "userContexts")]
	pub user_contexts: Option<Vec<UserContext>>,
	#[serde(rename = "sandbox")]
	pub sandbox: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPreloadScriptResult {
	#[serde(rename = "script")]
	pub script: PreloadScript,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRealmsResult {
	#[serde(rename = "realms")]
	pub realms: Vec<RealmInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disown {
	#[serde(rename = "method")]
	pub method: DisownMethod,
	#[serde(rename = "params")]
	pub params: DisownParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisownParameters {
	#[serde(rename = "handles")]
	pub handles: Vec<Handle>,
	#[serde(rename = "target")]
	pub target: Target,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallFunction {
	#[serde(rename = "method")]
	pub method: CallFunctionMethod,
	#[serde(rename = "params")]
	pub params: CallFunctionParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallFunctionParameters {
	#[serde(rename = "functionDeclaration")]
	pub function_declaration: String,
	#[serde(rename = "awaitPromise")]
	pub await_promise: bool,
	#[serde(rename = "target")]
	pub target: Target,
	#[serde(rename = "arguments")]
	pub arguments: Option<Vec<LocalValue>>,
	#[serde(rename = "resultOwnership")]
	pub result_ownership: Option<ResultOwnership>,
	#[serde(rename = "serializationOptions")]
	pub serialization_options: Option<SerializationOptions>,
	#[serde(rename = "this")]
	pub this: Option<LocalValue>,
	#[serde(rename = "userActivation", default = "default_user_activation")]
	pub user_activation: bool,
}

fn default_user_activation() -> bool {
	false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Evaluate {
	#[serde(rename = "method")]
	pub method: EvaluateMethod,
	#[serde(rename = "params")]
	pub params: EvaluateParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateParameters {
	#[serde(rename = "expression")]
	pub expression: String,
	#[serde(rename = "target")]
	pub target: Target,
	#[serde(rename = "awaitPromise")]
	pub await_promise: bool,
	#[serde(rename = "resultOwnership")]
	pub result_ownership: Option<ResultOwnership>,
	#[serde(rename = "serializationOptions")]
	pub serialization_options: Option<SerializationOptions>,
	#[serde(rename = "userActivation", default = "default_user_activation")]
	pub user_activation: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRealms {
	#[serde(rename = "method")]
	pub method: GetRealmsMethod,
	#[serde(rename = "params")]
	pub params: GetRealmsParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRealmsParameters {
	#[serde(rename = "context")]
	pub context: Option<BrowsingContext>,
	#[serde(rename = "type")]
	pub r#type: Option<RealmType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovePreloadScript {
	#[serde(rename = "method")]
	pub method: RemovePreloadScriptMethod,
	#[serde(rename = "params")]
	pub params: RemovePreloadScriptParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovePreloadScriptParameters {
	#[serde(rename = "script")]
	pub script: PreloadScript,
}

