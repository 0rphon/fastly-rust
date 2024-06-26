/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipsForWafExclusion {
    #[serde(rename = "waf_rules", skip_serializing_if = "Option::is_none")]
    pub waf_rules: Option<Box<crate::models::RelationshipWafRuleWafRule>>,
    #[serde(rename = "waf_rule_revisions", skip_serializing_if = "Option::is_none")]
    pub waf_rule_revisions: Option<Box<crate::models::RelationshipWafRuleRevisionWafRuleRevisions>>,
}

impl RelationshipsForWafExclusion {
    pub fn new() -> RelationshipsForWafExclusion {
        RelationshipsForWafExclusion {
            waf_rules: None,
            waf_rule_revisions: None,
        }
    }
}


