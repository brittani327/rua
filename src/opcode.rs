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

/// op mode
pub const OP_MODE_ABC: u8 = 0; // iABC
pub const OP_MODE_ABX: u8 = 1; // iABx
pub const OP_MODE_ASBX: u8 = 2; // iAsBx
pub const OP_MODE_AX: u8 = 3; // iAx
pub const OP_MODE_SJ: u8 = 4; // isJ

/// op code
pub const OP_MOVE: u8 = 0x00; // R[A] := R[B]
pub const OP_LOADI: u8 = 0x01; // R[A] := sBx
pub const OP_LOADF: u8 = 0x02; // R[A] := (lua_Number)sBx
pub const OP_LOADK: u8 = 0x03; // R[A] := K[Bx]
pub const OP_LOADKX: u8 = 0x04; // R[A] := K[extra arg]
pub const OP_LOADFALSE: u8 = 0x05; // R[A] := false
pub const OP_LFALSESKIP: u8 = 0x06; // R[A] := false; pc++	(*)
pub const OP_LOADTRUE: u8 = 0x07; // R[A] := true
pub const OP_LOADNIL: u8 = 0x08; // R[A], R[A+1], ..., R[A+B] := nil
pub const OP_GETUPVAL: u8 = 0x09; // R[A] := UpValue[B]
pub const OP_SETUPVAL: u8 = 0x0a; // UpValue[B] := R[A]
pub const OP_GETTABUP: u8 = 0x0b; // R[A] := UpValue[B][K[C]:string]
pub const OP_GETTABLE: u8 = 0x0c; // R[A] := R[B][R[C]]
pub const OP_GETI: u8 = 0x0d; // R[A] := R[B][C]
pub const OP_GETFIELD: u8 = 0x0e; // R[A] := R[B][K[C]:string]
pub const OP_SETTABUP: u8 = 0x0f; // UpValue[A][K[B]:string] := RK(C)
pub const OP_SETTABLE: u8 = 0x10; // R[A][R[B]] := RK(C)
pub const OP_SETI: u8 = 0x11; // R[A][B] := RK(C)
pub const OP_SETFIELD: u8 = 0x12; // R[A][K[B]:string] := RK(C)
pub const OP_NEWTABLE: u8 = 0x13; // R[A] := {}
pub const OP_SELF: u8 = 0x14; // R[A+1] := R[B]; R[A] := R[B][RK(C):string]
pub const OP_ADDI: u8 = 0x15; // R[A] := R[B] + sC
pub const OP_ADDK: u8 = 0x16; // R[A] := R[B] + K[C]:number
pub const OP_SUBK: u8 = 0x17; // R[A] := R[B] - K[C]:number
pub const OP_MULK: u8 = 0x18; // R[A] := R[B] * K[C]:number
pub const OP_MODK: u8 = 0x19; // R[A] := R[B] % K[C]:number
pub const OP_POWK: u8 = 0x1a; // R[A] := R[B] ^ K[C]:number
pub const OP_DIVK: u8 = 0x1b; // R[A] := R[B] / K[C]:number
pub const OP_IDIVK: u8 = 0x1c; // R[A] := R[B] // K[C]:number
pub const OP_BANDK: u8 = 0x1d; // R[A] := R[B] & K[C]:integer
pub const OP_BORK: u8 = 0x1e; // R[A] := R[B] | K[C]:integer
pub const OP_BXORK: u8 = 0x2f; // R[A] := R[B] ~ K[C]:integer
pub const OP_SHRI: u8 = 0x20; // R[A] := R[B] >> sC
pub const OP_SHLI: u8 = 0x21; // R[A] := sC << R[B]
pub const OP_ADD: u8 = 0x22; // R[A] := R[B] + R[C]
pub const OP_SUB: u8 = 0x23; // R[A] := R[B] - R[C]
pub const OP_MUL: u8 = 0x24; // R[A] := R[B] * R[C]
pub const OP_MOD: u8 = 0x25; // R[A] := R[B] % R[C]
pub const OP_POW: u8 = 0x26; // R[A] := R[B] ^ R[C]
pub const OP_DIV: u8 = 0x27; // R[A] := R[B] / R[C]
pub const OP_IDIV: u8 = 0x28; // R[A] := R[B] // R[C]
pub const OP_BAND: u8 = 0x29; // R[A] := R[B] & R[C]
pub const OP_BOR: u8 = 0x2a; // R[A] := R[B] | R[C]
pub const OP_BXOR: u8 = 0x2b; // R[A] := R[B] ~ R[C]
pub const OP_SHL: u8 = 0x2c; // R[A] := R[B] >> R[C]
pub const OP_SHR: u8 = 0x2d; // R[A] := R[B] << R[C]
pub const OP_MMBIN: u8 = 0x2e; // call C metamethod over R[A] and R[B]
pub const OP_MMBINI: u8 = 0x2f; // call C metamethod over R[A] and sB
pub const OP_MMBINK: u8 = 0x30; // call C metamethod over R[A] and K[B]
pub const OP_UNM: u8 = 0x31; // R[A] := -R[B]
pub const OP_BNOT: u8 = 0x32; // R[A] := ~R[B]
pub const OP_NOT: u8 = 0x33; // R[A] := not R[B]
pub const OP_LEN: u8 = 0x34; // R[A] := #R[B] (length operator)
pub const OP_CONCAT: u8 = 0x35; // R[A] := R[A].. ... ..R[A + B - 1]
pub const OP_CLOSE: u8 = 0x36; // close all upvalues >= R[A]
pub const OP_TBC: u8 = 0x37; // mark variable A "to be closed"
pub const OP_JMP: u8 = 0x38; // pc += sJ
pub const OP_EQ: u8 = 0x39; // if ((R[A] == R[B]) ~= k) then pc++
pub const OP_LT: u8 = 0x3a; // if ((R[A] <  R[B]) ~= k) then pc++
pub const OP_LE: u8 = 0x3b; // if ((R[A] <= R[B]) ~= k) then pc++
pub const OP_EQK: u8 = 0x3c; // if ((R[A] == K[B]) ~= k) then pc++
pub const OP_EQI: u8 = 0x3d; // if ((R[A] == sB) ~= k) then pc++
pub const OP_LTI: u8 = 0x3e; // if ((R[A] < sB) ~= k) then pc++
pub const OP_LEI: u8 = 0x3f; // if ((R[A] <= sB) ~= k) then pc++
pub const OP_GTI: u8 = 0x40; // if ((R[A] > sB) ~= k) then pc++
pub const OP_GEI: u8 = 0x41; // if ((R[A] >= sB) ~= k) then pc++
pub const OP_TEST: u8 = 0x42; // if (not R[A] == k) then pc++
pub const OP_TESTSET: u8 = 0x43; // if (not R[B] == k) then pc++ else R[A] := R[B] (*)
pub const OP_CALL: u8 = 0x44; // R[A], ... ,R[A+C-2] := R[A](R[A+1], ... ,R[A+B-1])
pub const OP_TAILCALL: u8 = 0x45; // return R[A](R[A+1], ... ,R[A+B-1])
pub const OP_RETURN: u8 = 0x46; // return R[A], ... ,R[A+B-2]
pub const OP_RETURN0: u8 = 0x47; // return
pub const OP_RETURN1: u8 = 0x48; // return R[A]
pub const OP_FORLOOP: u8 = 0x49; // update counters; if loop continues then pc-=Bx;
pub const OP_FORPREP: u8 = 0x4a; // <check values and prepare counters>; if not to run then pc+=Bx+1;
pub const OP_TFORPREP: u8 = 0x4b; // create upvalue for R[A + 3]; pc+=Bx
pub const OP_TFORCALL: u8 = 0x4c; // R[A+4], ... ,R[A+3+C] := R[A](R[A+1], R[A+2]);
pub const OP_TFORLOOP: u8 = 0x4d; // if R[A+2] ~= nil then { R[A]=R[A+2]; pc -= Bx }
pub const OP_SETLIST: u8 = 0x4e; // R[A][C+i] := R[A+i], 1 <= i <= B
pub const OP_CLOSURE: u8 = 0x4f; // R[A] := closure(KPROTO[Bx])
pub const OP_VARARG: u8 = 0x50; // R[A], R[A+1], ..., R[A+C-2] = vararg
pub const OP_VARARGPREP: u8 = 0x51; // (adjust vararg parameters)
pub const OP_EXTRAARG: u8 = 0x52; // extra (larger) argument for previous opcode

