use std::convert::TryInto;

pub trait InstGetter {
    fn length(&self) -> usize;
    fn a(&self, data: &[u8]) -> u64;
    fn b(&self, data: &[u8]) -> u64;
    fn c(&self, data: &[u8]) -> u64;
    fn d(&self, data: &[u8]) -> u64;
    fn e(&self, data: &[u8]) -> u64;
    fn f(&self, data: &[u8]) -> u64;
    fn g(&self, data: &[u8]) -> u64;
    fn h(&self, data: &[u8]) -> u64;
}

fn _b(data: &[u8], n: usize) -> u64 {
    (data[n] as u64) << (n * 8)
}

fn read_l(data: &[u8]) -> u64 {
    (data[0] & 0x0f).into()
}

fn read_h(data: &[u8]) -> u64 {
    (data[0] >> 4).into()
}

fn read_2(data: &[u8]) -> u16 {
    (data[0] as u64 + _b(data, 1)).try_into().unwrap()
}

fn read_4(data: &[u8]) -> u32 {
    (data[0] as u64 + _b(data, 1) + _b(data, 2) + _b(data, 3))
        .try_into()
        .unwrap()
}

fn read_8(data: &[u8]) -> u64 {
    data[0] as u64
        + _b(data, 1)
        + _b(data, 2)
        + _b(data, 3)
        + _b(data, 4)
        + _b(data, 5)
        + _b(data, 6)
        + _b(data, 7)
}

#[cfg(test)]
mod test_read_functions {
    use super::{_b, read_2, read_4, read_8, read_h, read_l};

    #[test]
    fn test_b() {
        let buf = [1, 2, 3];
        assert_eq!(_b(&buf, 0), 1);
        assert_eq!(_b(&buf, 1), 2 << 8);
        assert_eq!(_b(&buf, 2), 3 << 16);
    }

    #[test]
    fn test_read_l_and_read_h() {
        let buf = [0xab, 2];
        assert_eq!(read_l(&buf), 0xb);
        assert_eq!(read_h(&buf), 0xa);
    }

    #[test]
    fn test_read_n() {
        let buf = [0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9];
        assert_eq!(read_2(&buf), 0xf2f1);
        assert_eq!(read_4(&buf), 0xf4f3f2f1);
        assert_eq!(read_8(&buf), 0xf8f7f6f5f4f3f2f1);
    }
}

impl InstGetter for GetterOp00 {
    fn length(&self) -> usize {
        2
    }

    fn a(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00 can't get A");
    }

    fn b(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00 can't get B");
    }

    fn c(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00 can't get C");
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00 can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00 can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00 can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00 can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00 can't get H");
    }
}

impl InstGetter for GetterOpAA {
    fn length(&self) -> usize {
        2
    }

    fn a(&self, data: &[u8]) -> u64 {
        data[1].into()
    }

    fn b(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAA can't get B");
    }

    fn c(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAA can't get C");
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAA can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAA can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAA can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAA can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAA can't get H");
    }
}

impl InstGetter for Getter10t {
    fn length(&self) -> usize {
        2
    }

    fn a(&self, data: &[u8]) -> u64 {
        (data[1] as i8) as u64
    }

    fn b(&self, _data: &[u8]) -> u64 {
        panic!("Getter10t can't get B");
    }

