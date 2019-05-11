use reqwest::Client;
use urbandictionary::model::Definition;
use urbandictionary::ReqwestUrbanDictionaryRequester;

use serenity::{
    prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::{command, group}
    },
    model::channel::Message,
    utils::Colour
};


group!({
    name: "Knowledge",
    commands: [urban]
});

#[command("urban")]
#[aliases("ub", "define")]
#[usage = "<keyword>"]
#[description = "Search for a definition in Urban Dictionary"]
fn urban(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let keyword = args.parse::<String>().unwrap_or_else(|_| "".to_string());

    if keyword.is_empty() {
        let _ = message.channel_id.say(&context.http, "You need to input a anime title.");
        return Ok(());
    }

    // Code adopted from tofubot by noxim
    // github: https://owo.codes/noxim/tofu3/blob/master/src/modules/urban.rs
    let client = Client::new();
    let response: Option<Definition> = match client.define(&keyword) {
        Ok(res) => res,
        Err(why) => {
            error!("Err requesting UB definition: {:#?}", why);
            let _ = message.channel_id.say(&context.http, "Error requesting UB definition!");
            None
        }
    };

    match response {
        Some(def) => {
            // discord only accepts 2000 characters. 1800 should give us enough
            // headroom for our example field to fit
            let mut s = def.definition.clone();
            if s.len() > 1800 {
                s.truncate(1800);
            }

            match message.channel_id.send_message(&context.http, |f| f.embed(|embed| {
                embed.color(Colour::FOOYOO)
                    .title(&format!("Definition of {}", &def.word))
                    .url(&def.permalink)
                    .description(s)
                    .footer(|f| f
                        .text(&format!("Defined by {}", def.author)));

                // Only add example field if there's an example
                let example = def.example.clone();
                if !example.is_empty() {
                    embed.field("Example", example, true);
                }

                // This is a workaround since we can't order fields
                embed.field("Votes", format!("👍: **{}** 👎: **{}**",
                                                     &def.thumbs_up, &def.thumbs_down), true);

                embed
            })) {
                Ok(_) => {},
                Err(why) => error!("Sending UB failed: {:#?}", why)
            }
        },
        None => {
            let _ = message.channel_id.send_message(&context.http, |f| f.embed(|m| m
                .color(Colour::GOLD)
                .title(format!("Could not find \"{}\"", keyword))
                .description(format!(
                    "Could not find \"{}\" on Urban Dictionary. Are you \
                sure you wrote it correctly?",
                    keyword))));
        }
    }

    Ok(())
}