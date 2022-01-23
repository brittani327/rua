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

/// Literals
pub const ESC_LUA: &[u8; 4] = b"\x1bLua";
pub const LUAC_VERSION: u8 = 0x54;
pub const LUAC_FORMAT: u8 = 0;
pub const LUAC_DATA: &[u8; 6] = b"\x19\x93\r\n\x1a\n";
pub const INSTRUCTION_SIZE: u8 = 4;
pub const LUA_INTEGER_SIZE: u8 = 8;
pub const LUA_NUMBER_SIZE: u8 = 8;
pub const LUAC_INT: i64 = 0x5678;
pub const LUAC_NUM: f64 = 370.5;

pub const fn make_varint(t: u8, v: u8) -> u8 {
    t | (v << 4)
}

/// Basic types
pub const LUA_T_NIL: u8 = 0;
pub const LUA_T_BOOLEAN: u8 = 1;
pub const LUA_T_NUMBER: u8 = 3;
pub const LUA_T_STRING: u8 = 4;

/// Varint tags
pub const LUA_V_NIL: u8 = make_varint(LUA_T_NIL, 0);
pub const LUA_V_FALSE: u8 = make_varint(LUA_T_BOOLEAN, 0);
pub const LUA_V_TRUE: u8 = make_varint(LUA_T_BOOLEAN, 1);
pub const LUA_V_NUM_INT: u8 = make_varint(LUA_T_NUMBER, 0);
pub const LUA_V_NUM_FLT: u8 = make_varint(LUA_T_NUMBER, 1);
pub const LUA_V_SHR_STR: u8 = make_varint(LUA_T_STRING, 0);
pub const LUA_V_LNG_STR: u8 = make_varint(LUA_T_STRING, 1);
