use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MCRequestHeader {
    request_id: String,
    message_purpose: String,
    version: u32,
    message_type: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MCEventSubscribeRequestBody {
    event_name: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MCEventSubscribeRequest {
    body: MCEventSubscribeRequestBody,
    header: MCRequestHeader,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MCCommandRequestOrigin {
    r#type: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MCCommandRequestBody {
    origin: MCCommandRequestOrigin,
    command_line: String,
    version: u32,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MCCommandRequest {
    body: MCCommandRequestBody,
    header: MCRequestHeader,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MCMessageHeader {
    pub message_purpose: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MCMessageBodyProperty {
    pub sender: String,
    pub message: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MCMessageBody {
    pub event_name: Option<String>,
    pub properties: Option<MCMessageBodyProperty>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MCMessage {
    pub header: MCMessageHeader,
    pub body: MCMessageBody,
}

pub fn parse_message(msg: &str) -> MCMessage {
    serde_json::from_str(msg).unwrap()
}

pub fn create_player_message_subscribe_command() -> String {
    let player_chat_event_command = MCEventSubscribeRequest{
        body: MCEventSubscribeRequestBody{
            event_name: "PlayerMessage".to_owned(),
        },
        header: MCRequestHeader{
            request_id: Uuid::new_v4().to_hyphenated().to_string(),
            message_purpose: "subscribe".to_owned(),
            version: 1,
            message_type: "commandRequest".to_owned(),
        }
    };
    serde_json::to_string(&player_chat_event_command).unwrap()
}

pub fn create_set_block_command(x: u32, y: u32, z: u32, block_type: String, replace_type: String) -> String {
    let command = MCCommandRequest{
        body: MCCommandRequestBody{
            origin: MCCommandRequestOrigin{
                r#type: "player".to_owned(),
            },
            command_line: format!("setblock {} {} {} {} {}", x, y, z, block_type, replace_type),
            version: 1,
        },
        header: MCRequestHeader{
            request_id: Uuid::new_v4().to_hyphenated().to_string(),
            message_purpose: "commandRequest".to_owned(),
            version: 1,
            message_type: "commandRequest".to_owned(),
        }
    };
    serde_json::to_string(&command).unwrap()
}