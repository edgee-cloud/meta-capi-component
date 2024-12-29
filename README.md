<div align="center">

<p align="center">
  <a href="https://www.edgee.cloud">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.cloud/img/favicon-dark.svg">
      <img src="https://cdn.edgee.cloud/img/favicon.svg" height="100" alt="Edgee">
    </picture>
    <h1 align="center">Edgee</h1>
  </a>
</p>


**The full-stack edge platform for your edge oriented applications.**

[![Edgee](https://img.shields.io/badge/edgee-open%20source-blueviolet.svg)](https://www.edgee.cloud)
[![Edgee](https://img.shields.io/badge/slack-edgee-blueviolet.svg?logo=slack)](https://www.edgee.cloud/slack)
[![Docs](https://img.shields.io/badge/docs-published-blue)](https://docs.edgee.cloud)

</div>

This component implements the data collection protocol between [Meta CAPI](https://developers.facebook.com/docs/marketing-api/conversions-api/) and [Edgee](https://www.edgee.cloud).

### Event mapping:

Here is the mapping between Edgee events and GA events:

| Edgee event | GA Event  |
|-------------|-----------|
| Page   | `PageView`     |
| Track  | Name of the event |
| User   | `Lead` |

Each time you make a `user` call, Edgee will send an `Lead` event to Meta CAPI.

But when you make a `user` call using Edgee's JS library or Data Layer, the `user_id`, `anonymous_id` and `properties` are stored in the user's device.
This allows the user's data to be added to any subsequent page or follow-up calls for the user, so that you can correctly attribute these actions.

**BE CAREFUL:**
Meta Conversions API is designed to create a connection between an advertiser’s marketing data (such as website events) and Meta systems that optimize ad targeting, decrease cost per result and measure outcomes.
Each event you send to Meta CAPI must have a user property (at least one of the following: `email`, `phone_number`), otherwise the event will be ignored.

Here is an example of a user call:
```javascript
edgee.user({
  user_id: "123",
  properties: {
    email: "john.doe@example.com",
  },
});
```

## Usage

- Download the latest version in our [releases page](../../releases). 
- Place the wasm file in a known place in your server (e.g. `/var/edgee/components`).
- Update your edgee proxy config:
```toml
[[destinations.data_collection]]
name = "meta_capi"
component = "/var/edgee/components/meta_capi.wasm"
credentials.meta_access_token = "YOUR_ACCESS_TOKEN"
credentials.meta_pixel_id = "YOUR_PIXEL_ID"
credentials.meta_test_event_code = "TEST_EVENT_CODE" # Optional
```

To know how to get the access token and pixel id, please refer to the [Meta CAPI documentation](https://developers.facebook.com/docs/marketing-api/conversions-api/get-started).

## Contributing
If you're interested in contributing to Edgee, read our [contribution guidelines](./CONTRIBUTING.md)

## Reporting Security Vulnerabilities
If you've found a vulnerability or potential vulnerability in our code, please let us know at
[edgee-security](mailto:security@edgee.cloud).

## Building from source

To build the wasm file from source, you need to have installed 
- [Rust](https://www.rust-lang.org/tools/install)
- `wasm32-wasip2` target: run `rustup target add wasm32-wasip2`
- `wasm-tools`: run `cargo install --locked wasm-tools`

Then you can run the following commands:

```bash
make build
```