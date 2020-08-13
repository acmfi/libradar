// use std::convert::TryFrom;
use dex::code::CodeItem;
use std::convert::TryInto;
use std::fmt;

mod getters;
#[allow(dead_code)]
mod opcodes;

use crate::disass::getters::*;
use crate::disass::opcodes::*;

struct InstType {
    pub mnemonic: &'static str,
    pub syntax: &'static str,
    pub get: &'static dyn InstGetter,
}

const INSTTYPES: [InstType; 256] = include!("insn.in");

pub struct Inst<'a> {
    bytes: &'a [u8],
}

macro_rules! table {
    ($s:expr) => {
        &INSTTYPES[$s.op()].get
    };
}

impl Inst<'_> {
    pub fn op(&self) -> usize {
        self.bytes[0].into()
    }

    pub fn length(&self) -> usize {
        if self.op() == 0 {
            if self.bytes[1] == 0 {
                return 2;
            } else if self.bytes[1] == 1 {
                // packed-switch-payload
                let size: usize = (((self.bytes[3] as usize) << 8) + (self.bytes[2] as usize))
                    .try_into()
                    .unwrap();
                return 8 + 4 * size;
            } else if self.bytes[1] == 2 {
                // sparse-switch-payload
                let size: usize = (((self.bytes[3] as usize) << 8) + (self.bytes[2] as usize))
                    .try_into()
                    .unwrap();
                return 4 + 8 * size;
            } else if self.bytes[1] == 3 {
                // fill-array-data-payload
                let w = ((self.bytes[3] as usize) << 8) + (self.bytes[2] as usize);
                let n = ((self.bytes[7] as usize) << 24)
                    + ((self.bytes[6] as usize) << 16)
                    + ((self.bytes[5] as usize) << 8)
                    + (self.bytes[4] as usize);
                let mut len: usize = (8 + w * n).try_into().unwrap();
                if len % 2 == 1 {
                    len += 1;
                }
                return len;
            }

            panic!("Unexpected NOP type {:x}", self.bytes[1]);
        }

        return table!(self).length();
    }

    pub fn get_a(&self) -> u64 {
        table!(self).a(self.bytes)
    }

    pub fn get_b(&self) -> u64 {
        table!(self).b(self.bytes)
    }

    pub fn get_c(&self) -> u64 {
        table!(self).c(self.bytes)
    }

    pub fn get_d(&self) -> u64 {
        table!(self).d(self.bytes)
    }

    pub fn get_e(&self) -> u64 {
        table!(self).e(self.bytes)
    }

    pub fn get_f(&self) -> u64 {
        table!(self).f(self.bytes)
    }

    pub fn get_g(&self) -> u64 {
        table!(self).g(self.bytes)
    }

    pub fn get_h(&self) -> u64 {
        table!(self).h(self.bytes)
    }

    pub fn is_const(&self) -> bool {
        CONST4 <= self.op() && self.op() <= CONSTCLASS
    }

    pub fn is_const_string(&self) -> bool {
        self.op() == CONSTSTRING || self.op() == CONSTSTRINGJUMBO
    }

    pub fn is_invoke(&self) -> bool {
        if INVOKEVIRTUAL <= self.op() && self.op() <= INVOKEINTERFACE {
            return true;
        }
        if INVOKEVIRTUAL_RANGE <= self.op() && self.op() <= INVOKEINTERFACE_RANGE {
            return true;
        }
        if self.op() == INVOKEPOLYMORPHIC || self.op() == INVOKEPOLYMORPHIC_RANGE {
            return true;
        }
        return false;
    }

    pub fn is_read_field(&self) -> bool {
        if IGET <= self.op() && self.op() <= IGETSHORT {
            return true;
        }
        if SGET <= self.op() && self.op() <= SGETSHORT {
            return true;
        }
        return false;
    }

    pub fn is_return(&self) -> bool {
        RETURNVOID <= self.op() && self.op() <= RETURNOBJECT
    }

    pub fn is_throw(&self) -> bool {
        self.op() == THROW
    }

    pub fn is_goto(&self) -> bool {
        GOTO <= self.op() && self.op() <= GOTO_32
    }

    pub fn is_branch(&self) -> bool {
        IFEQ <= self.op() && self.op() <= IFLEZ
    }

    pub fn is_switch(&self) -> bool {
        self.op() == PACKEDSWITCH || self.op() == SPARSESWITCH
    }

    pub fn string_idx(&self) -> i32 {
        self.get_b() as i32
    }

    pub fn invoke_target(&self) -> i32 {
        self.get_b() as i32
    }

    pub fn field(&self) -> i32 {
        if self.op() < SGET {
            self.get_c().try_into().unwrap()
        } else {
            self.get_b().try_into().unwrap()
        }
    }

    pub fn mnemonic(&self) -> &str {
        INSTTYPES[self.op()].mnemonic
    }
}

impl fmt::Debug for Inst<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "lenght={} data={:?}",
            self.length(),
            &self.bytes[..self.length()]
        ))
    }
}

impl fmt::Display for Inst<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", INSTTYPES[self.op()].mnemonic)
    }
}

pub struct InstIterator<'a> {
    pub bytes: &'a [u8],
    index: usize,
    length: usize,
}

impl InstIterator<'_> {
    pub fn new(bytes: &[u8], length: usize) -> InstIterator {
        InstIterator {
            bytes: bytes,
            index: 0,
            length: length,
        }
    }
}

impl<'a> Iterator for InstIterator<'a> {
    type Item = Inst<'a>;

    fn next(&mut self) -> Option<Inst<'a>> {
        if self.index < self.length {
            let i = Inst {
                bytes: &self.bytes[self.index..],
            };
            self.index += i.length();
            Some(i)
        } else {
            None
        }
    }
}

pub fn disassemble<'a>(code: &'a CodeItem) -> InstIterator<'a> {
    let (_, data, _) = unsafe { (code.insns()).align_to::<u8>() };
    InstIterator::new(data, code.insns().len() * 2)
}

#[cfg(test)]
mod test_inst {
    use super::Inst;

    #[test]
    fn test_op() {
        let buf = [0, 0];
        let i = Inst { bytes: &buf };
        assert_eq!(i.op(), buf[0] as usize);
    }
}
