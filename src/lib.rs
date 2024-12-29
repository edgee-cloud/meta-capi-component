mod meta_payload;

use std::collections::HashMap;

use crate::exports::edgee::protocols::provider::{
    Data, Dict, EdgeeRequest, Event, Guest, HttpMethod,
};
use meta_payload::{parse_value, MetaEvent, MetaPayload};

wit_bindgen::generate!({world: "data-collection", path: "wit", with: { "edgee:protocols/provider": generate }});

export!(MetaComponent);

struct MetaComponent;

impl Guest for MetaComponent {
    fn page(edgee_event: Event, cred_map: Dict) -> Result<EdgeeRequest, String> {
        if let Data::Page(ref data) = edgee_event.data {
            let mut meta_payload = MetaPayload::new(cred_map).map_err(|e| e.to_string())?;

            let mut event = MetaEvent::new(&edgee_event, "PageView").map_err(|e| e.to_string())?;

            // Create custom data
            let mut custom_data: HashMap<String, serde_json::Value> = HashMap::new();

            if !data.name.is_empty() {
                custom_data.insert("page_name".to_string(), parse_value(&data.name));
            }
            if !data.category.is_empty() {
                custom_data.insert("page_category".to_string(), parse_value(&data.category));
            }
            if !data.title.is_empty() {
                custom_data.insert("page_title".to_string(), parse_value(&data.title));
            }

            // Add custom properties from page data
            for (key, value) in data.properties.iter() {
                custom_data.insert(key.clone(), parse_value(value));
            }

            event.custom_data = Some(custom_data);
            meta_payload.data.push(event);

            Ok(build_edgee_request(meta_payload))
        } else {
            Err("Missing page data".to_string())
        }
    }

    fn track(edgee_event: Event, cred_map: Dict) -> Result<EdgeeRequest, String> {
        if let Data::Track(ref data) = edgee_event.data {
            if data.name.is_empty() {
                return Err("Track name is not set".to_string());
            }

            let mut meta_payload = MetaPayload::new(cred_map).map_err(|e| e.to_string())?;
            let mut event =
                MetaEvent::new(&edgee_event, data.name.as_str()).map_err(|e| e.to_string())?;

            // Create custom data from properties
            let mut custom_data: HashMap<String, serde_json::Value> = HashMap::new();
            for (key, value) in data.properties.iter() {
                custom_data.insert(key.clone(), parse_value(value));
            }
            event.custom_data = Some(custom_data);
            meta_payload.data.push(event);

            Ok(build_edgee_request(meta_payload))
        } else {
            Err("Missing track data".to_string())
        }
    }

    fn user(edgee_event: Event, cred_map: Dict) -> Result<EdgeeRequest, String> {
        if let Data::User(ref data) = edgee_event.data {
            if data.user_id.is_empty() && data.anonymous_id.is_empty() {
                return Err("user_id or anonymous_id is not set".to_string());
            }

            let mut meta_payload = MetaPayload::new(cred_map).map_err(|e| e.to_string())?;
            let event = MetaEvent::new(&edgee_event, "Lead").map_err(|e| e.to_string())?;
            meta_payload.data.push(event);

            Ok(build_edgee_request(meta_payload))
        } else {
            Err("Missing user data".to_string())
        }
    }
}

fn build_edgee_request(meta_payload: MetaPayload) -> EdgeeRequest {
    let headers = vec![(
        String::from("content-type"),
        String::from("application/json"),
    )];

    let url = format!(
        "https://graph.facebook.com/v17.0/{}/events?access_token={}",
        meta_payload.pixel_id, meta_payload.access_token
    );

    let url = if let Some(test_code) = meta_payload.test_event_code.clone() {
        format!("{}&test_event_code={}", url, test_code)
    } else {
        url
    };

    EdgeeRequest {
        method: HttpMethod::Post,
        url,
        headers,
        body: serde_json::to_string(&meta_payload).unwrap(),
    }
}