/// order op
pub const OPCODES: &[OpCode] = &[
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "MOVE"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ASBX, "LOADI"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ASBX, "LOADF"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABX, "LOADK"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABX, "LOADKX"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "LOADFALSE"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "LFALSESKIP"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "LOADTRUE"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "LOADNIL"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "GETUPVAL"),
    opcode(0, 0, 0, 0, 0, OP_MODE_ABC, "SETUPVAL"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "GETTABUP"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "GETTABLE"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "GETI"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "GETFIELD"),
    opcode(0, 0, 0, 0, 0, OP_MODE_ABC, "SETTABUP"),
    opcode(0, 0, 0, 0, 0, OP_MODE_ABC, "SETTABLE"),
    opcode(0, 0, 0, 0, 0, OP_MODE_ABC, "SETI"),
    opcode(0, 0, 0, 0, 0, OP_MODE_ABC, "SETFIELD"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "NEWTABLE"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "SELF"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "ADDI"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "ADDK"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "SUBK"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "MULK"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "MODK"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "POWK"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "DIVK"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "IDIVK"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "BANDK"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "BORK"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "BXORK"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "SHRI"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "SHLI"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "ADD"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "SUB"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "MUL"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "MOD"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "POW"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "DIV"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "IDIV"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "BAND"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "BOR"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "BXOR"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "SHL"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "SHR"),
    opcode(1, 0, 0, 0, 0, OP_MODE_ABC, "MMBIN"),
    opcode(1, 0, 0, 0, 0, OP_MODE_ABC, "MMBIN"),
    opcode(1, 0, 0, 0, 0, OP_MODE_ABC, "MMBIN"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "UNM"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "BNOT"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "NOT"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "LEN"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABC, "CONCAT"),
    opcode(0, 0, 0, 0, 0, OP_MODE_ABC, "CLOSE"),
    opcode(0, 0, 0, 0, 0, OP_MODE_ABC, "TBC"),
    opcode(0, 0, 0, 0, 0, OP_MODE_SJ, "JMP"),
    opcode(0, 0, 0, 1, 0, OP_MODE_ABC, "EQ"),
    opcode(0, 0, 0, 1, 0, OP_MODE_ABC, "LT"),
    opcode(0, 0, 0, 1, 0, OP_MODE_ABC, "LE"),
    opcode(0, 0, 0, 1, 0, OP_MODE_ABC, "EQK"),
    opcode(0, 0, 0, 1, 0, OP_MODE_ABC, "EQI"),
    opcode(0, 0, 0, 1, 0, OP_MODE_ABC, "LTI"),
    opcode(0, 0, 0, 1, 0, OP_MODE_ABC, "LEI"),
    opcode(0, 0, 0, 1, 0, OP_MODE_ABC, "GTI"),
    opcode(0, 0, 0, 1, 0, OP_MODE_ABC, "GEI"),
    opcode(0, 0, 0, 1, 0, OP_MODE_ABC, "TEST"),
    opcode(0, 0, 0, 1, 1, OP_MODE_ABC, "TESTSET"),
    opcode(0, 1, 1, 0, 1, OP_MODE_ABC, "CALL"),
    opcode(0, 1, 1, 0, 1, OP_MODE_ABC, "TAILCALL"),
    opcode(0, 0, 1, 0, 0, OP_MODE_ABC, "RETURN"),
    opcode(0, 0, 0, 0, 0, OP_MODE_ABC, "RETURN0"),
    opcode(0, 0, 0, 0, 0, OP_MODE_ABC, "RETURN1"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABX, "FORLOOP"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABX, "FORPREP"),
    opcode(0, 0, 0, 0, 0, OP_MODE_ABX, "TFORPREP"),
    opcode(0, 0, 0, 0, 0, OP_MODE_ABC, "TFORCALL"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABX, "TFORLOOP"),
    opcode(0, 0, 1, 0, 0, OP_MODE_ABC, "SETLIST"),
    opcode(0, 0, 0, 0, 1, OP_MODE_ABX, "CLOSURE"),
    opcode(0, 1, 0, 0, 1, OP_MODE_ABC, "VARARG"),
    opcode(0, 0, 1, 0, 1, OP_MODE_ABC, "VARARGPREP"),
    opcode(0, 0, 0, 0, 0, OP_MODE_AX, "EXTRAARG"),
];

pub const fn opcode(mm: u8, ot: u8, it: u8, t: u8, a: u8, mode: u8, name: &'static str) -> OpCode {
    OpCode {
        mm: mm != 1,
        ot: ot != 1,
        it: it != 1,
        t: t != 1,
        a: a != 1,
        mode,
        name,
    }
}

pub struct OpCode {
    mm: bool,
    ot: bool,
    it: bool,
    t: bool,
    a: bool,
    mode: u8,
    name: &'static str,
}

impl OpCode {
    pub fn mm(&self) -> bool {
        self.mm
    }

    pub fn ot(&self) -> bool {
        self.ot
    }

    pub fn it(&self) -> bool {
        self.it
    }

    pub fn t(&self) -> bool {
        self.t
    }

    pub fn a(&self) -> bool {
        self.a
    }

    pub fn mode(&self) -> u8 {
        self.mode
    }

    pub fn name(&self) -> &str {
        self.name
    }
}
