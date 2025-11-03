use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct NodeType {
    pub children: Option<Children>,
    pub extra: Option<bool>,
    pub fields: Option<HashMap<String, Field>>,
    pub named: bool,
    pub root: Option<bool>,
    pub subtypes: Option<Vec<Subtype>>,
    #[allow(clippy::struct_field_names)]
    #[serde(rename = "type")]
    pub node_type_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Children {
    pub multiple: bool,
    pub required: bool,
    pub types: Vec<ChildrenType>,
}

#[derive(Serialize, Deserialize)]
pub struct ChildrenType {
    pub named: bool,
    #[serde(rename = "type")]
    pub child_type_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    pub multiple: bool,
    pub required: bool,
    pub types: Vec<FieldType>,
}

#[derive(Serialize, Deserialize)]
pub struct FieldType {
    pub named: bool,
    #[serde(rename = "type")]
    pub field_type_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Subtype {
    pub named: bool,
    #[serde(rename = "type")]
    pub subchild_type_name: String,
}
