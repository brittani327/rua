// Copyright 2022 tison <wander4096@gmail.com>.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod bytecode;
#[allow(dead_code)]
mod closure;
mod constants;
#[allow(dead_code)]
mod instruction;
#[allow(dead_code)]
pub mod opcode;
#[allow(dead_code)]
mod proto;

pub use bytecode::undump;
pub use closure::Closure;
