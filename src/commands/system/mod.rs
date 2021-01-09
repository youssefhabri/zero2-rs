pub mod cleanup;

use serenity::framework::standard::macros::group;

use cleanup::CLEANUP_COMMAND;

#[group]
#[commands(cleanup)]
struct System;
