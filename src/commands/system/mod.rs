pub mod cleanup;
pub mod interaction;

use serenity::framework::standard::macros::group;

use cleanup::CLEANUP_COMMAND;
use interaction::INTERACTION_COMMAND;

#[group]
#[commands(cleanup, interaction)]
struct System;
