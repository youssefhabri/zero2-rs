use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[command]
#[owners_only]
async fn interaction(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let sub_cmd: String = args.single().unwrap_or(String::new()).to_lowercase();

    let application_info = context.http.get_current_application_info().await?;
    let application_id = *application_info.id.as_u64();

    let guild_id = message.guild_id.unwrap();
    let guild_id = *guild_id.as_u64();

    match sub_cmd.as_str() {
        "list" => {
            let commands = context
                .http
                .get_guild_application_commands(application_id, guild_id)
                .await?;
            let fields = commands.iter().map(|cmd| {
                let value = format!("**ID:** {}\n**Description:** {}", cmd.id, cmd.description);

                (cmd.name.clone(), value, false)
            });

            let _ = message
                .channel_id
                .send_message(&context, |m| m.embed(|e| e.fields(fields)))
                .await;
        }
        "del" => {
            let command_id = args.single()?;
            let _ = context
                .http
                .delete_guild_application_command(application_id, guild_id, command_id)
                .await;
        }
        _ => {}
    }

    Ok(())
}
