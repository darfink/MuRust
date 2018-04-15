use super::{util, PacketSink, PacketStream};
use failure::{Context, Error, ResultExt};
use futures::prelude::*;
use murust_data_model::entities::{Account, Character};
use murust_data_model::types::Class;
use murust_service::{CharacterCreateError, CharacterDeleteError, CharacterService};
use protocol::game::{server, Client};
use std::time::Duration;

#[allow(unused_unsafe)]
#[async(boxed_send)]
pub fn serve<S: PacketStream + PacketSink + Send + 'static>(
  (account, character_service): (Account, CharacterService),
  stream: S,
) -> Result<(Character, S), Error> {
  // Only fetch the account's characters once
  let characters = character_service
    .find_by_account_id(account.id)
    .context("Character service failed to retrieve account characters")?;
  await!(serve_impl(account, characters, character_service, stream))
}

#[allow(unused_unsafe)]
#[async(boxed_send)]
pub fn serve_impl<S: PacketStream + PacketSink + Send + 'static>(
  account: Account,
  mut characters: Vec<Character>,
  character_service: CharacterService,
  stream: S,
) -> Result<(Character, S), Error> {
  // Process one incoming packet at a time.
  let (packet, mut stream) = await!(stream.next_packet())?;

  match Client::from_packet(&packet)? {
    Client::CharacterListRequest => {
      // TODO: Dynamically decide the maximum class for the character.
      let list = server::CharacterList::new(Class::FairyElf, &characters);
      stream = await!(stream.send_packet(&list))?;

      // The client might crash unless there's a delay between these packets
      await!(util::delay(Duration::from_millis(250)))?;

      let motd = "Welcome to Mu Online in Rust!";
      let motd = server::Message::Notice(motd.into());
      stream = await!(stream.send_packet(&motd))?;
    },
    Client::CharacterCreate(request) => {
      let result = character_service
        .create(&request.name, request.class, account.id)
        .context("Character service failed to create new character")?;

      match result {
        Ok(character) => {
          let success = server::CharacterCreateResult::success(&character);
          stream = await!(stream.send_packet(&success))?;
          characters.push(character);
        },
        Err(error) => {
          let failure = map_character_create_error(error);
          stream = await!(stream.send_packet(&failure))?;
        },
      }
    },
    Client::CharacterDelete(request) => {
      let position = characters
        .iter()
        .position(|c| c.name == request.name)
        .ok_or_else(|| {
          let error = "Client sent invalid character name for deletion";
          Error::from(Context::new(error))
        })?;

      // TODO: Avoid allocation here, change deserialization?
      // TODO: These attempts should perhaps be throttled as well?
      if request.security_code != account.security_code.to_string() {
        stream = await!(stream.send_packet(&server::CharacterDeleteResult::InvalidSecurityCode))?;
      } else {
        let character = characters.remove(position);
        let result = character_service
          .delete(character)
          .context("Character service failed to delete character")?;

        match result {
          Ok(_) => {
            let success = server::CharacterDeleteResult::Success;
            stream = await!(stream.send_packet(&success))?;
          },
          Err((character, error)) => {
            characters.push(character);
            let failure = map_character_delete_error(error);
            stream = await!(stream.send_packet(&failure))?;
          },
        }
      }
    },
    _ => return Err(Context::new("Invalid client packet; expected character lobby request").into()),
  }

  // Recursively handle lobby activity until a character has been selected
  await!(serve_impl(account, characters, character_service, stream))
}

/// Converts a character creation error to a packet result.
fn map_character_create_error(error: CharacterCreateError) -> server::CharacterCreateResult {
  match error {
    CharacterCreateError::LimitReached => server::CharacterCreateResult::LimitReached,
  }
}

/// Converts a character deletion error to a packet result.
fn map_character_delete_error(error: CharacterDeleteError) -> server::CharacterDeleteResult {
  match error {
    CharacterDeleteError::GuildCharacter => server::CharacterDeleteResult::GuildCharacter,
    CharacterDeleteError::Blocked => server::CharacterDeleteResult::Blocked,
  }
}
