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
pub type PreloadScript = String;

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
enum ChannelValueType {
	#[serde(rename = "channel")]
	Channel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelValue {
	#[serde(rename = "type")]
	pub r#type: ChannelValueType,
	#[serde(rename = "value")]
	pub value: ChannelProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelProperties {
	#[serde(rename = "channel")]
	pub channel: Channel,
	#[serde(rename = "serializationOptions")]
	pub serialization_options: Option<SerializationOptions>,
	#[serde(rename = "ownership")]
	pub ownership: Option<ResultOwnership>,
}

pub enum EvaluateResult {
	EvaluateResultSuccess(EvaluateResultSuccess),
	EvaluateResultException(EvaluateResultException),
}

#[derive(Debug, Serialize, Deserialize)]
enum EvaluateResultSuccessType {
	#[serde(rename = "success")]
	Success,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateResultSuccess {
	#[serde(rename = "type")]
	pub r#type: EvaluateResultSuccessType,
	#[serde(rename = "result")]
	pub result: RemoteValue,
	#[serde(rename = "realm")]
	pub realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
enum EvaluateResultExceptionType {
	#[serde(rename = "exception")]
	Exception,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateResultException {
	#[serde(rename = "type")]
	pub r#type: EvaluateResultExceptionType,
	#[serde(rename = "exceptionDetails")]
	pub exception_details: ExceptionDetails,
	#[serde(rename = "realm")]
	pub realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionDetails {
	#[serde(rename = "columnNumber")]
	pub column_number: u32,
	#[serde(rename = "exception")]
	pub exception: RemoteValue,
	#[serde(rename = "lineNumber")]
	pub line_number: u32,
	#[serde(rename = "stackTrace")]
	pub stack_trace: StackTrace,
	#[serde(rename = "text")]
	pub text: String,
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
enum ArrayLocalValueType {
	#[serde(rename = "array")]
	Array,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayLocalValue {
	#[serde(rename = "type")]
	pub r#type: ArrayLocalValueType,
	#[serde(rename = "value")]
	pub value: ListLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
enum DateLocalValueType {
	#[serde(rename = "date")]
	Date,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateLocalValue {
	#[serde(rename = "type")]
	pub r#type: DateLocalValueType,
	#[serde(rename = "value")]
	pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum MapLocalValueType {
	#[serde(rename = "map")]
	Map,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapLocalValue {
	#[serde(rename = "type")]
	pub r#type: MapLocalValueType,
	#[serde(rename = "value")]
	pub value: MappingLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
enum ObjectLocalValueType {
	#[serde(rename = "object")]
	Object,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectLocalValue {
	#[serde(rename = "type")]
	pub r#type: ObjectLocalValueType,
	#[serde(rename = "value")]
	pub value: MappingLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpValue {
	#[serde(rename = "pattern")]
	pub pattern: String,
	#[serde(rename = "flags")]
	pub flags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum RegExpLocalValueType {
	#[serde(rename = "regexp")]
	RegExp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpLocalValue {
	#[serde(rename = "type")]
	pub r#type: RegExpLocalValueType,
	#[serde(rename = "value")]
	pub value: RegExpValue,
}

#[derive(Debug, Serialize, Deserialize)]
enum SetLocalValueType {
	#[serde(rename = "set")]
	Set,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetLocalValue {
	#[serde(rename = "type")]
	pub r#type: SetLocalValueType,
	#[serde(rename = "value")]
	pub value: ListLocalValue,
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
enum UndefinedValueType {
	#[serde(rename = "undefined")]
	Undefined,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UndefinedValue {
	#[serde(rename = "type")]
	pub r#type: UndefinedValueType,
}

#[derive(Debug, Serialize, Deserialize)]
enum NullValueType {
	#[serde(rename = "null")]
	Null,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NullValue {
	#[serde(rename = "type")]
	pub r#type: NullValueType,
}

#[derive(Debug, Serialize, Deserialize)]
enum StringValueType {
	#[serde(rename = "string")]
	String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringValue {
	#[serde(rename = "type")]
	pub r#type: StringValueType,
	#[serde(rename = "value")]
	pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum NumberValueType {
	#[serde(rename = "number")]
	Number,
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
	pub r#type: NumberValueType,
	#[serde(rename = "value")]
	pub value: NumberValueValue,
}

#[derive(Debug, Serialize, Deserialize)]
enum BooleanValueType {
	#[serde(rename = "boolean")]
	Boolean,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanValue {
	#[serde(rename = "type")]
	pub r#type: BooleanValueType,
	#[serde(rename = "value")]
	pub value: bool,
}

#[derive(Debug, Serialize, Deserialize)]
enum BigIntValueType {
	#[serde(rename = "bigint")]
	BigInt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BigIntValue {
	#[serde(rename = "type")]
	pub r#type: BigIntValueType,
	#[serde(rename = "value")]
	pub value: String,
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
	pub realm: Realm,
	#[serde(rename = "origin")]
	pub origin: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RealmType {
	Window,
	DedicatedWorker,
	SharedWorker,
	ServiceWorker,
	Worker,
	PaintWorklet,
	AudioWorklet,
	Worklet,
}

#[derive(Debug, Serialize, Deserialize)]
enum WindowRealmInfoType {
	#[serde(rename = "window")]
	Window,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowRealmInfo {
	#[serde(flatten)]
	pub base: BaseRealmInfo,
	#[serde(rename = "type")]
	pub r#type: WindowRealmInfoType,
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "sandbox")]
	pub sandbox: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum DedicatedWorkerRealmInfoType {
	#[serde(rename = "dedicated-worker")]
	DedicatedWorker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DedicatedWorkerRealmInfo {
	#[serde(flatten)]
	pub base: BaseRealmInfo,
	#[serde(rename="type")]
	pub r#type: DedicatedWorkerRealmInfoType,
	#[serde(rename = "owners")]
	pub owners: Vec<Realm>
}

#[derive(Debug, Serialize, Deserialize)]
enum SharedWorkerRealmInfoType {
	#[serde(rename = "shared-worker")]
	SharedWorker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedWorkerRealmInfo {
	#[serde(flatten)]
	pub base: BaseRealmInfo,
	#[serde(rename="type")]
	pub r#type: SharedWorkerRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
enum ServiceWorkerRealmInfoType {
	#[serde(rename = "service-worker")]
	ServiceWorker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceWorkerRealmInfo {
	#[serde(flatten)]
	pub base: BaseRealmInfo,
	#[serde(rename = "type")]
	pub r#type: ServiceWorkerRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
enum WorkerRealmInfoType {
	#[serde(rename = "worker")]
	Worker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkerRealmInfo {
	#[serde(flatten)]
	pub base: BaseRealmInfo,
	#[serde(rename = "type")]
	pub r#type: WorkerRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
enum PaintWorkletRealmInfoType {
	#[serde(rename = "paint-worklet")]
	PaintWorklet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaintWorkletRealmInfo {
	#[serde(flatten)]
	pub base: BaseRealmInfo,
	#[serde(rename = "type")]
	pub r#type: PaintWorkletRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
enum AudioWorkletRealmInfoType {
	#[serde(rename = "audio-worklet")]
	AudioWorklet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioWorkletRealmInfo {
	#[serde(flatten)]
	pub base: BaseRealmInfo,
	#[serde(rename = "type")]
	pub r#type: AudioWorkletRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
enum WorkletRealmInfoType {
	#[serde(rename = "worklet")]
	Worklet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkletRealmInfo {
	#[serde(flatten)]
	pub base: BaseRealmInfo,
	#[serde(rename = "type")]
	pub r#type: WorkletRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RemoteReference {
	SharedReference(SharedReference),
	RemoteObjectReference(RemoteObjectReference),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedReference {
	#[serde(rename = "sharedId")]
	pub shared_id: SharedId,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteObjectReference {
	#[serde(rename = "handle")]
	pub handle: Handle,
	#[serde(rename = "sharedId")]
	pub shared_id: Option<SharedId>,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
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
enum SymbolRemoteValueType {
	#[serde(rename = "symbol")]
	Symbol,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolRemoteValue {
	#[serde(rename = "type")]
	pub r#type: SymbolRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum ArrayRemoteValueType {
	#[serde(rename = "array")]
	Array,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayRemoteValue {
	#[serde(rename = "type")]
	pub r#type: ArrayRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	pub value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
enum ObjectRemoteValueType {
	#[serde(rename = "object")]
	Object,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRemoteValue {
	#[serde(rename = "type")]
	pub r#type: ObjectRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	pub value: Option<MappingRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
enum FunctionRemoteValueType {
	#[serde(rename = "function")]
	Function,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionRemoteValue {
	#[serde(rename = "type")]
	pub r#type: FunctionRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpRemoteValue {
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,

	#[serde(flatten)]
	pub local: RegExpLocalValue,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DateRemoteValue {
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
	#[serde(flatten)]
	pub local: DateLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
enum MapRemoteValueType {
	#[serde(rename = "map")]
	Map,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapRemoteValue {
	#[serde(rename = "type")]
	pub r#type: MapRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	pub value: Option<MappingRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
enum SetRemoteValueType {
	#[serde(rename = "set")]
	Set,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetRemoteValue {
	#[serde(rename = "type")]
	pub r#type: SetRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	pub value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
enum WeakMapRemoteValueType {
	#[serde(rename = "weakmap")]
	WeakMap,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeakMapRemoteValue {
	#[serde(rename = "type")]
	pub r#type: WeakMapRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum WeakSetRemoteValueType {
	#[serde(rename = "weakset")]
	WeakSet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeakSetRemoteValue {
	#[serde(rename = "type")]
	pub r#type: WeakSetRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum GeneratorRemoteValueType {
	#[serde(rename = "generator")]
	Generator,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorRemoteValue {
	#[serde(rename = "type")]
	pub r#type: GeneratorRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum ErrorRemoteValueType {
	#[serde(rename = "error")]
	Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorRemoteValue {
	#[serde(rename = "type")]
	pub r#type: ErrorRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum ProxyRemoteValueType {
	#[serde(rename = "proxy")]
	Proxy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyRemoteValue {
	#[serde(rename = "type")]
	pub r#type: ProxyRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum PromiseRemoteValueType {
	#[serde(rename = "promise")]
	Promise,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromiseRemoteValue {
	#[serde(rename = "type")]
	pub r#type: PromiseRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum TypedArrayRemoteValueType {
	#[serde(rename = "typedarray")]
	TypedArray,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypedArrayRemoteValue {
	#[serde(rename = "type")]
	pub r#type: TypedArrayRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum ArrayBufferRemoteValueType {
	#[serde(rename = "arraybuffer")]
	ArrayBuffer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayBufferRemoteValue {
	#[serde(rename = "type")]
	pub r#type: ArrayBufferRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum NodeListRemoteValueType {
	#[serde(rename = "nodelist")]
	NodeList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeListRemoteValue {
	#[serde(rename = "type")]
	pub r#type: NodeListRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	pub value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
enum HTMLCollectionRemoteValueType {
	#[serde(rename = "htmlcollection")]
	HtmlCollection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HTMLCollectionRemoteValue {
	#[serde(rename = "type")]
	pub r#type: HTMLCollectionRemoteValueType,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	pub value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
enum NodeRemoteValueType {
	#[serde(rename = "node")]
	Node,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeRemoteValue {
	#[serde(rename = "type")]
	pub r#type: NodeRemoteValueType,
	#[serde(rename = "sharedId")]
	pub shared_id: Option<SharedId>,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	pub value: Option<Box<NodeProperties>>,
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
	pub node_type: u32,
	#[serde(rename = "childNodeCount")]
	pub child_node_count: u32,
	#[serde(rename = "attributes")]
	pub attributes: Option<HashMap<String, String>>,
	#[serde(rename = "children")]
	pub children: Option<Vec<NodeRemoteValue>>,
	#[serde(rename = "localName")]
	pub local_name: Option<String>,
	#[serde(rename = "mode")]
	pub mode: Option<NodePropertiesMode>,
	#[serde(rename = "namespaceURI")]
	pub namespace_u_r_i: Option<String>,
	#[serde(rename = "nodeValue")]
	pub node_value: Option<String>,
	#[serde(rename = "shadowRoot")]
	pub shadow_root: Option<Box<NodeRemoteValue>>,
}

#[derive(Debug, Serialize, Deserialize)]
enum WindowProxyRemoteValueType {
	#[serde(rename = "window")]
	Window,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowProxyRemoteValue {
	#[serde(rename = "type")]
	pub r#type: WindowProxyRemoteValueType,
	#[serde(rename = "value")]
	pub value: WindowProxyProperties,
	#[serde(rename = "handle")]
	pub handle: Option<Handle>,
	#[serde(rename = "internalId")]
	pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowProxyProperties {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
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
	#[serde(rename = "maxDomDepth", default = "serialization_options_default_max_dom_depth")]
	pub max_dom_depth: Option<u32>,
	
	#[serde(rename = "maxObjectDepth", default)]
	pub max_object_depth: Option<u32>,
	
	#[serde(rename = "includeShadowTree", default = "serialization_options_default_include_shadow_tree")]
	pub include_shadow_tree: IncludeShadowTree,
}

fn serialization_options_default_max_dom_depth() -> Option<u32> {
	Some(0)
}

fn serialization_options_default_include_shadow_tree() -> IncludeShadowTree {
	IncludeShadowTree::None
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StackFrame {
	#[serde(rename = "columnNumber")]
	pub column_number: u32,
	#[serde(rename = "functionName")]
	pub function_name: String,
	#[serde(rename = "linpub enumber")]
	pub line_number: u32,
	#[serde(rename = "url")]
	pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StackTrace {
	#[serde(rename = "callFrames")]
	pub call_frames: Vec<StackFrame>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
	#[serde(rename = "realm")]
	pub realm: Realm,
	#[serde(rename = "context")]
	pub context: Option<BrowsingContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmTarget {
	#[serde(rename = "realm")]
	pub realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextTarget {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "sandbox")]
	pub sandbox: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Target {
	ContextTarget(ContextTarget),
	RealmTarget(RealmTarget),
}