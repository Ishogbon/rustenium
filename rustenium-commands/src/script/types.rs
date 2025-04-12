use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use crate::browsing_context::types::BrowsingContext;

pub type Channel = String;
pub type Realm = String;
pub type ListLocalValue = Vec<LocalValue>;
pub type SharedId = String;
pub type Handle = String;
pub type InternalId = String;
pub type ListRemoteValue = Vec<RemoteValue>;

#[derive(Debug, Serialize, Deserialize)]
pub enum SpecialNumber {
	#[serde(rename = "NaN")]
	NaN,

	#[serde(rename = "-0")]
	NegativeZero,

	#[serde(rename = "Infinity")]
	Infinity,

	#[serde(rename = "-Infinity")]
	NegativeInfinity,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Number {
	Int(i64),
	Float(f64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocalValueOrText {
	LocalValue(LocalValue),
	Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingLocalValueEntry(
	pub LocalValueOrText,
	pub LocalValue,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingLocalValue(
	pub Vec<MappingLocalValueEntry>,
);


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoteValueOrText {
	RemoteValue(RemoteValue),
	Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingRemoteValueEntry(
	pub RemoteValueOrText,
	pub RemoteValue,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingRemoteValue(
	pub Vec<MappingRemoteValueEntry>
);


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResultOwnership {
	Root,
	None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: ChannelProperties,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateResultSuccess {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "result")]
	result: RemoteValue,
	#[serde(rename = "realm")]
	realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateResultException {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "exceptionDetails")]
	exception_details: ExceptionDetails,
	#[serde(rename = "realm")]
	realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
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


#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayLocalValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: ListLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateLocalValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapLocalValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: MappingLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectLocalValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: MappingLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpValue {
	#[serde(rename = "pattern")]
	pattern: String,
	#[serde(rename = "flags")]
	flags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpLocalValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: RegExpValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetLocalValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: ListLocalValue,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PrimitiveProtocolValue {
	UndefinedValue(UndefinedValue),
	NullValue(NullValue),
	StringValue(StringValue),
	NumberValue(NumberValue),
	BooleanValue(BooleanValue),
	BigIntValue(BigIntValue),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UndefinedValue {
	#[serde(rename = "type")]
	type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NullValue {
	#[serde(rename = "type")]
	type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NumberValueType {
	Number
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumberValueValue {
	Number(Number),
	SpecialNumber(SpecialNumber),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumberValue {
	#[serde(rename = "type")]
	type_: NumberValueType,
	#[serde(rename = "value")]
	value: NumberValueValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BigIntValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "value")]
	value: String,
}


#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WindowRealmInfoType {
	Window
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WindowRealmInfo {
	#[serde(flatten)]
base: BaseRealmInfo,
	#[serde(rename = "type")]
r#type: WindowRealmInfoType,
	#[serde(rename = "context")]
context: BrowsingContext,
	#[serde(rename = "sandbox")]
sandbox: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DedicatedWorkerRealmInfoType {
	#[serde(rename = "dedicated-worker")]
	DedicatedWorker
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DedicatedWorkerRealmInfo {
	#[serde(flatten)]
base: BaseRealmInfo,
	#[serde(rename="type")]
r#type: DedicatedWorkerRealmInfoType,
	#[serde(rename = "owners")]
owners: Vec<Realm>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SharedWorkerRealmInfoType {
	#[serde(rename = "shared-worker")]
	SharedWorker
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SharedWorkerRealmInfo {
	#[serde(flatten)]
base: BaseRealmInfo,
#[serde(rename="type")]
r#type: SharedWorkerRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceWorkerRealmInfoType {
	#[serde(rename = "service-worker")]
	ServiceWorker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceWorkerRealmInfo {
	#[serde(flatten)]
	base: BaseRealmInfo,
	#[serde(rename = "type")]
	r#type: ServiceWorkerRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkerRealmInfoType {
	#[serde(rename = "worker")]
	Worker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkerRealmInfo {
	#[serde(flatten)]
	base: BaseRealmInfo,
	#[serde(rename = "type")]
	r#type: WorkerRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PaintWorkletRealmInfoType {
	#[serde(rename = "paint-worklet")]
	PaintWorklet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaintWorkletRealmInfo {
	#[serde(flatten)]
	base: BaseRealmInfo,
	#[serde(rename = "type")]
	r#type: PaintWorkletRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AudioWorkletRealmInfoType {
	#[serde(rename = "audio-worklet")]
	AudioWorklet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioWorkletRealmInfo {
	#[serde(flatten)]
	base: BaseRealmInfo,
	#[serde(rename = "type")]
	r#type: AudioWorkletRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkletRealmInfoType {
	#[serde(rename = "worklet")]
	Worklet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkletRealmInfo {
	#[serde(flatten)]
	base: BaseRealmInfo,
	#[serde(rename = "type")]
	r#type: WorkletRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RemoteReference {
	SharedReference(SharedReference),
	RemoteObjectReference(RemoteObjectReference),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedReference {
	#[serde(rename = "sharedId")]
	shared_id: SharedId,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteObjectReference {
	#[serde(rename = "handle")]
	handle: Handle,
	#[serde(rename = "sharedId")]
	shared_id: Option<SharedId>,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpRemoteValue {
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,

	#[serde(flatten)]
	local: RegExpLocalValue,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DateRemoteValue {
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(flatten)]
	local: DateLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct WeakMapRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeakSetRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromiseRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypedArrayRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayBufferRemoteValue {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
	value: Option<Box<NodeProperties>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodePropertiesMode {
	Open,
	Closed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeProperties {
	#[serde(rename = "nodeType")]
	node_type: u32,
	#[serde(rename = "childNodeCount")]
	child_node_count: u32,
	#[serde(rename = "attributes")]
	attributes: Option<HashMap<String, String>>,
	#[serde(rename = "children")]
	children: Option<Vec<NodeRemoteValue>>,
	#[serde(rename = "localName")]
	local_name: Option<String>,
	#[serde(rename = "mode")]
	mode: Option<NodePropertiesMode>,
	#[serde(rename = "namespaceURI")]
	namespace_u_r_i: Option<String>,
	#[serde(rename = "nodeValue")]
	node_value: Option<String>,
	#[serde(rename = "shadowRoot")]
	shadow_root: Option<Box<NodeRemoteValue>>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowProxyProperties {
	#[serde(rename = "context")]
	context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IncludeShadowTree {
	None,
	Open,
	All,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializationOptions {
	#[serde(rename = "maxDomDepth")]
	max_dom_depth: Option<u32>,
	#[serde(rename = "maxObjectDepth")]
	max_object_depth: Option<u32>,
	#[serde(rename = "includeShadowTree", default = "default_include_shadow_tree")]
	include_shadow_tree: IncludeShadowTree,
}

fn default_include_shadow_tree() -> IncludeShadowTree {
	IncludeShadowTree::None
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct StackTrace {
	#[serde(rename = "callFrames")]
	call_frames: Vec<StackFrame>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
	#[serde(rename = "realm")]
	realm: Realm,
	#[serde(rename = "context")]
	context: Option<BrowsingContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmTarget {
	#[serde(rename = "realm")]
	realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextTarget {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "sandbox")]
	sandbox: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Target {
	ContextTarget(ContextTarget),
	RealmTarget(RealmTarget),
}
