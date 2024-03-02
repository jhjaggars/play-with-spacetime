use spacetimedb::{spacetimedb, Identity, ReducerContext, Timestamp};

#[spacetimedb(table)]
pub struct User {
    #[primarykey]
    identity: Identity,
    name: Option<String>,
    online: bool,
}

#[spacetimedb(table)]
pub struct Message {
    sender: Identity,
    sent: Timestamp,
    text: String,
}

#[spacetimedb(table)]
pub struct Character {
    #[primarykey]
    identity: Identity,
    level: u16,
    title: Option<String>,
}

fn validate_title(title: String) -> Result<String, String> {
    if title.is_empty() {
        return Err("Title cannot be empty".to_string());
    }
    Ok(title)
}

#[spacetimedb(reducer)]
pub fn set_title(ctx: ReducerContext, title: String) -> Result<(), String> {
    let title = validate_title(title)?;
    if let Some(character) = Character::filter_by_identity(&ctx.sender) {
        Character::update_by_identity(
            &ctx.sender,
            Character {
                title: Some(title),
                ..character
            },
        );
        Ok(())
    } else {
        Err("Cannot set title for unknown user".to_string())
    }
}

fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    Ok(name)
}

#[spacetimedb(reducer)]
pub fn set_name(ctx: ReducerContext, name: String) -> Result<(), String> {
    let name = validate_name(name)?;
    if let Some(user) = User::filter_by_identity(&ctx.sender) {
        User::update_by_identity(
            &ctx.sender,
            User {
                name: Some(name),
                ..user
            },
        );
        Ok(())
    } else {
        Err("Cannot set name for unknown user".to_string())
    }
}

fn validate_message(text: String) -> Result<String, String> {
    if text.is_empty() {
        Err("Messages must not be empty".to_string())
    } else {
        Ok(text)
    }
}

#[spacetimedb(reducer)]
pub fn send_message(ctx: ReducerContext, text: String) -> Result<(), String> {
    let text = validate_message(text)?;
    log::info!("{}", text);
    Message::insert(Message {
        sender: ctx.sender,
        text,
        sent: ctx.timestamp,
    });
    Ok(())
}

#[spacetimedb(connect)]
pub fn identity_connected(ctx: ReducerContext) {
    if let Some(user) = User::filter_by_identity(&ctx.sender) {
        User::update_by_identity(
            &ctx.sender,
            User {
                online: true,
                ..user
            },
        );
    } else {
        User::insert(User {
            identity: ctx.sender,
            name: None,
            online: true,
        })
        .unwrap();
    }

    if let Some(_character) = Character::filter_by_identity(&ctx.sender) {
        // idk
    } else {
        Character::insert(Character {
            identity: ctx.sender,
            title: None,
            level: 1,
        })
        .unwrap();
    }
}

#[spacetimedb(disconnect)]
pub fn identity_disconnected(ctx: ReducerContext) {
    if let Some(user) = User::filter_by_identity(&ctx.sender) {
        User::update_by_identity(
            &ctx.sender,
            User {
                online: false,
                ..user
            },
        );
    } else {
        log::warn!(
            "Disconnect event for unknown user with idenity {:?}",
            ctx.sender
        );
    }
}
