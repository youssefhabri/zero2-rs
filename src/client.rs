use serenity::{prelude::*, Client};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::core::{framework::Zero2Framework, handler::Zero2Handler, store::*};

pub struct Zero2Client(Client);

impl Zero2Client {
    pub fn new() -> Self {
        // Load token from environment variables or .env file
        let token: String = dotenv::var("DISCORD_TOKEN").expect("token");

        let mut client =
            Client::new(&token, Zero2Handler::default()).expect("Error creating client");

        let owner = match client.cache_and_http.http.get_current_application_info() {
            Ok(info) => info.owner,
            Err(why) => panic!("Couldn't get application info: {:?}", why),
        };

        let mut owner_ids = HashSet::new();
        owner_ids.insert(owner.id);

        {
            let mut data = client.data.write();
            data.insert::<CommandLogger>(HashMap::default());
            data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
            data.insert::<MessagePaginator>(HashMap::default());
            data.insert::<BotOwnerContainer>(owner);
        }

        client.with_framework(Zero2Framework::with_owners(owner_ids));

        Zero2Client(client)
    }

    pub fn start(&mut self) -> Result<(), SerenityError> {
        self.0.start()
    }
    pub fn start_autosharded(&mut self) -> Result<(), SerenityError> {
        self.0.start_autosharded()
    }
}
