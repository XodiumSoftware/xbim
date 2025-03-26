/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use std::collections::HashMap;

use serde_json::Value;

pub enum BimObjectType {
    Building,
    Story,
    Space,
    Element,
    Component,
}

pub struct BimObject {
    pub id: String,
    pub object_type: BimObjectType,
    pub name: String,
    pub properties: HashMap<String, Value>,
    pub geometry: Option<String>,
    pub parent_id: Option<String>,
    pub relations: Vec<BimRelation>,
}

pub struct BimRelation {
    pub relation_type: String,
    pub target_id: String,
}
