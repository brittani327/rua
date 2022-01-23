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

use std::{future::Future, io, ops::Not};

use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

use crate::{constants::*, proto};

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

    pub async fn read_u32(&mut self) -> io::Result<u32> {
        self.buf.read_u32_le().await
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

    async fn read_varint_inner(&mut self, limit: usize) -> io::Result<usize> {
        let mut x = 0 as usize;
        let limit = limit >> 7;
        while {
            let b = self.read_byte().await?;
            if x >= limit {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "integer overflow",
                ));
            }
            x = (x << 7) | ((b & 0x7F) as usize);
            (b & 0x80) == 0
        } {}
        Ok(x)
    }

    async fn read_usize_varint(&mut self) -> io::Result<usize> {
        self.read_varint_inner((0 as usize).not()).await
    }

    async fn read_i32_varint(&mut self) -> io::Result<i32> {
        let i = self.read_varint_inner(i32::MAX as usize).await?;
        Ok(i as i32)
    }

    async fn read_string(&mut self) -> io::Result<Option<String>> {
        let size = self.read_usize_varint().await?;
        Ok(if size == 0 {
            None
        } else {
            let bytes = self.read_bytes(size - 1).await?;
            String::from_utf8(bytes).ok()
        })
    }

    async fn read_vec<T, F, Fut>(&mut self, f: F) -> io::Result<Vec<T>>
    where
        F: Fn(&mut Self) -> Fut,
        Fut: Future<Output = io::Result<T>>,
    {
        let size = self.read_i32_varint().await?;
        let mut v = vec![];
        for _ in 0..size {
            v.push(f(self).await?);
        }
        Ok(v)
    }

    pub async fn read_proto(&mut self) -> io::Result<proto::Proto> {
        self.read_proto_inner(None).await
    }

    async fn read_proto_inner(
        &mut self,
        parent_source: Option<String>,
    ) -> io::Result<proto::Proto> {
        let source = self.read_string().await?.or(parent_source);
        let linedefined = self.read_i32_varint().await?;
        let lastlinedefined = self.read_i32_varint().await?;
        let numparams = self.read_byte().await?;
        let is_vararg = self.read_byte().await?;
        let maxstacksize = self.read_byte().await?;
        let code = self.read_vec(|r| r.read_u32()).await?;
        Ok(proto::Proto {
            linedefined,
            lastlinedefined,
            numparams,
            is_vararg,
            maxstacksize,
            source,
            code,
        })
    }
}
