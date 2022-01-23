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

use tokio::io::AsyncRead;

use crate::proto;

mod reader;

pub async fn parse<R: AsyncRead + Send + Unpin>(reader: R) -> io::Result<proto::Proto> {
    let mut r = reader::Reader::new(reader);
    r.check_header().await?;
    r.read_byte().await?; // sizeupvalues
    r.read_proto().await
}
