#![allow(non_snake_case)]

pub mod wrappers;

extern crate libc;

use std::fmt;
use std::str::SplitWhitespace;
use big::wrappers::*;
use fp2::wrappers::*;
use randapi::wrappers::*;
use ecp2::wrappers::*;

impl ECP2 {
    pub fn set(P: &mut ECP2, x: &FP2, y: &FP2) {
        unsafe {
            ECP2_set(P, x, y);
        }
    }

    pub fn mul(P: &mut ECP2, e: &BIG) {
        unsafe {
            ECP2_mul(P, e);
        }
    }

    pub fn add(P: &mut ECP2, Q: &ECP2) {
        unsafe {
            ECP2_add(P, Q);
        }
    }

    pub fn sub(P: &mut ECP2, Q:&ECP2) {
        unsafe {
            ECP2_sub(P, Q);
        }
    }

    pub fn toOctet(W: &mut octet, P: &ECP2) {
        unsafe {
            ECP2_toOctet(W, P);
        }
    }

    pub fn fromOctet(W: &octet) -> ECP2 {
        let mut ret: ECP2 = ECP2::default();
        unsafe {
            ECP2_fromOctet(&mut ret, W);
        }
        return ret;
    }


    pub fn inf(P: &mut ECP2) {
        unsafe {
            ECP2_inf(P);
        }
    }

    pub fn new_fp2s(x: FP2, y: FP2, z: FP2) -> ECP2 {
        ECP2 {
            inf: 0,
            x: x,
            y: y,
            z: z
        }
    }

    pub fn to_hex(&self) -> String {
        let mut ret: String = String::with_capacity(7 * BIG_HEX_STRING_LEN);
        ret.push_str(&format!("{} {} {} {}", self.inf, self.x.to_hex(), self.y.to_hex(), self.z.to_hex()));
        return ret;
    }

    pub fn from_hex_iter(iter: &mut SplitWhitespace) -> ECP2 {
        let mut ret:ECP2 = ECP2::default();
        if let Some(x) = iter.next() {
            ret.inf = i32::from_str_radix(x, 16).unwrap();
            ret.x = FP2::from_hex_iter(iter);
            ret.y = FP2::from_hex_iter(iter);
            ret.z = FP2::from_hex_iter(iter);
        }
        return ret;
    }

    pub fn from_hex(val: String) -> ECP2 {
        let mut iter = val.split_whitespace();
        return ECP2::from_hex_iter(&mut iter);
    }
}

impl PartialEq for ECP2 {
    fn eq(&self, other: &ECP2) -> bool {
        return (self.inf == other.inf) &&
            (self.x == other.x) &&
            (self.y == other.y) &&
            (self.z == other.z);
    }
}

impl Copy for ECP2 { }

impl Clone for ECP2 {
    fn clone(&self) -> ECP2 {
        ECP2 {
            inf: self.inf,
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
}

impl fmt::Display for ECP2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ECP2: [ {}, {}, {}, {} ]", self.inf, self.x, self.y, self.z)
    }
}

impl fmt::Debug for ECP2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ECP2: [ {}, {}, {}, {} ]", self.inf, self.x, self.y, self.z)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecp2_hex_io() {
        let m = ECP2::from_hex(String::from("0 \
                                             1 0 0 0 0 0 0 0 0 0 \
                                             2 0 0 0 0 0 0 0 0 0 \
                                             3 0 0 0 0 0 0 0 0 0"));
        let s = m.to_hex();
        let r = ECP2::from_hex(s.clone());
        println!("ecp2_hex_io=s:{},m:{},r:{}", s, m, r);
        assert_eq!(m, r);
    }
}
