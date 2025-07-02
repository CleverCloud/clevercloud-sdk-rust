use serde::{Deserialize, Serialize};

mod http_error_context {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Empty;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FieldError {
        #[serde(rename = "value")]
        value: String,
        #[serde(rename = "reason")]
        reason: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Field {
        #[serde(rename = "name")]
        name: String,
        #[serde(rename = "error")]
        error: FieldError,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GatewayError {
        #[serde(rename = "originalStatus")]
        original_status: i32,
        #[serde(rename = "originalBody")]
        original_body: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Input {
        #[serde(rename = "names")]
        names: Vec<String>,
    }

    type MapFieldError = HashMap<String, FieldError>;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct MultipleFields {
        #[serde(rename = "fields")]
        fields: MapFieldError,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Resource {
        #[serde(rename = "kind")]
        kind: String,
        #[serde(rename = "name")]
        name: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Operation {
        #[serde(rename = "operation")]
        operation: String,
        #[serde(rename = "kind")]
        kind: String,
        #[serde(rename = "name")]
        name: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Selector {
        #[serde(rename = "path")]
        path: Vec<String>,
    }
}

#[derive(Debug, Serialize)]
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

impl<'de> Deserialize<'de> for HttpErrorContext {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(tag = "type")]
        pub enum TaggedHttpErrorContext {
            Empty(http_error_context::Empty),
            Field(http_error_context::Field),
            Gateway(http_error_context::GatewayError),
            Input(http_error_context::Input),
            MultipleFields(http_error_context::MultipleFields),
            Operation(http_error_context::Operation),
            Resource(http_error_context::Resource),
            Selector(http_error_context::Selector),
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum UntaggedHttpErrorContext {
            Empty(http_error_context::Empty),
            Field(http_error_context::Field),
            Gateway(http_error_context::GatewayError),
            Input(http_error_context::Input),
            MultipleFields(http_error_context::MultipleFields),
            Operation(http_error_context::Operation),
            Resource(http_error_context::Resource),
            Selector(http_error_context::Selector),
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        pub enum MaybeTagged {
            Untagged(UntaggedHttpErrorContext),
            Tag(TaggedHttpErrorContext),
        }

        Ok(match MaybeTagged::deserialize(deserializer)? {
            MaybeTagged::Untagged(UntaggedHttpErrorContext::Empty(x))
            | MaybeTagged::Tag(TaggedHttpErrorContext::Empty(x)) => Self::Empty(x),
            MaybeTagged::Untagged(UntaggedHttpErrorContext::Field(x))
            | MaybeTagged::Tag(TaggedHttpErrorContext::Field(x)) => Self::Field(x),
            MaybeTagged::Untagged(UntaggedHttpErrorContext::Gateway(x))
            | MaybeTagged::Tag(TaggedHttpErrorContext::Gateway(x)) => Self::Gateway(x),
            MaybeTagged::Untagged(UntaggedHttpErrorContext::Input(x))
            | MaybeTagged::Tag(TaggedHttpErrorContext::Input(x)) => Self::Input(x),
            MaybeTagged::Untagged(UntaggedHttpErrorContext::MultipleFields(x))
            | MaybeTagged::Tag(TaggedHttpErrorContext::MultipleFields(x)) => {
                Self::MultipleFields(x)
            }
            MaybeTagged::Untagged(UntaggedHttpErrorContext::Operation(x))
            | MaybeTagged::Tag(TaggedHttpErrorContext::Operation(x)) => Self::Operation(x),
            MaybeTagged::Untagged(UntaggedHttpErrorContext::Resource(x))
            | MaybeTagged::Tag(TaggedHttpErrorContext::Resource(x)) => Self::Resource(x),
            MaybeTagged::Untagged(UntaggedHttpErrorContext::Selector(x))
            | MaybeTagged::Tag(TaggedHttpErrorContext::Selector(x)) => Self::Selector(x),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpError {
    #[serde(rename = "apiRequestId")]
    pub api_request_id: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "context", flatten)]
    pub context: HttpErrorContext,
    #[serde(rename = "error")]
    pub error: String,
}
