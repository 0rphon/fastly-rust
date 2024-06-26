/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BulkUpdateDictionaryItem {
    /// Item key, maximum 256 characters.
    #[serde(rename = "item_key", skip_serializing_if = "Option::is_none")]
    pub item_key: Option<String>,
    /// Item value, maximum 8000 characters.
    #[serde(rename = "item_value", skip_serializing_if = "Option::is_none")]
    pub item_value: Option<String>,
    #[serde(rename = "op", skip_serializing_if = "Option::is_none")]
    pub op: Option<Op>,
}

impl BulkUpdateDictionaryItem {
    pub fn new() -> BulkUpdateDictionaryItem {
        BulkUpdateDictionaryItem {
            item_key: None,
            item_value: None,
            op: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Op {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "upsert")]
    Upsert,
}

impl Default for Op {
    fn default() -> Op {
        Self::Create
    }
}

