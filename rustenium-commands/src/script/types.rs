pub struct ChannelValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: ChannelProperties,
}

pub struct ChannelProperties {
	#[serde(rename = "channel")]
	channel: Channel,
	#[serde(rename = "serializationOptions")]
	serialization_options: Option<SerializationOptions>,
	#[serde(rename = "ownership")]
	ownership: Option<ResultOwnership>,
}

pub enum EvaluateResult {
	EvaluateResultSuccess(EvaluateResultSuccess),
	EvaluateResultException(EvaluateResultException),
}

pub struct EvaluateResultSuccess {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "result")]
	result: RemoteValue,
	#[serde(rename = "realm")]
	realm: Realm,
}

pub struct EvaluateResultException {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "exceptionDetails")]
	exception_details: ExceptionDetails,
	#[serde(rename = "realm")]
	realm: Realm,
}

pub struct ExceptionDetails {
	#[serde(rename = "columnNumber")]
	column_number: u32,
	#[serde(rename = "exception")]
	exception: RemoteValue,
	#[serde(rename = "linpub enumber")]
	line_number: u32,
	#[serde(rename = "stackTrace")]
	stack_trace: StackTrace,
	#[serde(rename = "text")]
	text: String,
}

pub enum LocalValue {
	RemoteReference(RemoteReference),
	PrimitiveProtocolValue(PrimitiveProtocolValue),
	ChannelValue(ChannelValue),
	ArrayLocalValue(ArrayLocalValue),
	DateLocalValue(DateLocalValue),
	MapLocalValue(MapLocalValue),
	ObjectLocalValue(ObjectLocalValue),
	RegExpLocalValue(RegExpLocalValue),
	SetLocalValue(SetLocalValue),
}

pub struct ArrayLocalValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: ListLocalValue,
}

pub struct DateLocalValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: String,
}

pub struct MapLocalValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: MappingLocalValue,
}

pub struct ObjectLocalValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: MappingLocalValue,
}

pub struct RegExpValue {
	#[serde(rename = "pattern")]
	pattern: String,
	#[serde(rename = "flags")]
	flags: Option<String>,
}

pub struct RegExpLocalValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: RegExpValue,
}

pub struct SetLocalValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: ListLocalValue,
}

pub enum PrimitiveProtocolValue {
	UndefinedValue(UndefinedValue),
	NullValue(NullValue),
	StringValue(StringValue),
	NumberValue(NumberValue),
	BooleanValue(BooleanValue),
	BigIntValue(BigIntValue),
}

pub struct UndefinedValue {
	#[serde(rename = "type")]
	type_: String,
}

pub struct NullValue {
	#[serde(rename = "type")]
	type_: String,
}

pub struct StringValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: String,
}

pub struct NumberValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value:  | SpecialNumber,
}

pub struct BooleanValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: bool,
}

pub struct BigIntValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: String,
}

