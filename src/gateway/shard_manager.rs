use crate::{error::Result, gateway::shard::ReconnectionKind, model::gateway::event::Event};

use super::shard::{Shard, ShardAction};

#[derive(Debug)]
pub struct ShardManager {
    pub shard: Shard,
}

impl ShardManager {
    pub async fn run(&mut self) -> Result<()> {
        loop {
            if !self.shard.do_heartbeat_interval().await {
                println!("heartbeat failed");
            }
            let (event, action) = self.receive_event().await?;

            if event.is_some() || action.is_some() {
                if let Some(shard_information) = self.shard.shard_information {
                    println!(
                        "Shard {}/{}\n-----\nEvent: {event:?}\nAction: {action:?}\n\n",
                        shard_information.id, shard_information.total
                    );
                }
            }

            match action {
                Some(ShardAction::Reconnect(kind)) => {
                    self.shard.reset(kind == ReconnectionKind::Resume);
                    match kind {
                        ReconnectionKind::Resume => self.shard.resume().await?,
                        ReconnectionKind::Identify => self.shard.identify().await?,
                    }
                }
                Some(ShardAction::Heartbeat) => self.shard.heartbeat().await?,
                Some(ShardAction::Identify) => self.shard.identify().await?,
                None => {}
            }
        }
    }

    async fn receive_event(&mut self) -> Result<(Option<Event>, Option<ShardAction>)> {
        let Some(gateway_event) = self.shard.websocket.receive().await? else {
            return Ok((None, None));
        };
        let action = self.shard.handle_event(Ok(&gateway_event))?;

        Ok((Some(gateway_event), action))
    }
}
