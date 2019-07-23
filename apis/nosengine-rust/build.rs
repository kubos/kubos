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
// Contributed by: Timothy Scott (tmscott@mix.wvu.edu)
//

extern crate cmake;

fn main() {
    // This runs the CMake file in nos3_c_interface.
    // See the comments in that file for an explanation of why it's necessary.
    // The important thing is that it doesn't actually build anything, so all I
    // have to do is run this one command.
    cmake::build("nosengine_c_interface");
}
