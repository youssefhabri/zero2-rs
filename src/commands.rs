use serenity::framework::standard::{StandardFramework, HelpBehaviour, help_commands};

pub mod anilist;
pub mod giphy;
pub mod fun;
pub mod meta;
pub mod nekoslife;
pub mod urban;

pub fn register(mut framework: StandardFramework) -> StandardFramework {
    framework = anilist::register(framework);
    framework = fun::register(framework);
    framework = meta::register(framework);
    framework = nekoslife::register(framework);
    framework = urban::register(framework);

    // No category
    framework = no_category(framework);

    framework
}

fn no_category(mut framework: StandardFramework) -> StandardFramework {
    framework = framework
        .command("gif", |c| c.cmd(giphy::GiphyCommand))
        .customised_help(help_commands::with_embeds, |c| c
            .individual_command_tip("Hello! こんにちは！Hola! Bonjour! 您好!\n\
                If you want more information about a specific command, just pass the command as argument.")
            .command_not_found_text("Could not find: `{}`.")
            .max_levenshtein_distance(3)
            .lacking_permissions(HelpBehaviour::Hide)
            .lacking_role(HelpBehaviour::Nothing)
            .wrong_channel(HelpBehaviour::Strike)
        );

    framework
}