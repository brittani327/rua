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

use crate::instruction::Instruction;

#[derive(Debug)]
pub struct Proto {
    pub(crate) linedefined: i32,
    pub(crate) lastlinedefined: i32,
    pub(crate) numparams: u8,
    pub(crate) is_vararg: u8,
    pub(crate) maxstacksize: u8,
    pub(crate) source: Option<String>,
    pub(crate) code: Vec<Instruction>,
    pub(crate) constants: Vec<Constant>,
    pub(crate) upvalues: Vec<Upvalue>,
    pub(crate) protos: Vec<Proto>,
    pub(crate) lineinfo: Vec<i8>,
    pub(crate) abslineinfo: Vec<AbsLineInfo>,
    pub(crate) locvars: Vec<LocVar>,
}

#[derive(Debug)]
pub enum Constant {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    String(String),
}

#[derive(Debug)]
pub struct Upvalue {
    pub(crate) name: Option<String>,
    pub(crate) instack: u8,
    pub(crate) idx: u8,
    pub(crate) kind: u8,
}

#[derive(Debug)]
pub struct AbsLineInfo {
    pub(crate) pc: i32,
    pub(crate) line: i32,
}

#[derive(Debug)]
pub struct LocVar {
    pub(crate) varname: Option<String>,
    pub(crate) startpc: i32,
    pub(crate) endpc: i32,
}
