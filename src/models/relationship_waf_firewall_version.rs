/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipWafFirewallVersion {
    #[serde(rename = "waf_firewall_version", skip_serializing_if = "Option::is_none")]
    pub waf_firewall_version: Option<Box<crate::models::RelationshipWafFirewallVersionWafFirewallVersion>>,
}

impl RelationshipWafFirewallVersion {
    pub fn new() -> RelationshipWafFirewallVersion {
        RelationshipWafFirewallVersion {
            waf_firewall_version: None,
        }
    }
}


