# Clever-Cloud Software Development Kit - Rust edition

[![crates.io](https://img.shields.io/crates/v/clevercloud-sdk.svg)](https://crates.io/crates/clevercloud-sdk)
[![Released API docs](https://docs.rs/clevercloud-sdk/badge.svg)](https://docs.rs/clevercloud-sdk)
[![Continuous integration](https://github.com/CleverCloud/clevercloud-sdk-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/CleverCloud/clevercloud-sdk-rust/actions/workflows/ci.yml)

> This crate provides structures and a client to interact with the Clever-Cloud API.

## Status

This crate is under development, you can use it, but it may have bugs or unimplemented features.

## Installation

To install this dependency, just add the following line to your `Cargo.toml` manifest.

```toml
clevercloud-sdk = { version = "^0.11.1", features = ["metrics", "jsonschemas"] }
```

## Usage

Below, you will find an example of executing a request to get information about
myself.

```rust
use clevercloud_sdk::{
    Client,
    oauth10a::credentials::Credentials,
    v2::myself::{self, Myself},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error + Send + Sync>> {
    let client = Client::from(Credentials::OAuth1 {
        token: "",
        secret: "",
        consumer_key: "",
        consumer_secret: "",
    });

    let _myself: Myself = myself::get(&client).await?;

    Ok(())
}
```

You could found more examples of how you could use the clevercloud-sdk by looking at the [command line](examples/cli/README.md) example.

## Features

| name          | description                                                                                      |
| ------------- |--------------------------------------------------------------------------------------------------|
| trace         | Use `tracing` crate to expose traces                                                             |
| jsonschemas   | Use `schemars` to add a derive instruction to generate json schemas representation of structures |
| logging       | Use the `log` facility crate to print logs. Implies `oauth10a/logging` feature                   |
| metrics       | Expose HTTP metrics through `oauth10a` crate feature.                                            |
| network-group | Enables Clever-Cloud Network Group API.                                                          |

### Metrics

Below, the exposed metrics gathered by prometheus:

| name                             | labels                                                          | kind    | description                        |
| -------------------------------- | --------------------------------------------------------------- | ------- | ---------------------------------- |
| oauth10a_client_request          | endpoint: String, method: String, status: Integer               | Counter | number of request on API           |
| oauth10a_client_request_duration | endpoint: String, method: String, status: Integer, unit: String | Counter | duration of request on API         |
| oauth10a_client_sse              | endpoint: String                                                | Counter | number of events received from API |

## License

See the [license](LICENSE).

## Getting in touch

- [@FlorentinDUBOIS](https://twitter.com/FlorentinDUBOIS)
