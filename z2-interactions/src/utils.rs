use serenity::{
    builder::CreateApplicationCommandOption,
    model::prelude::application_command::{
        ApplicationCommandInteraction, ApplicationCommandOptionType,
    },
    model::prelude::{GuildId, Interaction},
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
    name: &str,
    description: &str,
    opts: Vec<CommandOption<'a>>,
) -> Result<(), SerenityError> {
    let options = opts.into_iter().map(map_command_option).collect();

    guild_id
        .create_application_command(&context, |cmd| {
            cmd.name(name).description(description).set_options(options)
        })
        .await?;

    Ok(())
}

fn map_command_option(option: CommandOption) -> CreateApplicationCommandOption {
    let mut new_opt = CreateApplicationCommandOption::default()
        .name(option.name)
        .description(option.description)
        .required(option.required)
        .kind(option.kind)
        .to_owned();

    match option.kind {
        ApplicationCommandOptionType::String => {
            option.choices.map(|choices| {
                choices.iter().for_each(|choice| {
                    if let Some(value) = choice.value.as_str() {
                        new_opt.add_string_choice(choice.name, value);
                    }
                });
            });
        }
        ApplicationCommandOptionType::Integer => {
            option.choices.map(|choices| {
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
}

pub fn get_application_command(
    interaction: &Interaction,
) -> Result<ApplicationCommandInteraction, SerenityError> {
    match interaction.clone().application_command() {
        Some(application_command) => Ok(application_command),
        None => {
            return Err(SerenityError::Other(
                "application_command() is None. TODO: user custom error type",
            ))
        }
    }
}
