#[allow(warnings)]
mod bindings;

use bindings::exports::component::loro_wit::doc::{
    Guest, GuestLoroDoc, GuestLoroList, GuestLoroMap, GuestLoroText, LoroDoc as WitLoroDoc,
    LoroList as WitLoroList, LoroMap as WitLoroMap, LoroText as WitLoroText,
};
use bindings::exports::component::loro_wit::types::{
    Frontiers, LoroError, OpId, PeerId, VersionVector, VersionVectorEntry,
};

use loro::{ExportMode, LoroDoc as InnerLoroDoc, LoroList as InnerLoroList, LoroMap as InnerLoroMap, LoroText as InnerLoroText, LoroValue, ToJson};

struct Component;

// Wrapper for LoroDoc
pub struct LoroDocWrapper(InnerLoroDoc);

// Wrapper for LoroText
pub struct LoroTextWrapper(InnerLoroText);

// Wrapper for LoroMap
pub struct LoroMapWrapper(InnerLoroMap);

// Wrapper for LoroList
pub struct LoroListWrapper(InnerLoroList);

fn convert_error(e: loro::LoroError) -> LoroError {
    LoroError::ContainerError(e.to_string())
}

impl GuestLoroDoc for LoroDocWrapper {
    fn new() -> Self {
        LoroDocWrapper(InnerLoroDoc::new())
    }

    fn peer_id(&self) -> PeerId {
        self.0.peer_id().to_string()
    }

    fn set_peer_id(&self, peer: u64) -> Result<(), LoroError> {
        self.0.set_peer_id(peer).map_err(convert_error)
    }

    fn get_text(&self, name: String) -> WitLoroText {
        let text = self.0.get_text(&*name);
        WitLoroText::new(LoroTextWrapper(text))
    }

    fn get_map(&self, name: String) -> WitLoroMap {
        let map = self.0.get_map(&*name);
        WitLoroMap::new(LoroMapWrapper(map))
    }

    fn get_list(&self, name: String) -> WitLoroList {
        let list = self.0.get_list(&*name);
        WitLoroList::new(LoroListWrapper(list))
    }

    fn commit(&self) {
        self.0.commit();
    }

    fn export_snapshot(&self) -> Result<Vec<u8>, LoroError> {
        self.0
            .export(ExportMode::Snapshot)
            .map_err(|e| LoroError::EncodingError(e.to_string()))
    }

    fn export_updates(&self) -> Result<Vec<u8>, LoroError> {
        self.0
            .export(ExportMode::all_updates())
            .map_err(|e| LoroError::EncodingError(e.to_string()))
    }

    fn import_bytes(&self, data: Vec<u8>) -> Result<(), LoroError> {
        self.0.import(&data).map(|_| ()).map_err(convert_error)
    }

    fn oplog_version(&self) -> VersionVector {
        self.0
            .oplog_vv()
            .iter()
            .map(|(peer, counter)| VersionVectorEntry {
                peer: peer.to_string(),
                counter: *counter,
            })
            .collect()
    }

    fn state_frontiers(&self) -> Frontiers {
        self.0
            .state_frontiers()
            .iter()
            .map(|id| OpId {
                peer: id.peer.to_string(),
                counter: id.counter,
            })
            .collect()
    }

    fn get_deep_value_json(&self) -> String {
        self.0.get_deep_value().to_json()
    }

    fn fork(&self) -> WitLoroDoc {
        let forked = self.0.fork();
        WitLoroDoc::new(LoroDocWrapper(forked))
    }

    fn is_detached(&self) -> bool {
        self.0.is_detached()
    }

    fn attach(&self) {
        self.0.attach();
    }

    fn detach(&self) {
        self.0.detach();
    }
}

impl GuestLoroText for LoroTextWrapper {
    fn insert(&self, pos: u32, text: String) -> Result<(), LoroError> {
        self.0
            .insert(pos as usize, &text)
            .map_err(convert_error)
    }

    fn delete(&self, pos: u32, len: u32) -> Result<(), LoroError> {
        self.0
            .delete(pos as usize, len as usize)
            .map_err(convert_error)
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn len_unicode(&self) -> u32 {
        self.0.len_unicode() as u32
    }

    fn len_utf8(&self) -> u32 {
        self.0.len_utf8() as u32
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl GuestLoroMap for LoroMapWrapper {
    fn insert_string(&self, key: String, value: String) -> Result<(), LoroError> {
        self.0.insert(&key, value).map_err(convert_error)
    }

    fn insert_i64(&self, key: String, value: i64) -> Result<(), LoroError> {
        self.0.insert(&key, value).map_err(convert_error)
    }

    fn insert_f64(&self, key: String, value: f64) -> Result<(), LoroError> {
        self.0.insert(&key, value).map_err(convert_error)
    }

    fn insert_bool(&self, key: String, value: bool) -> Result<(), LoroError> {
        self.0.insert(&key, value).map_err(convert_error)
    }

    fn insert_null(&self, key: String) -> Result<(), LoroError> {
        self.0.insert(&key, LoroValue::Null).map_err(convert_error)
    }

    fn delete(&self, key: String) -> Result<(), LoroError> {
        self.0.delete(&key).map_err(convert_error)
    }

    fn get_json(&self, key: String) -> Option<String> {
        self.0.get(&key).map(|v| v.get_deep_value().to_json())
    }

    fn keys(&self) -> Vec<String> {
        self.0.keys().into_iter().map(|s| s.to_string()).collect()
    }

    fn len(&self) -> u32 {
        self.0.len() as u32
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn get_deep_value_json(&self) -> String {
        self.0.get_deep_value().to_json()
    }
}

impl GuestLoroList for LoroListWrapper {
    fn insert_string(&self, pos: u32, value: String) -> Result<(), LoroError> {
        self.0.insert(pos as usize, value).map_err(convert_error)
    }

    fn insert_i64(&self, pos: u32, value: i64) -> Result<(), LoroError> {
        self.0.insert(pos as usize, value).map_err(convert_error)
    }

    fn insert_f64(&self, pos: u32, value: f64) -> Result<(), LoroError> {
        self.0.insert(pos as usize, value).map_err(convert_error)
    }

    fn insert_bool(&self, pos: u32, value: bool) -> Result<(), LoroError> {
        self.0.insert(pos as usize, value).map_err(convert_error)
    }

    fn insert_null(&self, pos: u32) -> Result<(), LoroError> {
        self.0
            .insert(pos as usize, LoroValue::Null)
            .map_err(convert_error)
    }

    fn delete(&self, pos: u32, len: u32) -> Result<(), LoroError> {
        self.0
            .delete(pos as usize, len as usize)
            .map_err(convert_error)
    }

    fn get_json(&self, index: u32) -> Option<String> {
        self.0.get(index as usize).map(|v| v.get_deep_value().to_json())
    }

    fn push_string(&self, value: String) -> Result<(), LoroError> {
        self.0.push(value).map_err(convert_error)
    }

    fn push_i64(&self, value: i64) -> Result<(), LoroError> {
        self.0.push(value).map_err(convert_error)
    }

    fn len(&self) -> u32 {
        self.0.len() as u32
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn get_deep_value_json(&self) -> String {
        self.0.get_deep_value().to_json()
    }
}

impl Guest for Component {
    type LoroDoc = LoroDocWrapper;
    type LoroText = LoroTextWrapper;
    type LoroMap = LoroMapWrapper;
    type LoroList = LoroListWrapper;
}

bindings::export!(Component with_types_in bindings);
