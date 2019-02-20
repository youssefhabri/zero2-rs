use reqwest::Client;
use urbandictionary::model::Definition;
use urbandictionary::ReqwestUrbanDictionaryRequester;

use serenity::{
    prelude::*,
    builder::CreateEmbed,
    framework::StandardFramework,
    framework::standard::{Args, Command, CommandError},
    model::channel::Message,
    utils::Colour
};


pub fn register(framework: StandardFramework) -> StandardFramework {
    framework.group("Knowledge", |cg| cg
        .command("urban", |c| c
            .cmd(UrbanDictionary)
            .batch_known_as(vec!["ud", "define"])
            .usage("<keyword>")
            .desc("Search for a definition in Urban Dictionary")
        )
    )
}

pub struct UrbanDictionary;

impl Command for UrbanDictionary {
    fn execute(&self, _context: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {
        if args.full().len() <= 0 {
            let _ = message.channel_id.say("You need to input a anime title.");
            return Ok(());
        }

        let keyword = args.full();

        // Code adopted from tofubot by noxim
        // github: https://owo.codes/noxim/tofu3/blob/master/src/modules/urban.rs
        let client = Client::new();
        let response: Option<Definition> = match client.define(&keyword) {
            Ok(res) => res,
            Err(why) => {
                error!("Err requesting UB definition: {:#?}", why);
                let _ = message.channel_id.say("Error requesting UB definition!");
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

                let mut embed = CreateEmbed::default()
                    .color(Colour::FOOYOO)
                    .title(&format!("Definition of {}", &def.word))
                    .url(&def.permalink)
                    .description(s)
                    .footer(|f| f
                        .text(&format!("Defined by {}", def.author)));

                // Only add example field if there's an example
                let example = def.example.clone();
                if !example.is_empty() {
                    embed = embed.field("Example", example, true);
                }

                // This is a workaround since we can't order fields
                embed = embed.field("Votes", format!("👍: **{}** 👎: **{}**",
                                            &def.thumbs_up, &def.thumbs_down), true);

                match message.channel_id.send_message(|f| f.embed(|_| embed)) {
                    Ok(_) => {},
                    Err(why) => error!("Sending UB failed: {:#?}", why)
                }
            },
            None => {
                let _ = message.channel_id.send_message(|f| f.embed(|m| m
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
}