    fn c(&self, _data: &[u8]) -> u64 {
        panic!("Getter10t can't get C");
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("Getter10t can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("Getter10t can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("Getter10t can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("Getter10t can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("Getter10t can't get H");
    }
}

impl InstGetter for GetterOpBA {
    fn length(&self) -> usize {
        2
    }

    fn a(&self, data: &[u8]) -> u64 {
        read_l(&data[1..])
    }

    fn b(&self, data: &[u8]) -> u64 {
        read_h(&data[1..])
    }

    fn c(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpBA can't get C");
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpBA can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpBA can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpBA can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpBA can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpBA can't get H");
    }
}

impl InstGetter for GetterOp00AAAA {
    fn length(&self) -> usize {
        4
    }

    fn a(&self, data: &[u8]) -> u64 {
        read_2(&data[2..]).into()
    }

    fn b(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAA can't get B");
    }

    fn c(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAA can't get C");
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAA can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAA can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAA can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAA can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAA can't get H");
    }
}

impl InstGetter for Getter20t {
    fn length(&self) -> usize {
        4
    }

    fn a(&self, data: &[u8]) -> u64 {
        (read_2(&data[2..]) as i16) as u64
    }

    fn b(&self, _data: &[u8]) -> u64 {
        panic!("Getter20t can't get B");
    }

    fn c(&self, _data: &[u8]) -> u64 {
        panic!("Getter20t can't get C");
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("Getter20t can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("Getter20t can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("Getter20t can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("Getter20t can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("Getter20t can't get H");
    }
}

impl InstGetter for GetterOpAABBBB {
    fn length(&self) -> usize {
        4
    }
    fn a(&self, data: &[u8]) -> u64 {
        data[1].into()
    }

    fn b(&self, data: &[u8]) -> u64 {
        read_2(&data[2..]).into()
    }

    fn c(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBB can't get C");
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBB can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBB can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBB can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBB can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBB can't get H");
    }
}

impl InstGetter for GetterOpAACCBB {
    fn length(&self) -> usize {
        4
    }
    fn a(&self, data: &[u8]) -> u64 {
        data[1].into()
    }

    fn b(&self, data: &[u8]) -> u64 {
        data[3].into()
    }

    fn c(&self, data: &[u8]) -> u64 {
        data[2].into()
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAACCBB can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAACCBB can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAACCBB can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAACCBB can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAACCBB can't get H");
    }
}

impl InstGetter for Getter21t {
    fn length(&self) -> usize {
        4
    }
    fn a(&self, data: &[u8]) -> u64 {
        data[1].into()
    }
    fn b(&self, data: &[u8]) -> u64 {
        (read_2(&data[2..]) as i16) as u64
    }

    fn c(&self, _data: &[u8]) -> u64 {
        panic!("Getter21t can't get C");
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("Getter21t can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("Getter21t can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("Getter21t can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("Getter21t can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("Getter21t can't get H");
    }
}

impl InstGetter for GetterOpBACCCC {
    fn length(&self) -> usize {
        4
    }
    fn a(&self, data: &[u8]) -> u64 {
        read_l(&data[1..])
    }
    fn b(&self, data: &[u8]) -> u64 {
        read_h(&data[1..])
    }
    fn c(&self, data: &[u8]) -> u64 {
        read_2(&data[2..]).into()
    }
    fn d(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpBACCCC can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpBACCCC can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpBACCCC can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpBACCCC can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpBACCCC can't get H");
    }
}

impl InstGetter for Getter22t {
    fn length(&self) -> usize {
        4
    }
    fn a(&self, data: &[u8]) -> u64 {
        read_l(&data[1..])
    }
    fn b(&self, data: &[u8]) -> u64 {
        read_h(&data[1..])
    }
    fn c(&self, data: &[u8]) -> u64 {
        (read_2(&data[2..]) as i16) as u64
    }
    fn d(&self, _data: &[u8]) -> u64 {
        panic!("Getter22t can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("Getter22t can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("Getter22t can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("Getter22t can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("Getter22t can't get H");
    }
}

impl InstGetter for Getter30t {
    fn length(&self) -> usize {
        6
    }
    fn a(&self, data: &[u8]) -> u64 {
        (read_4(&data[2..]) as i32) as u64
    }
    fn b(&self, _data: &[u8]) -> u64 {
        panic!("Getter30t can't get B");
    }

    fn c(&self, _data: &[u8]) -> u64 {
        panic!("Getter30t can't get C");
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("Getter30t can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("Getter30t can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("Getter30t can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("Getter30t can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("Getter30t can't get H");
    }
}

impl InstGetter for GetterOp00AAAAAAAA {
    fn length(&self) -> usize {
        6
    }
    fn a(&self, data: &[u8]) -> u64 {
        read_4(&data[2..]).into()
    }
    fn b(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAAAAAA can't get B");
    }

    fn c(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAAAAAA can't get C");
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAAAAAA can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAAAAAA can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAAAAAA can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAAAAAA can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAAAAAA can't get H");
    }
}

impl InstGetter for GetterOp00AAAABBBB {
    fn length(&self) -> usize {
        6
    }
    fn a(&self, data: &[u8]) -> u64 {
        read_2(&data[2..]).into()
    }
    fn b(&self, data: &[u8]) -> u64 {
        read_2(&data[4..]).into()
    }
    fn c(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAABBBB can't get C");
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAABBBB can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAABBBB can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAABBBB can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAABBBB can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("GetterOp00AAAABBBB can't get H");
    }
}

impl InstGetter for GetterOpAABBBBBBBB {
    fn length(&self) -> usize {
        6
    }
    fn a(&self, data: &[u8]) -> u64 {
        data[1].into()
    }
    fn b(&self, data: &[u8]) -> u64 {
        read_4(&data[2..]).into()
    }
    fn c(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBBBBB can't get C");
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBBBBB can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBBBBB can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBBBBB can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBBBBB can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBBBBB can't get H");
    }
}

impl InstGetter for GetterOpAABBBBCCCC {
    fn length(&self) -> usize {
        6
    }
    fn a(&self, data: &[u8]) -> u64 {
        data[1].into()
    }
    fn b(&self, data: &[u8]) -> u64 {
        read_2(&data[2..]).into()
    }
    fn c(&self, data: &[u8]) -> u64 {
        read_2(&data[4..]).into()
    }
    fn d(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBCCCC can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBCCCC can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBCCCC can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBCCCC can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBCCCC can't get H");
    }
}

impl InstGetter for GetterOpAGBBBBDCFE {
    fn length(&self) -> usize {
        6
    }
    fn a(&self, data: &[u8]) -> u64 {
        read_h(&data[1..])
    }
    fn b(&self, data: &[u8]) -> u64 {
        read_2(&data[2..]).into()
    }
    fn c(&self, data: &[u8]) -> u64 {
        read_l(&data[4..])
    }
    fn d(&self, data: &[u8]) -> u64 {
        read_h(&data[4..])
    }
    fn e(&self, data: &[u8]) -> u64 {
        read_l(&data[5..])
    }
    fn f(&self, data: &[u8]) -> u64 {
        read_h(&data[5..])
    }
    fn g(&self, data: &[u8]) -> u64 {
        read_l(&data[1..])
    }
    fn h(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAGBBBBDCFE can't get H");
    }
}

impl InstGetter for GetterOpAABBBBCCCCHHHH {
    fn length(&self) -> usize {
        8
    }
    fn a(&self, data: &[u8]) -> u64 {
        data[1].into()
    }
    fn b(&self, data: &[u8]) -> u64 {
        read_2(&data[2..]).into()
    }
    fn c(&self, data: &[u8]) -> u64 {
        read_2(&data[4..]).into()
    }
    fn d(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBCCCCHHHH can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBCCCCHHHH can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBCCCCHHHH can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("GetterOpAABBBBCCCCHHHH can't get G");
    }
    fn h(&self, data: &[u8]) -> u64 {
        read_2(&data[6..]).into()
    }
}

impl InstGetter for GetterOpAGBBBBDCFEHHHH {
    fn length(&self) -> usize {
        8
    }
    fn a(&self, data: &[u8]) -> u64 {
        read_h(&data[1..])
    }
    fn b(&self, data: &[u8]) -> u64 {
        read_2(&data[2..]).into()
    }
    fn c(&self, data: &[u8]) -> u64 {
        read_l(&data[4..])
    }
    fn d(&self, data: &[u8]) -> u64 {
        read_h(&data[4..])
    }
    fn e(&self, data: &[u8]) -> u64 {
        read_l(&data[5..])
    }
    fn f(&self, data: &[u8]) -> u64 {
        read_h(&data[5..])
    }
    fn g(&self, data: &[u8]) -> u64 {
        read_l(&data[1..])
    }
    fn h(&self, data: &[u8]) -> u64 {
        read_2(&data[6..]).into()
    }
}

impl InstGetter for GetterOpAABBBBBBBBBBBBBBBB {
    fn length(&self) -> usize {
        10
    }
    fn a(&self, data: &[u8]) -> u64 {
        data[1].into()
    }
    fn b(&self, data: &[u8]) -> u64 {
        read_8(&data[2..])
    }

    fn c(&self, _data: &[u8]) -> u64 {
        panic!("OpAABBBBBBBBBBBBBBBB can't get C");
    }

    fn d(&self, _data: &[u8]) -> u64 {
        panic!("OpAABBBBBBBBBBBBBBBB can't get D");
    }

    fn e(&self, _data: &[u8]) -> u64 {
        panic!("OpAABBBBBBBBBBBBBBBB can't get E");
    }

    fn f(&self, _data: &[u8]) -> u64 {
        panic!("OpAABBBBBBBBBBBBBBBB can't get F");
    }

    fn g(&self, _data: &[u8]) -> u64 {
        panic!("OpAABBBBBBBBBBBBBBBB can't get G");
    }

    fn h(&self, _data: &[u8]) -> u64 {
        panic!("OpAABBBBBBBBBBBBBBBB can't get H");
    }
}

pub struct GetterOp00;
pub struct GetterOpAA;
pub struct Getter10t;
pub struct GetterOpBA;
pub struct GetterOp00AAAA;
pub struct Getter20t;
pub struct GetterOpAABBBB;
pub struct Getter21t;
pub struct GetterOpAACCBB;
pub struct GetterOpBACCCC;
pub struct Getter22t;
pub struct GetterOp00AAAAAAAA;
pub struct Getter30t;
pub struct GetterOp00AAAABBBB;
pub struct GetterOpAABBBBBBBB;
pub struct GetterOpAABBBBCCCC;
pub struct GetterOpAGBBBBDCFE;
pub struct GetterOpAABBBBCCCCHHHH;
pub struct GetterOpAGBBBBDCFEHHHH;
pub struct GetterOpAABBBBBBBBBBBBBBBB;

#[cfg(test)]
mod test_getters {
    use super::*;
    use std::panic;

    macro_rules! test_length {
        ($g:ident, $l:expr) => {
            assert_eq!(($g {}).length(), $l);
        };
    }

    macro_rules! assert_panic {
        ($g:ident, $f:ident, $b:ident) => {{
            let result = panic::catch_unwind(|| {
                let getter = $g {};
                getter.$f(&$b);
            });
            assert!(result.is_err());
        }};
    }

    macro_rules! panics_on {
        ($g:ident, $f:ident) => {{
            let buffer = [
                0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb,
            ];
            assert_panic!($g, $f, buffer);
        }};
    }

    macro_rules! gets_byte_at_pos {
        ($g:ident, $f:ident, $p:expr) => {{
            let buffer = [
                0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb,
            ];
            let getter = $g {};
            assert_eq!(getter.$f(&buffer), buffer[$p] as u64);
        }};
    }

    macro_rules! gets_i8_at_pos {
        ($g:ident, $f:ident, $p:expr) => {{
            let buffer = [
                0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb,
            ];
            let getter = $g {};
            assert_eq!(getter.$f(&buffer), (buffer[$p] as i8) as u64);
        }};
    }

    macro_rules! panics_on_empty_buffer {
        ($g:ident) => {{
            let buffer = [];
            assert_panic!($g, a, buffer);
            assert_panic!($g, b, buffer);
            assert_panic!($g, c, buffer);
            assert_panic!($g, d, buffer);
            assert_panic!($g, e, buffer);
            assert_panic!($g, f, buffer);
            assert_panic!($g, g, buffer);
            assert_panic!($g, h, buffer);
        }};
    }

    #[test]
    fn test_lengths() {
        test_length!(GetterOp00, 2);
        test_length!(GetterOpAA, 2);
        test_length!(Getter10t, 2);
        test_length!(GetterOpBA, 2);
        test_length!(GetterOp00AAAA, 4);
        test_length!(Getter20t, 4);
        test_length!(GetterOpAABBBB, 4);
        test_length!(Getter21t, 4);
        test_length!(GetterOpAACCBB, 4);
        test_length!(GetterOpBACCCC, 4);
        test_length!(Getter22t, 4);
        test_length!(GetterOp00AAAAAAAA, 6);
        test_length!(Getter30t, 6);
        test_length!(GetterOp00AAAABBBB, 6);
        test_length!(GetterOpAABBBBBBBB, 6);
        test_length!(GetterOpAABBBBCCCC, 6);
        test_length!(GetterOpAGBBBBDCFE, 6);
        test_length!(GetterOpAABBBBCCCCHHHH, 8);
        test_length!(GetterOpAGBBBBDCFEHHHH, 8);
        test_length!(GetterOpAABBBBBBBBBBBBBBBB, 10);
    }

    #[test]
    fn test_empty_buffer() {
        panics_on_empty_buffer!(GetterOp00);
        panics_on_empty_buffer!(GetterOpAA);
        panics_on_empty_buffer!(Getter10t);
        panics_on_empty_buffer!(GetterOpBA);
        panics_on_empty_buffer!(GetterOp00AAAA);
        panics_on_empty_buffer!(Getter20t);
        panics_on_empty_buffer!(GetterOpAABBBB);
        panics_on_empty_buffer!(Getter21t);
        panics_on_empty_buffer!(GetterOpAACCBB);
        panics_on_empty_buffer!(GetterOpBACCCC);
        panics_on_empty_buffer!(Getter22t);
        panics_on_empty_buffer!(GetterOp00AAAAAAAA);
        panics_on_empty_buffer!(Getter30t);
        panics_on_empty_buffer!(GetterOp00AAAABBBB);
        panics_on_empty_buffer!(GetterOpAABBBBBBBB);
        panics_on_empty_buffer!(GetterOpAABBBBCCCC);
        panics_on_empty_buffer!(GetterOpAGBBBBDCFE);
        panics_on_empty_buffer!(GetterOpAABBBBCCCCHHHH);
        panics_on_empty_buffer!(GetterOpAGBBBBDCFEHHHH);
        panics_on_empty_buffer!(GetterOpAABBBBBBBBBBBBBBBB);
    }

    #[test]
    fn test_a() {
        panics_on!(GetterOp00, a);
        gets_byte_at_pos!(GetterOpAA, a, 1);
        gets_i8_at_pos!(Getter10t, a, 1);
        //        panics_on!(GetterOpBA, a);
        //        panics_on!(GetterOp00AAAA, a);
        //        panics_on!(Getter20t, a);
        //        panics_on!(GetterOpAABBBB, a);
        //        panics_on!(Getter21t, a);
        //        panics_on!(GetterOpAACCBB, a);
        //        panics_on!(GetterOpBACCCC, a);
        //        panics_on!(Getter22t, a);
        //        panics_on!(GetterOp00AAAAAAAA, a);
        //        panics_on!(Getter30t, a);
        //        panics_on!(GetterOp00AAAABBBB, a);
        //        panics_on!(GetterOpAABBBBBBBB, a);
        //        panics_on!(GetterOpAABBBBCCCC, a);
        //        panics_on!(GetterOpAGBBBBDCFE, a);
        //        panics_on!(GetterOpAABBBBCCCCHHHH, a);
        //        panics_on!(GetterOpAGBBBBDCFEHHHH, a);
        //        panics_on!(GetterOpAABBBBBBBBBBBBBBBB, a);
    }

    #[test]
    fn test_b() {
        panics_on!(GetterOp00, b);
        panics_on!(GetterOpAA, b);
        panics_on!(Getter10t, b);
        //        panics_on!(GetterOpBA, b);
        panics_on!(GetterOp00AAAA, b);
        panics_on!(Getter20t, b);
        //        panics_on!(GetterOpAABBBB, b);
        //        panics_on!(Getter21t, b);
        //        panics_on!(GetterOpAACCBB, b);
        //        panics_on!(GetterOpBACCCC, b);
        //        panics_on!(Getter22t, b);
        panics_on!(GetterOp00AAAAAAAA, b);
        panics_on!(Getter30t, b);
        //        panics_on!(GetterOp00AAAABBBB, b);
        //        panics_on!(GetterOpAABBBBBBBB, b);
        //        panics_on!(GetterOpAABBBBCCCC, b);
        //        panics_on!(GetterOpAGBBBBDCFE, b);
        //        panics_on!(GetterOpAABBBBCCCCHHHH, b);
        //        panics_on!(GetterOpAGBBBBDCFEHHHH, b);
        //        panics_on!(GetterOpAABBBBBBBBBBBBBBBB, b);
    }

    #[test]
    fn test_c() {
        panics_on!(GetterOp00, c);
        panics_on!(GetterOpAA, c);
        panics_on!(Getter10t, c);
        panics_on!(GetterOpBA, c);
        panics_on!(GetterOp00AAAA, c);
        panics_on!(Getter20t, c);
        panics_on!(GetterOpAABBBB, c);
        panics_on!(Getter21t, c);
        //        panics_on!(GetterOpAACCBB, c);
        //        panics_on!(GetterOpBACCCC, c);
        //        panics_on!(Getter22t, c);
        panics_on!(GetterOp00AAAAAAAA, c);
        panics_on!(Getter30t, c);
        panics_on!(GetterOp00AAAABBBB, c);
        panics_on!(GetterOpAABBBBBBBB, c);
        //        panics_on!(GetterOpAABBBBCCCC, c);
        //        panics_on!(GetterOpAGBBBBDCFE, c);
        //        panics_on!(GetterOpAABBBBCCCCHHHH, c);
        //        panics_on!(GetterOpAGBBBBDCFEHHHH, c);
        panics_on!(GetterOpAABBBBBBBBBBBBBBBB, c);
    }

    #[test]
    fn test_d() {
        panics_on!(GetterOp00, d);
        panics_on!(GetterOpAA, d);
        panics_on!(Getter10t, d);
        panics_on!(GetterOpBA, d);
        panics_on!(GetterOp00AAAA, d);
        panics_on!(Getter20t, d);
        panics_on!(GetterOpAABBBB, d);
        panics_on!(Getter21t, d);
        panics_on!(GetterOpAACCBB, d);
        panics_on!(GetterOpBACCCC, d);
        panics_on!(Getter22t, d);
        panics_on!(GetterOp00AAAAAAAA, d);
        panics_on!(Getter30t, d);
        panics_on!(GetterOp00AAAABBBB, d);
        panics_on!(GetterOpAABBBBBBBB, d);
        panics_on!(GetterOpAABBBBCCCC, d);
        //        panics_on!(GetterOpAGBBBBDCFE, d);
        panics_on!(GetterOpAABBBBCCCCHHHH, d);
        //        panics_on!(GetterOpAGBBBBDCFEHHHH, d);
        //        panics_on!(GetterOpAABBBBBBBBBBBBBBBB, d);
    }

    #[test]
    fn test_e() {
        panics_on!(GetterOp00, e);
        panics_on!(GetterOpAA, e);
        panics_on!(Getter10t, e);
        panics_on!(GetterOpBA, e);
        panics_on!(GetterOp00AAAA, e);
        panics_on!(Getter20t, e);
        panics_on!(GetterOpAABBBB, e);
        panics_on!(Getter21t, e);
        panics_on!(GetterOpAACCBB, e);
        panics_on!(GetterOpBACCCC, e);
        panics_on!(Getter22t, e);
        panics_on!(GetterOp00AAAAAAAA, e);
        panics_on!(Getter30t, e);
        panics_on!(GetterOp00AAAABBBB, e);
        panics_on!(GetterOpAABBBBBBBB, e);
        panics_on!(GetterOpAABBBBCCCC, e);
        //        panics_on!(GetterOpAGBBBBDCFE, e);
        panics_on!(GetterOpAABBBBCCCCHHHH, e);
        //        panics_on!(GetterOpAGBBBBDCFEHHHH, e);
        panics_on!(GetterOpAABBBBBBBBBBBBBBBB, e);
    }

    #[test]
    fn test_f() {
        panics_on!(GetterOp00, f);
        panics_on!(GetterOpAA, f);
        panics_on!(Getter10t, f);
        panics_on!(GetterOpBA, f);
        panics_on!(GetterOp00AAAA, f);
        panics_on!(Getter20t, f);
        panics_on!(GetterOpAABBBB, f);
        panics_on!(Getter21t, f);
        panics_on!(GetterOpAACCBB, f);
        panics_on!(GetterOpBACCCC, f);
        panics_on!(Getter22t, f);
        panics_on!(GetterOp00AAAAAAAA, f);
        panics_on!(Getter30t, f);
        panics_on!(GetterOp00AAAABBBB, f);
        panics_on!(GetterOpAABBBBBBBB, f);
        panics_on!(GetterOpAABBBBCCCC, f);
        //        panics_on!(GetterOpAGBBBBDCFE, f);
        panics_on!(GetterOpAABBBBCCCCHHHH, f);
        //        panics_on!(GetterOpAGBBBBDCFEHHHH, f);
        panics_on!(GetterOpAABBBBBBBBBBBBBBBB, f);
    }

    #[test]
    fn test_g() {
        panics_on!(GetterOp00, g);
        panics_on!(GetterOpAA, g);
        panics_on!(Getter10t, g);
        panics_on!(GetterOpBA, g);
        panics_on!(GetterOp00AAAA, g);
        panics_on!(Getter20t, g);
        panics_on!(GetterOpAABBBB, g);
        panics_on!(Getter21t, g);
        panics_on!(GetterOpAACCBB, g);
        panics_on!(GetterOpBACCCC, g);
        panics_on!(Getter22t, g);
        panics_on!(GetterOp00AAAAAAAA, g);
        panics_on!(Getter30t, g);
        panics_on!(GetterOp00AAAABBBB, g);
        panics_on!(GetterOpAABBBBBBBB, g);
        panics_on!(GetterOpAABBBBCCCC, g);
        //        panics_on!(GetterOpAGBBBBDCFE, g);
        panics_on!(GetterOpAABBBBCCCCHHHH, g);
        //        panics_on!(GetterOpAGBBBBDCFEHHHH, g);
        panics_on!(GetterOpAABBBBBBBBBBBBBBBB, g);
    }

    #[test]
    fn test_h() {
        panics_on!(GetterOp00, h);
        panics_on!(GetterOpAA, h);
        panics_on!(Getter10t, h);
        panics_on!(GetterOpBA, h);
        panics_on!(GetterOp00AAAA, h);
        panics_on!(Getter20t, h);
        panics_on!(GetterOpAABBBB, h);
        panics_on!(Getter21t, h);
        panics_on!(GetterOpAACCBB, h);
        panics_on!(GetterOpBACCCC, h);
        panics_on!(Getter22t, h);
        panics_on!(GetterOp00AAAAAAAA, h);
        panics_on!(Getter30t, h);
        panics_on!(GetterOp00AAAABBBB, h);
        panics_on!(GetterOpAABBBBBBBB, h);
        panics_on!(GetterOpAABBBBCCCC, h);
        panics_on!(GetterOpAGBBBBDCFE, h);
        //        panics_on!(GetterOpAABBBBCCCCHHHH, h);
        //        panics_on!(GetterOpAGBBBBDCFEHHHH, h);
        panics_on!(GetterOpAABBBBBBBBBBBBBBBB, h);
    }
}