pub enum RealmInfo {
	WindowRealmInfo(WindowRealmInfo),
	DedicatedWorkerRealmInfo(DedicatedWorkerRealmInfo),
	SharedWorkerRealmInfo(SharedWorkerRealmInfo),
	ServiceWorkerRealmInfo(ServiceWorkerRealmInfo),
	WorkerRealmInfo(WorkerRealmInfo),
	PaintWorkletRealmInfo(PaintWorkletRealmInfo),
	AudioWorkletRealmInfo(AudioWorkletRealmInfo),
	WorkletRealmInfo(WorkletRealmInfo),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealmInfo {
	#[serde(rename = "realm")]
	realm: Realm,
	#[serde(rename = "origin")]
	origin: String,
}

pub enum RemoteReference {
	SharedReference(SharedReference),
	RemoteObjectReference(RemoteObjectReference),
}

pub struct SharedReference {
	#[serde(rename = "sharedId")]
	shared_id: SharedId,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

pub struct RemoteObjectReference {
	#[serde(rename = "handle")]
	handle: Handle,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
	#[serde(rename = "sharedId")]
	shared_id: Option<SharedId>,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

pub enum RemoteValue {
	PrimitiveProtocolValue(PrimitiveProtocolValue),
	SymbolRemoteValue(SymbolRemoteValue),
	ArrayRemoteValue(ArrayRemoteValue),
	ObjectRemoteValue(ObjectRemoteValue),
	FunctionRemoteValue(FunctionRemoteValue),
	RegExpRemoteValue(RegExpRemoteValue),
	DateRemoteValue(DateRemoteValue),
	MapRemoteValue(MapRemoteValue),
	SetRemoteValue(SetRemoteValue),
	WeakMapRemoteValue(WeakMapRemoteValue),
	WeakSetRemoteValue(WeakSetRemoteValue),
	GeneratorRemoteValue(GeneratorRemoteValue),
	ErrorRemoteValue(ErrorRemoteValue),
	ProxyRemoteValue(ProxyRemoteValue),
	PromiseRemoteValue(PromiseRemoteValue),
	TypedArrayRemoteValue(TypedArrayRemoteValue),
	ArrayBufferRemoteValue(ArrayBufferRemoteValue),
	NodeListRemoteValue(NodeListRemoteValue),
	HTMLCollectionRemoteValue(HTMLCollectionRemoteValue),
	NodeRemoteValue(NodeRemoteValue),
	WindowProxyRemoteValue(WindowProxyRemoteValue),
}

pub struct SymbolRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

pub struct ArrayRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<ListRemoteValue>,
}

pub struct ObjectRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<MappingRemoteValue>,
}

pub struct FunctionRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

pub struct RegExpRemoteValue {
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	RegExpLocalValue(RegExpLocalValue),
pub struct DateRemoteValue {
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	DateLocalValue(DateLocalValue),
pub struct MapRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<MappingRemoteValue>,
}

pub struct SetRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<ListRemoteValue>,
}

pub struct WeakMapRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

pub struct WeakSetRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

pub struct GeneratorRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

pub struct ErrorRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

pub struct ProxyRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

pub struct PromiseRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

pub struct TypedArrayRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

pub struct ArrayBufferRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

pub struct NodeListRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<ListRemoteValue>,
}

pub struct HTMLCollectionRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<ListRemoteValue>,
}

pub struct NodeRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "sharedId")]
	shared_id: Option<SharedId>,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<NodeProperties>,
}

pub struct NodeProperties {
	#[serde(rename = "nodeType")]
	node_type: u32,
	#[serde(rename = "childNodeCount")]
	child_node_count: u32,
	#[serde(rename = "attributes")]
	attributes: Option<>,
	#[serde(rename = "children")]
	children: Option<Vec<NodeRemoteValue>>,
	#[serde(rename = "localName")]
	local_name: Option<String>,
	#[serde(rename = "mode")]
	mode: Option<String | String>,
	#[serde(rename = "namespaceURI")]
	namespace_u_r_i: Option<String>,
	#[serde(rename = "nodeValue")]
	node_value: Option<String>,
	#[serde(rename = "shadowRoot")]
	shadow_root: Option<NodeRemoteValue | None>,
}

pub struct WindowProxyRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: WindowProxyProperties,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

pub struct WindowProxyProperties {
	#[serde(rename = "context")]
	context: BrowsingContext,
}

pub struct SerializationOptions {
	#[serde(rename = "maxDomDepth")]
	max_dom_depth: Option< | >,
	#[serde(rename = "maxObjectDepth")]
	max_object_depth: Option< | >,
	#[serde(rename = "includeShadowTree")]
	include_shadow_tree: Option<String | String | String>,
}

pub struct StackFrame {
	#[serde(rename = "columnNumber")]
	column_number: u32,
	#[serde(rename = "functionName")]
	function_name: String,
	#[serde(rename = "linpub enumber")]
	line_number: u32,
	#[serde(rename = "url")]
	url: String,
}

pub struct StackTrace {
	#[serde(rename = "callFrames")]
	call_frames: Vec<StackFrame>,
}

pub struct Source {
	#[serde(rename = "realm")]
	realm: Realm,
	#[serde(rename = "context")]
	context: Option<BrowsingContext>,
}

pub struct RealmTarget {
	#[serde(rename = "realm")]
	realm: Realm,
}

pub struct ContextTarget {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "sandbox")]
	sandbox: Option<String>,
}

pub enum Target {
	ContextTarget(ContextTarget),
	RealmTarget(RealmTarget),
}

