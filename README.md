# Unkey for Rust

An asynchronous Rust SDK for the [Unkey API](https://docs.unkey.dev/introduction).

All the API key management features you love, now with more type safety!

## MSRV

The minimum supported Rust verision for the project is `1.63.0`.

## Documentation

Full documentation can be found at [https://docs.rs/unkey](https://docs.rs/unkey).

## Setup

Add the following to your `Cargo.toml` dependencies array:

```toml
unkey = "0.1"
```

## Examples

### Verifying a key

```rust
use unkey::models::{VerifyKeyRequest, Wrapped};
use unkey::Client;

async fn verify_key() {
    let c = Client::new("unkey_ABC");
    let req = VerifyKeyRequest::new("test_DEF");

    match c.verify_key(req).await {
        Wrapped::Ok(res) => println!("{res:?}"),
        Wrapped::Err(err) => eprintln!("{err:?}"),
    }
}
```

### Creating a key

```rust
use unkey::models::{CreateKeyRequest, Wrapped};
use unkey::Client;

async fn create_key() {
    let c = Client::new("unkey_ABC");
    let req = CreateKeyRequest::new("api_123")
        .set_prefix("test")
        .set_remaining(100)
        .set_name("test_name")
        .set_owner_id("jonxslays");

    match c.create_key(req).await {
        Wrapped::Ok(res) => println!("{res:?}"),
        Wrapped::Err(err) => eprintln!("{err:?}"),
    }
}
```

### Updating a key

```rust
use unkey::models::{UpdateKeyRequest, Wrapped};
use unkey::Client;

async fn update_key() {
    let c = Client::new("unkey_ABC");
    let req = UpdateKeyRequest::new("key_XYZ")
        .set_name(Some("new_name")) // Update the keys name
        .set_ratelimit(None); // Remove any ratelimit on the key

    match c.update_key(req).await {
        Wrapped::Ok(res) => println!("{res:?}"),
        Wrapped::Err(err) => eprintln!("{err:?}"),
    }
}
```

### Revoking a key

```rust
use unkey::models::{RevokeKeyRequest, Wrapped};
use unkey::Client;

async fn revoke_key() {
    let c = Client::new("unkey_ABC");
    let req = RevokeKeyRequest::new("key_XYZ");

    match c.revoke_key(req).await {
        Wrapped::Ok(res) => println!("{res:?}"),
        Wrapped::Err(err) => eprintln!("{err:?}"),
    }
}
```

### Listing api keys

```rust
use unkey::models::{ListKeysRequest, Wrapped};
use unkey::Client;

async fn list_keys() {
    let c = Client::new("unkey_ABC");
    let req = ListKeysRequest::new("api_123");

    match c.list_keys(req).await {
        Wrapped::Ok(res) => println!("{res:?}"),
        Wrapped::Err(err) => eprintln!("{err:?}"),
    }
}
```

### Getting api information

```rust
use unkey::models::{GetApiRequest, Wrapped};
use unkey::Client;

async fn get_api() {
    let c = Client::new("unkey_ABC");
    let req = GetApiRequest::new("api_123");

    match c.get_api(req).await {
        Wrapped::Ok(res) => println!("{res:?}"),
        Wrapped::Err(err) => eprintln!("{err:?}"),
    }
}
```

## Contributions

Unkey for Rust is open to contributions! Check out the
[contributing guide](https://github.com/Jonxslays/unkey/blob/master/CONTRIBUTING.md)
to get started.

## License

Unkey for Rust is licensed under the
[MIT License](https://github.com/Jonxslays/unkey/blob/master/LICENSE).
