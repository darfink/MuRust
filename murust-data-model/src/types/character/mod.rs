pub use self::class::Class;
pub use self::guild::GuildRole;
pub use self::hero::HeroStatus;
use std::ops::Range;

mod class;
mod guild;
mod hero;

/// The range of slots availabe for a character.
pub const CHARACTER_SLOTS: Range<usize> = 0..5;
