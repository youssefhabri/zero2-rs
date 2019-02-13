use reqwest::Client;
use urbandictionary::model::Definition;
use urbandictionary::ReqwestUrbanDictionaryRequester;

use serenity::{
    prelude::*,
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

                // discord doesn't allow empty fields, so add placeholder in case no
                // example
                // TODO: I guess we just shouldn't send a field in case this is empty
                let mut e = def.example.clone();
                if e.is_empty() {
                    e = "<none>".into();
                }

                match message.channel_id.send_message(|f| f.embed(|m| m
                    .color(Colour::FOOYOO)
                    .title(&format!("Definition of {}", &def.word))
                    .url(&def.permalink)
                    .description(s)
                    .field("Example", e, true)
                    .field("Votes", format!("👍: **{}** 👎: **{}**",
                                            &def.thumbs_up, &def.thumbs_down), true)
                    .footer(|f| f
                        .text(&format!("Defined by {}", def.author)))
                )) {
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