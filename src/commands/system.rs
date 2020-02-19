mod blacklist;
mod cleanup;
mod dm;
mod echo;
mod embed;
mod log;
mod shutdown;

use serenity::framework::standard::macros::group;

use self::cleanup::*;
use self::dm::*;
use self::echo::*;
use self::embed::*;
use self::log::*;
use self::shutdown::*;

#[group]
#[commands(cleanup, dm, echo, embed, log, shutdown)]
struct System;
