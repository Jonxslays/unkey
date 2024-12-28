# Unkey for Rust

An asynchronous Rust SDK for the [Unkey API](https://docs.unkey.dev/introduction).

All the API key management features you love, now with more type safety!

## MSRV

The minimum supported Rust version for the project is `1.63.0`.

## Documentation

Full documentation can be found at [https://docs.rs/unkey](https://docs.rs/unkey).

## Setup

### Using `cargo`

```bash
cargo add unkey
```

### Manually

Add the following to your `Cargo.toml` dependencies array:

```toml
unkey = "0.6" # I won't forget to update this™
```

## Examples

### Verifying a key

```rust
use unkey::models::VerifyKeyRequest;
use unkey::Client;

async fn verify_key() {
    let c = Client::new("unkey_ABC");
    let req = VerifyKeyRequest::new("test_DEF", "api_JJJ");

    match c.verify_key(req).await {
        Ok(res) => println!("{res:?}"),
        Err(err) => eprintln!("{err:?}"),
    }
}
```

### Creating a key

```rust
use unkey::models::CreateKeyRequest;
use unkey::Client;

async fn create_key() {
    let c = Client::new("unkey_ABC");
    let req = CreateKeyRequest::new("api_123")
        .set_prefix("test")
        .set_remaining(100)
        .set_name("test_name")
        .set_owner_id("jonxslays");

    match c.create_key(req).await {
        Ok(res) => println!("{res:?}"),
        Err(err) => eprintln!("{err:?}"),
    }
}
```

### Updating a key

```rust
use unkey::models::{Refill, RefillInterval, UpdateKeyRequest};
use unkey::Client;

async fn update_key() {
    let c = Client::new("unkey_ABC");
    let req = UpdateKeyRequest::new("key_XYZ")
        .set_name(Some("new_name")) // Update the keys name
        .set_ratelimit(None) // Remove any ratelimit on the key
        .set_expires(None) // Remove any expiration date
        .set_refill(Some(Refill::new(100, RefillInterval::Daily)));

    match c.update_key(req).await {
        Ok(res) => println!("{res:?}"),
        Err(err) => eprintln!("{err:?}"),
    }
}
```

### Revoking a key

```rust
use unkey::models::RevokeKeyRequest;
use unkey::Client;

async fn revoke_key() {
    let c = Client::new("unkey_ABC");
    let req = RevokeKeyRequest::new("key_XYZ");

    match c.revoke_key(req).await {
        Ok(res) => println!("{res:?}"),
        Err(err) => eprintln!("{err:?}"),
    }
}
```

### Listing api keys

```rust
use unkey::models::ListKeysRequest;
use unkey::Client;

async fn list_keys() {
    let c = Client::new("unkey_ABC");
    let req = ListKeysRequest::new("api_123");

    match c.list_keys(req).await {
        Ok(res) => println!("{res:?}"),
        Err(err) => eprintln!("{err:?}"),
    }
}
```

### Getting api information

```rust
use unkey::models::GetApiRequest;
use unkey::Client;

async fn get_api() {
    let c = Client::new("unkey_ABC");
    let req = GetApiRequest::new("api_123");

    match c.get_api(req).await {
        Ok(res) => println!("{res:?}"),
        Err(err) => eprintln!("{err:?}"),
    }
}
```

### Deleting an api

```rust
use unkey::models::DeleteApiRequest;
use unkey::Client;

async fn delete_api() {
    let c = Client::new("unkey_ABC");
    let req = DeleteApiRequest::new("api_XYZ");

    match c.delete_api(req).await {
        Ok(res) => println!("{res:?}"),
        Err(err) => eprintln!("{err:?}"),
    }
}
```

### Getting key details

```rust
use unkey::models::GetKeyRequest;
use unkey::Client;

async fn get_key() {
    let c = Client::new("unkey_ABC");
    let req = GetKeyRequest::new("key_123");

    match c.get_key(req).await {
        Ok(res) => println!("{res:?}"),
        Err(err) => eprintln!("{err:?}"),
    }
}
```

### Update remaining verifications

```rust
use unkey::models::{UpdateOp, UpdateRemainingRequest};
use unkey::Client;

async fn update_remaining() {
    let c = Client::new("unkey_ABC");
    let req = UpdateRemainingRequest::new("key_123", Some(100), UpdateOp::Set);

    match c.update_remaining(req).await {
        Ok(res) => println!("{res:?}"),
        Err(err) => eprintln!("{err:?}"),
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
