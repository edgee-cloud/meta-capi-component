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

This component implements the data collection protocol between [Edgee](https://www.edgee.cloud) and [Meta CAPI](https://developers.facebook.com/docs/marketing-api/conversions-api/).

## Quick Start

1. Download the latest component version from our [releases page](../../releases)
2. Place the `meta_capi.wasm` file in your server (e.g., `/var/edgee/components`)
3. Add the following configuration to your `edgee.toml`:

```toml
[[destinations.data_collection]]
name = "meta_capi"
component = "/var/edgee/components/meta_capi.wasm"
credentials.meta_access_token = "YOUR_ACCESS_TOKEN"
credentials.meta_pixel_id = "YOUR_PIXEL_ID"
credentials.meta_test_event_code = "TEST_EVENT_CODE" # Optional
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

## Configuration Options

### Basic Configuration
```toml
[[destinations.data_collection]]
name = "meta_capi"
component = "/var/edgee/components/meta_capi.wasm"
credentials.meta_access_token = "YOUR_ACCESS_TOKEN"
credentials.meta_pixel_id = "YOUR_PIXEL_ID"
credentials.meta_test_event_code = "TEST_EVENT_CODE" # Optional

# Optional configurations
config.default_consent = "pending" # Set default consent status
```

### Event Controls
Control which events are forwarded to Meta CAPI:
```toml
config.page_event_enabled = true   # Enable/disable page view tracking
config.track_event_enabled = true  # Enable/disable custom event tracking
config.user_event_enabled = true   # Enable/disable user identification
```

### Consent Management
Before sending events to Meta CAPI, you can set the user consent using the Edgee SDK: 
```javascript
edgee.consent("granted");
```

Or using the Data Layer:
```html
<script id="__EDGEE_DATA_LAYER__" type="application/json">
  {
    "data_collection": {
      "consent": "granted"
    }
  }
</script>
```

If the consent is not set, the component will use the default consent status.
**Important:** Meta CAPI requires the consent status to be set to `granted`. If not, the events will be ignored.

| Consent | Events |
|---------|--------|
| pending | ignored |
| denied  | ignored |
| granted | forwarded |

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