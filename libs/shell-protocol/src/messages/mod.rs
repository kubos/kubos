//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use error::ProtocolError;
use serde_cbor::Value;

#[derive(Debug, Eq, PartialEq)]
pub enum Message {
    Spawn(u32, String),
}

pub mod spawn;

pub fn parse_message(message: Value) -> Result<Message, ProtocolError> {
    if let Some(msg) = spawn::from_cbor(message)? {
        return Ok(msg);
    }

    return Err(ProtocolError::MessageParseError {
        err: "No message found".to_owned(),
    });
}
