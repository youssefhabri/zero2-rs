use menu::types::PaginationContainer;
use serenity::client::{bridge::gateway::GatewayIntents, Client as SerenityClient};
use serenity::http::Http;
use serenity::prelude::{RwLock, SerenityError};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::core::event_handler::Zero2EventHandler;
use crate::core::framework::Zero2Framework;
use crate::core::store::ShardManagerContainer;

pub struct Zero2Client {
    client: SerenityClient,
}

impl Zero2Client {
    pub async fn new() -> Zero2Client {
        let token = kankyo::key("DISCORD_TOKEN").expect("Expected a token in the environment");
        let http = Http::new_with_token(&token);
        let app_info = match http.get_current_application_info().await {
            Ok(info) => info,
            Err(why) => panic!("Could not access application info: {:?}", why),
        };

        let mut owners = HashSet::new();
        owners.insert(app_info.owner.id);

        let framework = Zero2Framework::with_info(owners, Some(app_info.id));

        let intents = GatewayIntents::all();

        let client = SerenityClient::builder(&token)
            .event_handler(Zero2EventHandler)
            .intents(intents)
            .framework(framework)
            .await
            .expect("Err creating client");

        {
            let mut data = client.data.write().await;
            data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
            data.insert::<PaginationContainer>(Arc::new(RwLock::new(HashMap::new())));
        }

        let shard_manager = client.shard_manager.clone();

        tokio::spawn(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Could not register ctrl+c handler");
            shard_manager.lock().await.shutdown_all().await;
        });

        Zero2Client { client }
    }

    pub async fn start(&mut self) -> Result<(), SerenityError> {
        self.client.start_autosharded().await
    }
}
