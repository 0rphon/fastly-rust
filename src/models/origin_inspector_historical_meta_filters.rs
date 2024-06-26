/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// OriginInspectorHistoricalMetaFilters : Filters that were applied when calculating the results for this query.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OriginInspectorHistoricalMetaFilters {
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

impl OriginInspectorHistoricalMetaFilters {
    /// Filters that were applied when calculating the results for this query.
    pub fn new() -> OriginInspectorHistoricalMetaFilters {
        OriginInspectorHistoricalMetaFilters {
            region: None,
            datacenter: None,
            host: None,
        }
    }
}


