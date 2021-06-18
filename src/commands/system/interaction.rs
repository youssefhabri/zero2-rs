use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::{CommandId, Message};
use serenity::prelude::Context;

#[command]
#[owners_only]
async fn interaction(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let sub_cmd: String = args.single::<String>().unwrap_or_default().to_lowercase();

    let guild_id = message.guild_id.unwrap();

    match sub_cmd.as_str() {
        "list" => {
            let commands = guild_id.get_application_commands(&context).await.unwrap();
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
            let command_id = CommandId(args.single()?);
            let _ = guild_id
                .delete_application_command(&context, command_id)
                .await;
        }
        _ => {}
    }

    Ok(())
}
