use std::sync::Arc;

use async_trait::async_trait;
use pumpkin_protocol::java::client::play::{ArgumentType, CommandSuggestion, SuggestionProviders};

use crate::command::CommandSender;
use crate::command::dispatcher::CommandError;
use crate::command::tree::RawArgs;
use crate::entity::player::Player;
use crate::server::Server;

use super::super::args::ArgumentConsumer;
use super::players::PlayersArgumentConsumer;
use super::{Arg, DefaultNameArgConsumer, FindArg, GetClientSideArgParser};

/// todo: implement (currently just calls [`super::arg_player::PlayerArgumentConsumer`])
///
/// For selecting zero, one or multiple entities, eg. using @s, a player name, @a or @e
pub struct EntitiesArgumentConsumer;

impl GetClientSideArgParser for EntitiesArgumentConsumer {
    fn get_client_side_parser(&self) -> ArgumentType {
        // todo: investigate why this does not accept target selectors
        ArgumentType::Entity { flags: 0 }
    }

    fn get_client_side_suggestion_type_override(&self) -> Option<SuggestionProviders> {
        None
    }
}

#[async_trait]
impl ArgumentConsumer for EntitiesArgumentConsumer {
    async fn consume<'a>(
        &'a self,
        src: &CommandSender,
        server: &'a Server,
        args: &mut RawArgs<'a>,
    ) -> Option<Arg<'a>> {
        // todo
        match PlayersArgumentConsumer.consume(src, server, args).await {
            Some(Arg::Players(p)) => Some(Arg::Entities(p)),
            _ => None,
        }
    }

    async fn suggest<'a>(
        &'a self,
        _sender: &CommandSender,
        _server: &'a Server,
        _input: &'a str,
    ) -> Result<Option<Vec<CommandSuggestion>>, CommandError> {
        Ok(None)
    }
}

impl DefaultNameArgConsumer for EntitiesArgumentConsumer {
    fn default_name(&self) -> &'static str {
        "targets"
    }
}

impl<'a> FindArg<'a> for EntitiesArgumentConsumer {
    type Data = &'a [Arc<Player>];

    fn find_arg(args: &'a super::ConsumedArgs, name: &str) -> Result<Self::Data, CommandError> {
        match args.get(name) {
            Some(Arg::Entities(data)) => Ok(data),
            _ => Err(CommandError::InvalidConsumption(Some(name.to_string()))),
        }
    }
}
