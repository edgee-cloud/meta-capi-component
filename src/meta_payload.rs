use anyhow::anyhow;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use crate::exports::edgee::protocols::data_collection::{Consent, Data, Dict, Event};

#[derive(Serialize, Debug, Default)]
pub(crate) struct MetaPayload {
    pub data: Vec<MetaEvent>,
    #[serde(skip)]
    pub access_token: String,
    #[serde(skip)]
    pub pixel_id: String,
    #[serde(skip)]
    pub test_event_code: Option<String>,
}

impl MetaPayload {
    pub fn new(cred_map: Dict) -> anyhow::Result<Self> {
        let cred: HashMap<String, String> = cred_map
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        let access_token = match cred.get("meta_access_token") {
            Some(key) => key,
            None => return Err(anyhow!("Missing Meta Access Token")),
        }
        .to_string();

        let pixel_id = match cred.get("meta_pixel_id") {
            Some(key) => key,
            None => return Err(anyhow!("Missing Meta Pixel ID")),
        }
        .to_string();

        let test_event_code = cred.get("meta_test_event_code").map(|s| s.to_string());

        Ok(Self {
            data: vec![],
            access_token,
            pixel_id,
            test_event_code,
        })
    }
}

/// Meta Event
///
/// This is the event that will be sent to Meta CAPI.
/// To know more about the event structure, check the online documentation: https://developers.facebook.com/docs/marketing-api/conversions-api/parameters/server-event
///
/// There are three ways of tracking conversions using this component:
/// - Standard events, which are user actions that we've defined and that you record by calling a `track`event. To know more about the standard event list, please visit this documentation https://developers.facebook.com/docs/meta-pixel/reference#standard-events
/// - Personalized events, which are user actions defined by you and recorded by calling by calling a `track`event with a custom event name.
/// - Personalized conversions, which are visitor actions that are automatically tracked by analyzing your website's referring URLs.
#[derive(Serialize, Debug)]
pub struct MetaEvent {
    pub event_name: String,
    pub event_time: i64,
    pub user_data: UserData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_url: Option<String>,
    pub event_id: String,
    pub action_source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer_url: Option<String>,
}

// User Data
//
// This is the user data that will be sent to Meta CAPI.
// To know more about the user data structure, check the online documentation: https://developers.facebook.com/docs/marketing-api/conversions-api/parameters/customer-information-parameters
#[derive(Serialize, Debug, Default)]
pub struct UserData {
    #[serde(rename = "em", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>, // hashed email SHA256
    #[serde(rename = "ph", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>, // hashed phone number SHA256
    #[serde(rename = "fn", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>, // hashed
    #[serde(rename = "ln", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>, // hashed
    #[serde(rename = "db", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>, // hashed
    #[serde(rename = "ge", skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>, // hashed
    #[serde(rename = "ct", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>, // hashed
    #[serde(rename = "st", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>, // hashed
    #[serde(rename = "zp", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>, // hashed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>, // hashed

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbp: Option<String>,
}

impl MetaEvent {
    pub fn new(edgee_event: &Event, event_name: &str) -> anyhow::Result<Self> {
        // Default meta event
        let mut meta_event = MetaEvent {
            event_name: event_name.to_string(),
            event_time: edgee_event.timestamp,
            event_id: edgee_event.uuid.clone(),
            event_source_url: None,
            user_data: UserData::default(),
            custom_data: Some(HashMap::new()),
            action_source: "website".to_string(),
            referrer_url: None,
        };

        // Set event source URL
        if !edgee_event.context.page.url.is_empty() {
            let document_location = format!(
                "{}{}",
                edgee_event.context.page.url.clone(),
                edgee_event.context.page.search.clone()
            );
            meta_event.event_source_url = Some(document_location);
        }

        // Set referrer URL
        if !edgee_event.context.page.referrer.is_empty() {
            meta_event.referrer_url = Some(edgee_event.context.page.referrer.clone());
        }

        // Set user data
        let mut user_data = UserData {
            client_ip_address: Some(edgee_event.context.client.ip.clone()),
            client_user_agent: Some(edgee_event.context.client.user_agent.clone()),
            ..UserData::default()
        };

        // Set user IDs
        if !edgee_event.context.user.user_id.is_empty() {
            user_data.external_id = Some(hash_value(&edgee_event.context.user.user_id));
        }

        let mut user_properties = edgee_event.context.user.properties.clone();
        if let Data::User(ref data) = edgee_event.data {
            user_properties = data.properties.clone();
        }

        if edgee_event.consent.is_some() && edgee_event.consent.unwrap() != Consent::Granted {
            // Consent is not granted, so we don't send the event
            return Err(anyhow!("Consent is not granted"));
        }

        // user properties
        // You must provide at least one of the following user property.
        if user_properties.is_empty() {
            return Err(anyhow!("User properties are empty"));
        }

        // Set user properties
        for (key, value) in user_properties.iter() {
            match key.as_str() {
                "email" => user_data.email = Some(hash_value(value)),
                "phone_number" => user_data.phone_number = Some(hash_value(value)),
                "first_name" => user_data.first_name = Some(hash_value(value)),
                "last_name" => user_data.last_name = Some(hash_value(value)),
                "gender" => user_data.gender = Some(hash_value(value)),
                "date_of_birth" => user_data.date_of_birth = Some(hash_value(value)),
                "city" => user_data.city = Some(hash_value(value)),
                "state" => user_data.state = Some(hash_value(value)),
                "zip_code" => user_data.zip_code = Some(hash_value(value)),
                "country" => user_data.country = Some(hash_value(value)),
                _ => {
                    // do nothing
                }
            }
        }

        // return error if user data doesn't have any user property
        if user_data.email.is_none() && user_data.phone_number.is_none() {
            return Err(anyhow!(
                "User properties must contain email or phone_number"
            ));
        }

        meta_event.user_data = user_data;

        Ok(meta_event)
    }
}

/// Parse value
///
/// This function is used to parse the value of a property.
/// It converts the value to a JSON value.
/// TODO: add object and array support
pub(crate) fn parse_value(value: &str) -> serde_json::Value {
    if value == "true" {
        serde_json::Value::from(true)
    } else if value == "false" {
        serde_json::Value::from(false)
    } else if value.parse::<f64>().is_ok() {
        serde_json::Value::Number(value.parse().unwrap())
    } else {
        serde_json::Value::String(value.to_string())
    }
}

/// SHA256 hash value
///
/// This function is used to hash the value.
pub(crate) fn hash_value(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}
