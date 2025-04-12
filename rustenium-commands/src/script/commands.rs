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
pub struct Disown {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: DisownParameters,
}

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
	#[serde(rename = "userActivation")]
	user_activation: Option<>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Evaluate {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: EvaluateParameters,
}

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
	#[serde(rename = "userActivation")]
	user_activation: Option<>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRealms {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: GetRealmsParameters,
}

pub struct GetRealmsParameters {
	#[serde(rename = "context")]
	context: Option<BrowsingContext>,
	#[serde(rename = "type")]
	type_: Option<RealmType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovePreloadScript {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: RemovePreloadScriptParameters,
}

pub struct RemovePreloadScriptParameters {
	#[serde(rename = "script")]
	script: PreloadScript,
}

