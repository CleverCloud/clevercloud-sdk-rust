use oauth10a::client::reqwest::StatusCode;
use serde::{Deserialize, Serialize};

pub type HttpOutput = (StatusCode, HttpError);

mod http_error_context {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Empty;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct FieldError {
        #[serde(rename = "value")]
        value: String,
        #[serde(rename = "reason")]
        reason: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Field {
        #[serde(rename = "name")]
        name: String,
        #[serde(rename = "error")]
        error: FieldError,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct GatewayError {
        #[serde(rename = "originalStatus")]
        original_status: i32,
        #[serde(rename = "originalBody")]
        original_body: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Input {
        #[serde(rename = "names")]
        names: Vec<String>,
    }

    type MapFieldError = HashMap<String, FieldError>;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct MultipleFields {
        #[serde(rename = "fields")]
        fields: MapFieldError,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Resource {
        #[serde(rename = "kind")]
        kind: String,
        #[serde(rename = "name")]
        name: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Operation {
        #[serde(rename = "operation")]
        operation: String,
        #[serde(rename = "kind")]
        kind: String,
        #[serde(rename = "name")]
        name: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Selector {
        #[serde(rename = "path")]
        path: Vec<String>,
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum HttpErrorContext {
    Empty(http_error_context::Empty),
    Field(http_error_context::Field),
    Gateway(http_error_context::GatewayError),
    Input(http_error_context::Input),
    MultipleFields(http_error_context::MultipleFields),
    Operation(http_error_context::Operation),
    Resource(http_error_context::Resource),
    Selector(http_error_context::Selector),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpError {
    #[serde(rename = "apiRequestId")]
    pub api_request_id: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "context")]
    pub context: HttpErrorContext,
    #[serde(rename = "error")]
    pub error: String,
}

// TODO: unit tests
// maybe use https://github.com/Orange-OpenSource/hurl or something
