use serenity::framework::standard::{macros::check, CheckResult, Reason};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use super::consts::OWNER_ID;

#[check]
#[name = "Owner"]
async fn owner_check(_: &Context, msg: &Message) -> CheckResult {
    if msg.author.id == OWNER_ID {
        return CheckResult::Success;
    }

    CheckResult::Failure(Reason::User("User is not Mittens".to_string()))
}

#[check]
#[name = "Admin"]
async fn admin_check(context: &Context, message: &Message) -> CheckResult {
    if let Ok(member) = message.member(&context).await {
        if let Ok(permissions) = member.permissions(&context).await {
            return permissions.administrator().into();
        }
    }

    false.into()
}
