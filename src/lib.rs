#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_possible_truncation)]
// TODO: Remove up to missing_docs comment
#![allow(clippy::missing_errors_doc)]
// #![deny(missing_docs)]

//! # discors
//!
//! A new in-development Discord library written in Rust

pub mod error;
pub mod gateway;
pub mod model;

#[tokio::test]
async fn test() {
    let _ = dotenvy::dotenv();
    use gateway::{
        shard::{Shard, ShardInformation},
        shard_manager::ShardManager,
    };
    use model::gateway::intents::GatewayIntents;

    let total_shards = 1;
    for i in 0..total_shards {
        tokio::task::spawn(async move {
            ShardManager {
                shard: Shard::new(
                    "wss://gateway.discord.gg/?v=10&encoding=json",
                    std::env::var("DISCORD_TOKEN").unwrap().as_str(),
                    ShardInformation {
                        id: i,
                        total: total_shards,
                    },
                    GatewayIntents::non_privileged(),
                )
                .await
                .unwrap(),
            }
            .run()
            .await
            .unwrap();
        });
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
