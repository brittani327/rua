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

use std::io;

use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

use crate::bytecode::chunk::*;

pub struct Reader<R: AsyncRead + Unpin> {
    buf: BufReader<R>,
}

impl<R: AsyncRead + Unpin> Reader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            buf: BufReader::new(reader),
        }
    }

    pub async fn read_bytes(&mut self, n: usize) -> io::Result<Vec<u8>> {
        let mut v = vec![];
        for _ in 0..n {
            v.push(self.buf.read_u8().await?);
        }
        Ok(v)
    }

    pub async fn read_byte(&mut self) -> io::Result<u8> {
        self.buf.read_u8().await
    }

    pub async fn read_lua_integer(&mut self) -> io::Result<i64> {
        self.buf.read_i64_le().await
    }

    pub async fn read_lua_number(&mut self) -> io::Result<f64> {
        self.buf.read_f64_le().await
    }

    pub async fn check_header(&mut self) -> io::Result<()> {
        assert_eq!(self.read_bytes(4).await?, ESC_LUA);
        assert_eq!(self.read_byte().await?, LUAC_VERSION);
        assert_eq!(self.read_byte().await?, LUAC_FORMAT);
        assert_eq!(self.read_bytes(6).await?, LUAC_DATA);
        assert_eq!(self.read_byte().await?, INSTRUCTION_SIZE);
        assert_eq!(self.read_byte().await?, LUA_INTEGER_SIZE);
        assert_eq!(self.read_byte().await?, LUA_NUMBER_SIZE);
        assert_eq!(self.read_lua_integer().await?, LUAC_INT);
        assert_eq!(self.read_lua_number().await?, LUAC_NUM);
        Ok(())
    }
}
