/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AclEntryResponse {
    /// Whether to negate the match. Useful primarily when creating individual exceptions to larger subnets.
    #[serde(rename = "negated", skip_serializing_if = "Option::is_none")]
    pub negated: Option<Negated>,
    /// A freeform descriptive note.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// An IP address.
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Number of bits for the subnet mask applied to the IP address. For IPv4 addresses, a value of 32 represents the smallest subnet mask (1 address), 24 represents a class C subnet mask (256 addresses), 16 represents a class B subnet mask (65k addresses), and 8 is class A subnet mask (16m addresses). If not provided, no mask is applied.
    #[serde(rename = "subnet", skip_serializing_if = "Option::is_none")]
    pub subnet: Option<i32>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "acl_id", skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<Box<String>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Box<String>>,
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Box<String>>,
}

impl AclEntryResponse {
    pub fn new() -> AclEntryResponse {
        AclEntryResponse {
            negated: None,
            comment: None,
            ip: None,
            subnet: None,
            created_at: None,
            deleted_at: None,
            updated_at: None,
            acl_id: None,
            id: None,
            service_id: None,
        }
    }
}

/// Whether to negate the match. Useful primarily when creating individual exceptions to larger subnets.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Negated {
    #[serde(rename = "0")]
    NegatedDisable,
    #[serde(rename = "1")]
    NegatedEnable,
}

impl Default for Negated {
    fn default() -> Negated {
        Self::NegatedDisable
    }
}

