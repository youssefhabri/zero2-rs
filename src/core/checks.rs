use serenity::framework::standard::{macros::check, Reason};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use super::consts::OWNER_ID;

#[check]
#[name = "Owner"]
async fn owner_check(_: &Context, msg: &Message) -> Result<(), Reason> {
    if msg.author.id == OWNER_ID {
        return Ok(());
    }

    Err(Reason::User("User is not Mittens".to_string()))
}

#[check]
#[name = "Admin"]
async fn admin_check(context: &Context, message: &Message) -> Result<(), Reason> {
    if let Ok(member) = message.member(&context).await {
        if let Ok(permissions) = member.permissions(&context).await {
            if permissions.administrator() {
                return Ok(());
            }
        }
    }

    Err(Reason::User("User lacked admin permission.".to_string()))
}
