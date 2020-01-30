use reqwest::blocking::Client;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
    utils::Colour,
};
use urbandictionary::model::Definition;
use urbandictionary::ReqwestUrbanDictionaryRequester;

use crate::menu;
use crate::menu::builders;

group!({
    name: "Knowledge",
    commands: [urban]
});

#[command]
#[aliases("ud", "define")]
#[usage = "<keyword>"]
#[description = "Search for a definition in Urban Dictionary"]
fn urban(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        let _ = message
            .channel_id
            .say(&context.http, "You need to input a keyword.");
        return Ok(());
    }

    let keyword = args.message().to_string();

    // Code adopted from tofubot by noxim
    // github: https://owo.codes/noxim/tofu3/blob/master/src/modules/urban.rs
    let client = Client::new();
    let definitions: Vec<Definition> = match client.definitions(&keyword) {
        Ok(res) => res.definitions,
        Err(why) => {
            error!("Err requesting UB definition: {:#?}", why);
            let _ = message
                .channel_id
                .say(&context.http, "Error requesting UB definition!");
            vec![]
        }
    };

    if definitions.is_empty() {
        let _ = message.channel_id.send_message(&context.http, |f| {
            f.embed(|m| {
                m.color(Colour::GOLD)
                    .title(format!("Could not find \"{}\"", keyword))
                    .description(format!(
                        "Could not find \"{}\" on Urban Dictionary. Are you \
                         sure you wrote it correctly?",
                        keyword
                    ))
            })
        });

        return Ok(());
    }

    let definition: &Definition = &definitions[0];
    let sending = message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.clone_from(&builders::urban_embed_builder(
                definition,
                format!("Page: {}/{} | ", 1, definitions.len()),
            ));

            e
        })
        .reactions(menu::reactions::default())
    });

    if let Ok(sending_msg) = sending {
        menu::new_pagination(
            context,
            sending_msg.id,
            message.author.id,
            builders::pages_builder::<Definition>(definitions, builders::urban_embed_builder),
        )
    }

    Ok(())
}
