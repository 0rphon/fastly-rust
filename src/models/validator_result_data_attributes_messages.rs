/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValidatorResultDataAttributesMessages {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "warning")]
    pub warning: bool,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "tokens")]
    pub tokens: Vec<::std::collections::HashMap<String, crate::models::TokensAdditionalProps>>,
}

impl ValidatorResultDataAttributesMessages {
    pub fn new(_type: String, warning: bool, message: String, tokens: Vec<::std::collections::HashMap<String, crate::models::TokensAdditionalProps>>) -> ValidatorResultDataAttributesMessages {
        ValidatorResultDataAttributesMessages {
            _type,
            warning,
            message,
            tokens,
        }
    }
}


