//
// Copyright (C) 2019 Kubos Corporation
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

#[derive(Clone, Debug, PartialEq)]
pub struct MonitStatus {
    status: String
}

impl MonitStatus {
    /// Create an empty MemInfo object
    pub fn get() -> Self {
        let status = if let Ok(output) = Command::new("monit").arg("status").output() {
            if output.stderr.is_empty() {
                output.stdout
            } else {
                output.stderr
            };
        } else {
            error!("Failed to get current disk usage info");
            "Failed to call 'monit status'"
        };

        Self {
            status
        }
    }
}
