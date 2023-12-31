# v0.3.0 (Dec 2023)

## Breaking changes

- `VerifyKeyRequest` now requires an `api_id`.
- `ListKeysRequest` no longer has an `offset` field.

## Additions

- Add `Conflict` variant to `ErrorCode` enum.
- Add `get_key` method to `KeyService`.
- Add `cursor` field to `ListKeysRequest`.
- Add `Refill`, `RefillInterval`, and `UpdateOp` models/enums.
- Add `key_id` property onto `ApiKeyVerification`.
- Add `refill` property onto `ApiKeyMeta` and `ApiKeyVerification`.
- Add support for `refill` when creating and updating a key.
- Add `update_remaining` method to `KeyService` and corresponding `Route`.

## Changes

- Refactor internal routes to use new API endpoints.
- Use new headers requested by Unkey.

---

# v0.2.0 (Sep 2023)

## Additions

- Add `NotUnique` and `InvalidKeyType` variants to `ErrorCode` enum.

## Changes

- Rename `UsageExceeded` error code to `KeyUsageExceeded`.

# v0.1.0 (Aug 2023)

## Additions

- Add `Client` for interacting with the unkey api.
- Add `ApiService` for api related requests.
- Add `KeyService` for key related requests.
- Add `create_key` method.
- Add `verify_key` method.
- Add `revoke_key` method.
- Add `update_key` method.
- Add `list_keys` method.
- Add `get_api` method.
- Add various models supporting api service.
- Add various models supporting key service.
