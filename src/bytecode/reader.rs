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

use std::{io, ops::Not};

use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

use crate::{
    constants::*,
    instruction::Instruction,
    proto::{AbsLineInfo, Constant, LocVar, Proto, Upvalue},
};

pub struct Reader<R: AsyncRead + Send + Unpin> {
    buf: BufReader<R>,
}

impl<R: AsyncRead + Send + Unpin> Reader<R> {
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

    pub async fn read_i8(&mut self) -> io::Result<i8> {
        self.buf.read_i8().await
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
        let mut x = 0_usize;
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
        self.read_varint_inner(0_usize.not()).await
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

    async fn read_code(&mut self) -> io::Result<Vec<Instruction>> {
        let n = self.read_i32_varint().await?;
        let mut v = vec![];
        for _ in 0..n {
            v.push(self.read_u32().await?.into());
        }
        Ok(v)
    }

    async fn read_lineinfo(&mut self) -> io::Result<Vec<i8>> {
        let n = self.read_i32_varint().await?;
        let mut v = vec![];
        for _ in 0..n {
            v.push(self.read_i8().await?);
        }
        Ok(v)
    }

    async fn read_abslineinfo(&mut self) -> io::Result<Vec<AbsLineInfo>> {
        let n = self.read_i32_varint().await?;
        let mut v = vec![];
        for _ in 0..n {
            v.push(AbsLineInfo {
                pc: self.read_i32_varint().await?,
                line: self.read_i32_varint().await?,
            });
        }
        Ok(v)
    }

    async fn read_constants(&mut self) -> io::Result<Vec<Constant>> {
        let n = self.read_i32_varint().await?;
        let mut v = vec![];
        for _ in 0..n {
            let tag = self.read_byte().await?;
            let constant = match tag {
                LUA_V_NIL => Constant::Nil,
                LUA_V_FALSE => Constant::Boolean(false),
                LUA_V_TRUE => Constant::Boolean(true),
                LUA_V_NUM_FLT => Constant::Number(self.read_lua_number().await?),
                LUA_V_NUM_INT => Constant::Integer(self.read_lua_integer().await?),
                LUA_V_SHR_STR | LUA_V_LNG_STR => {
                    Constant::String(self.read_string().await?.ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "bad format for constant string")
                    })?)
                }
                tag => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("malformed tag: {}", tag),
                    ))
                }
            };
            v.push(constant)
        }
        Ok(v)
    }

    async fn read_upvalues(&mut self) -> io::Result<Vec<Upvalue>> {
        let n = self.read_i32_varint().await?;
        let mut v = vec![];
        for _ in 0..n {
            v.push(Upvalue {
                name: None,
                instack: self.read_byte().await?,
                idx: self.read_byte().await?,
                kind: self.read_byte().await?,
            })
        }
        Ok(v)
    }

    async fn read_locvars(&mut self) -> io::Result<Vec<LocVar>> {
        let n = self.read_i32_varint().await?;
        let mut v = vec![];
        for _ in 0..n {
            v.push(LocVar {
                varname: self.read_string().await?,
                startpc: self.read_i32_varint().await?,
                endpc: self.read_i32_varint().await?,
            })
        }
        Ok(v)
    }

    #[async_recursion::async_recursion]
    async fn read_protos(&mut self, parent_source: Option<String>) -> io::Result<Vec<Proto>> {
        let n = self.read_i32_varint().await?;
        let mut v = vec![];
        for _ in 0..n {
            v.push(self.read_proto_inner(parent_source.clone()).await?)
        }
        Ok(v)
    }

    pub async fn read_proto(&mut self) -> io::Result<Proto> {
        self.read_proto_inner(None).await
    }

    #[async_recursion::async_recursion]
    async fn read_proto_inner(&mut self, parent_source: Option<String>) -> io::Result<Proto> {
        let source = self.read_string().await?.or(parent_source);
        let linedefined = self.read_i32_varint().await?;
        let lastlinedefined = self.read_i32_varint().await?;
        let numparams = self.read_byte().await?;
        let is_vararg = self.read_byte().await?;
        let maxstacksize = self.read_byte().await?;
        let code = self.read_code().await?;
        let constants = self.read_constants().await?;
        let mut upvalues = self.read_upvalues().await?;
        let protos = self.read_protos(source.clone()).await?;
        let lineinfo = self.read_lineinfo().await?;
        let abslineinfo = self.read_abslineinfo().await?;
        let locvars = self.read_locvars().await?;

        // populate upvalues' name - the order is for historical reason of Lua.
        {
            let n = self.read_i32_varint().await? as usize;
            let len = upvalues.len();
            for i in 0..n {
                let name = self.read_string().await?;
                let upvalue = upvalues.get_mut(i).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("out of range (i: {}, n: {}, len: {})", i, n, len),
                    )
                })?;
                upvalue.name = name;
            }
        }

        Ok(Proto {
            linedefined,
            lastlinedefined,
            numparams,
            is_vararg,
            maxstacksize,
            source,
            code,
            constants,
            upvalues,
            protos,
            lineinfo,
            abslineinfo,
            locvars,
        })
    }
}
