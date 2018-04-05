/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use i2c_api::Stream;

use commands::*;

pub struct Eps {
    connection: Box<Stream>,
}

impl Eps {
    pub fn new(connection: Box<Stream>) -> Self {
        Eps { connection }
    }

    pub fn get_board_status(&self) -> Result<Status, String> {
        let data = Status::command();
        let res = self.connection.write(&data);
        Ok(Status::parse(&self.connection.read(2).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;

    struct MockConn;

    impl Stream for MockConn {
        fn write(&self, data: &[u8]) -> Result<(), Error> {
            Ok(())
        }

        fn read(&self, length: usize) -> Result<Vec<u8>, Error> {
            Ok(vec![])
        }
    }

    #[test]
    fn test_board_status() {
        let eps = Eps::new(Box::new(MockConn {}));
        assert_eq!(Ok(Status::default()), eps.get_board_status());
    }
}
