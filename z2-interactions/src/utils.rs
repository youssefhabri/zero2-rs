use serenity::{
    builder::CreateInteractionOption,
    model::prelude::{ApplicationCommandOptionType, GuildId},
    prelude::{Context, SerenityError},
};

lazy_static::lazy_static! {
    static ref GUILD_ID: u64 = kankyo::key("INTERACTIONS_GUILD_ID").unwrap().parse().unwrap();
}

pub struct CommandOptionChoice<'a> {
    name: &'a str,
    value: serde_json::Value,
}

impl<'a> CommandOptionChoice<'a> {
    pub fn string(name: &'a str, value: impl ToString) -> Self {
        let value = serde_json::Value::String(value.to_string());
        CommandOptionChoice { name, value }
    }

    pub fn int(name: &'a str, value: u32) -> Self {
        let value = serde_json::Value::Number(value.into());
        CommandOptionChoice { name, value }
    }
}

pub struct CommandOption<'a> {
    name: &'a str,
    description: &'a str,
    required: bool,
    kind: ApplicationCommandOptionType,
    choices: Option<Vec<CommandOptionChoice<'a>>>, // TODO add choice types
}

impl<'a> CommandOption<'a> {
    pub fn string(name: &'a str, description: &'a str) -> CommandOption<'a> {
        Self::string_with_choices(name, description, Vec::new())
    }

    pub fn string_with_choices(
        name: &'a str,
        description: &'a str,
        choices: Vec<CommandOptionChoice<'a>>,
    ) -> CommandOption<'a> {
        CommandOption {
            name,
            description,
            required: true,
            kind: ApplicationCommandOptionType::String,
            choices: Some(choices),
        }
    }

    pub fn user(required: bool) -> CommandOption<'a> {
        CommandOption {
            name: "user",
            description: "User",
            required,
            kind: ApplicationCommandOptionType::User,
            choices: None,
        }
    }
}

pub async fn regitser_command<'a>(
    context: &Context,
    guild_id: GuildId,
    app_id: u64,
    name: &str,
    description: &str,
    opts: Vec<CommandOption<'a>>,
) -> Result<(), SerenityError> {
    let options = opts
        .into_iter()
        .map(|opt| {
            let mut new_opt = CreateInteractionOption::default()
                .name(opt.name)
                .description(opt.description)
                .required(opt.required)
                .kind(opt.kind)
                .to_owned();

            match opt.kind {
                ApplicationCommandOptionType::String => {
                    opt.choices.map(|choices| {
                        choices.iter().for_each(|choice| {
                            if let Some(value) = choice.value.as_str() {
                                new_opt.add_string_choice(choice.name, value);
                            }
                        });
                    });
                }
                ApplicationCommandOptionType::Integer => {
                    opt.choices.map(|choices| {
                        choices.iter().for_each(|choice| {
                            if let Some(value) = choice.value.as_i64() {
                                new_opt.add_int_choice(choice.name, value as i32);
                            }
                        });
                    });
                }
                _ => {}
            }

            new_opt
        })
        .collect();

    guild_id
        .create_application_command(&context, app_id, |ci| {
            ci.name(name)
                .description(description)
                .set_interaction_options(options)
        })
        .await?;

    Ok(())
}
