/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipWafActiveRules {
    #[serde(rename = "waf_active_rules", skip_serializing_if = "Option::is_none")]
    pub waf_active_rules: Option<Box<crate::models::RelationshipWafActiveRulesWafActiveRules>>,
}

impl RelationshipWafActiveRules {
    pub fn new() -> RelationshipWafActiveRules {
        RelationshipWafActiveRules {
            waf_active_rules: None,
        }
    }
}


