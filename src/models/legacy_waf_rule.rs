/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LegacyWafRule {
    /// Message metadata for the rule.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Corresponding ModSecurity rule ID.
    #[serde(rename = "rule_id", skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// Severity metadata for the rule.
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
    /// The ModSecurity rule logic.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The VCL representation of the rule logic.
    #[serde(rename = "vcl", skip_serializing_if = "Option::is_none")]
    pub vcl: Option<String>,
}

impl LegacyWafRule {
    pub fn new() -> LegacyWafRule {
        LegacyWafRule {
            message: None,
            rule_id: None,
            severity: None,
            source: None,
            vcl: None,
        }
    }
}


