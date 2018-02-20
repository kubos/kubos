// This module contains structs and parsers for messages received on
// serial connection.

mod file;
mod file_count;
mod state_of_health;

use nom::IResult;

pub type ParseFn<T> = fn(input: &[u8]) -> IResult<&[u8], T>;

pub use messages::file::File;
pub use messages::file_count::FileCount;
pub use messages::state_of_health::StateOfHealth;
