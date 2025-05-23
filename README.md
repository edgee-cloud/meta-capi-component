<div align="center">
<p align="center">
  <a href="https://www.edgee.cloud">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.cloud/img/component-dark.svg">
      <img src="https://cdn.edgee.cloud/img/component.svg" height="100" alt="Edgee">
    </picture>
  </a>
</p>
</div>


<h1 align="center">Meta CAPI Component for Edgee</h1>

[![Coverage Status](https://coveralls.io/repos/github/edgee-cloud/meta-capi-component/badge.svg)](https://coveralls.io/github/edgee-cloud/meta-capi-component)
[![GitHub issues](https://img.shields.io/github/issues/edgee-cloud/meta-capi-component.svg)](https://github.com/edgee-cloud/meta-capi-component/issues)
[![Edgee Component Registry](https://img.shields.io/badge/Edgee_Component_Registry-Public-green.svg)](https://www.edgee.cloud/edgee/meta-capi)

This component implements the data collection protocol between [Edgee](https://www.edgee.cloud) and [Meta CAPI](https://developers.facebook.com/docs/marketing-api/conversions-api/).

## Quick Start

1. Download the latest component version from our [releases page](../../releases)
2. Place the `meta_capi.wasm` file in your server (e.g., `/var/edgee/components`)
3. Add the following configuration to your `edgee.toml`:

```toml
[[components.data_collection]]
id = "meta_capi"
file = "/var/edgee/components/meta_capi.wasm"
settings.meta_access_token = "YOUR_ACCESS_TOKEN"
settings.meta_pixel_id = "YOUR_PIXEL_ID"
settings.meta_test_event_code = "TEST_EVENT_CODE" # Optional
```

## Event Handling

### Event Mapping
The component maps Edgee events to Meta CAPI events as follows:

| Edgee event | Meta CAPI Event  | Description |
|-------------|-----------|-------------|
| Page   | `PageView`     | Triggered when a user views a page |
| Track  | Name of the event | Uses the provided event name directly |
| User   | `Lead` | Used for lead identification |

### User Event Handling
User events in Meta CAPI serve multiple purposes:
- Triggers an `Lead` call to Meta CAPI
- Stores `user_id`, `anonymous_id`, and `properties` on the user's device
- Enriches subsequent Page and Track events with user data
- Enables proper user attribution across sessions

## Configuration Options

### Basic Configuration
```toml
[[components.data_collection]]
id = "meta_capi"
file = "/var/edgee/components/meta_capi.wasm"
settings.meta_access_token = "YOUR_ACCESS_TOKEN"
settings.meta_pixel_id = "YOUR_PIXEL_ID"
settings.meta_test_event_code = "TEST_EVENT_CODE" # Optional

# Optional configurations
settings.edgee_default_consent = "pending" # Set default consent status
```

### Event Controls
Control which events are forwarded to Meta CAPI:
```toml
settings.edgee_page_event_enabled = true   # Enable/disable page view tracking
settings.edgee_track_event_enabled = true  # Enable/disable custom event tracking
settings.edgee_user_event_enabled = true   # Enable/disable user identification
```

## Development

### Building from Source
Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)
- WASM target: `rustup target add wasm32-wasip2`
- wit-deps: `cargo install wit-deps`

Build command:
```bash
make wit-deps
make build
```

### Contributing
Interested in contributing? Read our [contribution guidelines](./CONTRIBUTING.md)

### Security
Report security vulnerabilities to [security@edgee.cloud](mailto:security@edgee.cloud)
```