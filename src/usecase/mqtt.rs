use base64::Engine;
use base64::engine::general_purpose;
use paho_mqtt::{AsyncClient, Message};

use crate::domain::object::door::Door;
use crate::domain::object::mqtt::door_state::DoorState;
use crate::domain::object::mqtt::door_switch_state::DoorSwitchState;
use crate::domain::object::mqtt::key_state::KeyState;
use crate::domain::object::mqtt::mqtt_card::MqttCard;
use crate::domain::repository::card::CardRepository;
use crate::domain::repository::door::DoorRepository;
use crate::domain::repository::mqtt::client::MqttClientRepository;
use crate::domain::repository::register::RegisterRepository;
use crate::infrastructures::config::mqtt_config::MqttConfig;
use crate::server::connection::RequestContext;
use crate::server::handler::mqtt_listener::mqtt_register_listener;

pub async fn run(mut client: impl MqttClientRepository, cfg: MqttConfig) -> anyhow::Result<()> {
    client.connect().await?;
    println!("connected");
    mqtt_register_listener(&mut client, cfg.clone());
    println!("Register listeners");
    client
        .publish("test/test_message", "API UP")
        .expect("failed publish message");
    client.start();
    Ok(())
}

pub fn check_card(
    _client: &AsyncClient,
    msg: &Message,
    _data: &RequestContext,
    key_state_path: String,
    door_state_path: String,
    door_switch_state_path: String,
) {
    let result = serde_json::from_str::<MqttCard>(&msg.payload_str());

    if let Err(e) = result {
        println!("Failed to parse Error: {}", e);
        return;
    }

    let card = result.unwrap();

    if _data.register_repository().is_register_mode() {
        _data.register_repository().add_card(card.id.clone());
        return;
    }

    let decoded_id = general_purpose::STANDARD.decode(card.id.clone()).unwrap();

    if let Err(e) = _data.card_repository().find_by_card_number(&decoded_id) {
        println!("Error: {}", e);
        return;
    }

    if !_data
        .card_repository()
        .find_by_card_number(&decoded_id)
        .unwrap()
    {
        println!("Card not found {:?}", decoded_id);
        return;
    }

    let result = _data
        .door_repository()
        .find_by_device_id(card.device_id.clone());

    if let Err(e) = result {
        println!("Error: {}", e);

        request_door_states(
            _client,
            card.device_id,
            door_state_path,
            door_switch_state_path,
        );
        return;
    }

    let key_state = KeyState {
        device_id: card.device_id.clone(),
        open: !result.unwrap().door_switch_state,
        timestamp: chrono::offset::Local::now(),
    };

    publish_key(_client, key_state_path, key_state);
}

pub fn request_door_states(
    client: &AsyncClient,
    _device_id: String,
    door_state_path: String,
    door_switch_state_path: String,
) {
    let door_state_request = DoorState {
        device_id: _device_id.clone(),
        is_open: true,
        timestamp: chrono::offset::Local::now(),
    };
    let door_switch_request = DoorState {
        device_id: _device_id.clone(),
        is_open: true,
        timestamp: chrono::offset::Local::now(),
    };

    let door_json_str = serde_json::to_string(&door_state_request).unwrap();
    let door_switch_json_str = serde_json::to_string(&door_switch_request).unwrap();

    client.publish(Message::new(door_state_path, door_json_str, 0));
    client.publish(Message::new(
        door_switch_state_path,
        door_switch_json_str,
        0,
    ));
}

pub fn publish_key(client: &AsyncClient, key_state_path: String, key_state: KeyState) {
    let json_str = serde_json::to_string(&key_state).unwrap();

    client.publish(Message::new(key_state_path, json_str, 0));
}

pub fn update_door_state(_data: &RequestContext, _door_state: DoorState) {
    let result = _data
        .door_repository()
        .find_by_device_id(_door_state.device_id.clone());
    if let Err(e) = result {
        println!("Error: {}", e);
        insert_default_door_states(_data, _door_state.device_id.clone());
        return;
    }
    let old_state = result.unwrap();
    let _device_id = _door_state.device_id;
    let new_state = Door {
        door_state: _door_state.is_open,
        door_switch_state: old_state.door_switch_state,
        device_id: _device_id,
    };
    _data
        .door_repository()
        .status_update(new_state)
        .expect("TODO: panic message");
}

pub fn update_door_switch_state(_data: &RequestContext, _door_switch_state: DoorSwitchState) {
    let result = _data
        .door_repository()
        .find_by_device_id(_door_switch_state.device_id.clone());
    if let Err(e) = result {
        println!("Error: {}", e);
        insert_default_door_states(_data, _door_switch_state.device_id.clone());
        return;
    }
    let old_state = result.unwrap();
    let _device_id = _door_switch_state.device_id;
    let new_state = Door {
        door_state: old_state.door_state,
        door_switch_state: _door_switch_state.is_open,
        device_id: _device_id,
    };
    _data
        .door_repository()
        .status_update(new_state)
        .expect("TODO: panic message");
}

fn insert_default_door_states(_data: &RequestContext, _device_id: String) {
    _data
        .door_repository()
        .insert(
            DoorState {
                is_open: false,
                timestamp: Default::default(),
                device_id: _device_id.clone(),
            },
            DoorSwitchState {
                is_open: false,
                timestamp: Default::default(),
                device_id: _device_id.clone(),
            },
        )
        .expect("TODO: panic message");
}
