mod module_bindings;
use module_bindings::*;

use spacetimedb_sdk::{
    disconnect,
    identity::{load_credentials, once_on_connect, save_credentials, Credentials, Identity},
    on_disconnect, on_subscription_applied,
    reducer::Status,
    subscribe,
    table::{TableType, TableWithPrimaryKey},
    Address,
};

fn main() {
    register_callbacks();
    connect_to_db();
    subscribe_to_tables();
    user_input_loop();
}

fn register_callbacks() {
    once_on_connect(on_connected);

    User::on_insert(on_user_inserted);

    User::on_update(on_user_updated);

    Message::on_insert(on_message_inserted);

    on_subscription_applied(on_sub_applied);

    on_set_name(on_name_set);

    on_send_message(on_message_sent);

    on_disconnect(on_disconnected);
}

fn on_connected(creds: &Credentials, _client_address: Address) {
    if let Err(e) = save_credentials(CREDS_DIR, creds) {
        eprintln!("Failed to save credentials: {:?}", e);
    }
}

const CREDS_DIR: &str = ".spacetime_chat";

fn on_user_inserted(user: &User, _: Option<&ReducerEvent>) {
    if user.online {
        println!("User {} connected.", user_name_or_identity(user));
    }
}

fn user_name_or_identity(user: &User) -> String {
    user.name
        .clone()
        .unwrap_or_else(|| identity_leading_hex(&user.identity))
}

fn identity_leading_hex(id: &Identity) -> String {
    hex::encode(&id.bytes()[0..8])
}

fn on_user_updated(old: &User, new: &User, _: Option<&ReducerEvent>) {
    if old.name != new.name {
        println!(
            "User {} renamed to {}.",
            user_name_or_identity(old),
            user_name_or_identity(new)
        );
    }
    if old.online && !new.online {
        println!("User {} disconnected.", user_name_or_identity(new));
    }
    if !old.online && new.online {
        println!("User {} connected.", user_name_or_identity(new));
    }
}

fn on_message_inserted(message: &Message, reducer_event: Option<&ReducerEvent>) {
    if reducer_event.is_some() {
        print_message(message);
    }
}

fn title_or_default(character: &Character) -> String {
    character
        .title
        .clone()
        .unwrap_or_else(|| "unset".to_string())
}

fn print_message(message: &Message) {
    let sender = User::filter_by_identity(message.sender.clone())
        .map(|u| user_name_or_identity(&u))
        .unwrap_or_else(|| "unknown".to_string());
    let title = Character::filter_by_identity(message.sender.clone())
        .map(|c| title_or_default(&c))
        .unwrap_or_else(|| "unknown".to_string());
    println!("({}) {}: {}", title, sender, message.text);
}

fn on_sub_applied() {
    let mut messages = Message::iter().collect::<Vec<_>>();
    messages.sort_by_key(|m| m.sent);
    for message in messages {
        print_message(&message);
    }
}

fn on_name_set(
    _sender_id: &Identity,
    _sender_address: Option<Address>,
    status: &Status,
    name: &String,
) {
    if let Status::Failed(err) = status {
        eprintln!("Failed to change name to {:?}: {}", name, err);
    }
}

fn on_message_sent(
    _sender_id: &Identity,
    _sender_address: Option<Address>,
    status: &Status,
    text: &String,
) {
    if let Status::Failed(err) = status {
        eprintln!("Failed to send message {:?}: {}", text, err);
    }
}

fn on_disconnected() {
    eprintln!("Disconnected");
    std::process::exit(0);
}

const SPACETIMEDB_URI: &str = "http://localhost:3000";

const DB_NAME: &str = "chat";

fn connect_to_db() {
    connect(
        SPACETIMEDB_URI,
        DB_NAME,
        load_credentials(CREDS_DIR).expect("Error reading stored credentials"),
    )
    .expect("Failed to connect");
}

fn subscribe_to_tables() {
    subscribe(&[
        "SELECT * FROM User;",
        "SELECT * FROM Message;",
        "SELECT * FROM Character;",
    ])
    .unwrap();
}

fn user_input_loop() {
    for line in std::io::stdin().lines() {
        let Ok(line) = line else {
            panic!("Failed to read from stdin.");
        };
        if let Some(name) = line.strip_prefix("/name ") {
            set_name(name.to_string());
        } else if let Some(title) = line.strip_prefix("/title ") {
            set_title(title.to_string());
        } else {
            send_message(line);
        }
    }
}
