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

//   We assume that instructions are unsigned 32-bit integers.
//   All instructions have an opcode in the first 7 bits.
//   Instructions can have the following formats:
//
//         3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0
//         1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// iABC          C(8)     |      B(8)     |k|     A(8)      |   Op(7)     |
// iABx                Bx(17)               |     A(8)      |   Op(7)     |
// iAsBx              sBx (signed)(17)      |     A(8)      |   Op(7)     |
// iAx                           Ax(25)                     |   Op(7)     |
// isJ                           sJ(25)                     |   Op(7)     |
//
//   A signed argument is represented in excess K: the represented value is
//   the written unsigned value minus K, where K is half the maximum for the
//   corresponding unsigned argument.

use crate::{opcode, opcode::OPCODES};

const MAXARG_BX: isize = (1 << 16) - 1; // 65535
const MAXARG_SBX: isize = MAXARG_BX >> 1; // 32767

pub trait Instruction {
    fn opname(self) -> &'static str;
    fn opmode(self) -> u8;
    fn mm_mode(self) -> bool;
    fn ot_mode(self) -> bool;
    fn it_mode(self) -> bool;
    fn t_mode(self) -> bool;
    fn a_mode(self) -> bool;
    fn opcode(self) -> u8;
    fn abc(self) -> (isize, isize, isize, isize);
    fn a_bx(self) -> (isize, isize);
    fn a_sbx(self) -> (isize, isize);
    fn ax(self) -> isize;
    fn sj(self) -> isize;
    fn execute(self);
}

impl Instruction for u32 {
    fn opname(self) -> &'static str {
        OPCODES[self.opcode() as usize].name()
    }

    fn opmode(self) -> u8 {
        OPCODES[self.opcode() as usize].mode()
    }

    fn mm_mode(self) -> bool {
        OPCODES[self.opcode() as usize].mm()
    }

    fn ot_mode(self) -> bool {
        OPCODES[self.opcode() as usize].ot()
    }

    fn it_mode(self) -> bool {
        OPCODES[self.opcode() as usize].it()
    }

    fn t_mode(self) -> bool {
        OPCODES[self.opcode() as usize].t()
    }

    fn a_mode(self) -> bool {
        OPCODES[self.opcode() as usize].a()
    }

    fn opcode(self) -> u8 {
        self as u8 & 0x7F
    }

    fn abc(self) -> (isize, isize, isize, isize) {
        let a = (self >> 7 & 0xFF) as isize;
        let k = (self >> 15 & 0x01) as isize;
        let b = (self >> 16 & 0xFF) as isize;
        let c = (self >> 24) as isize;
        (a, k, b, c)
    }

    fn a_bx(self) -> (isize, isize) {
        let a = (self >> 7 & 0xFF) as isize;
        let bx = (self >> 15) as isize;
        (a, bx)
    }

    fn a_sbx(self) -> (isize, isize) {
        let (a, bx) = self.a_bx();
        (a, bx - MAXARG_SBX)
    }

    fn ax(self) -> isize {
        (self >> 7) as isize
    }

    fn sj(self) -> isize {
        (self >> 7) as isize
    }

    fn execute(self) {
        match self.opmode() {
            opcode::OP_MODE_ABC => println!("{:?} {:?}", self.opname(), self.abc()),
            opcode::OP_MODE_ABX => println!("{:?} {:?}", self.opname(), self.a_bx()),
            opcode::OP_MODE_ASBX => println!("{:?} {:?}", self.opname(), self.a_sbx()),
            opcode::OP_MODE_AX => println!("{:?} {:?}", self.opname(), self.ax()),
            opcode::OP_MODE_SJ => println!("{:?} {:?}", self.opname(), self.sj()),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::instruction::Instruction;

    #[test]
    pub fn test() {
        let ops: Vec<u32> = vec![81, 11, 32899, 16908356, 16842822];
        for op in ops {
            op.execute();
        }
    }
}
