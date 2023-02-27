use num::{Integer,PrimInt};

use crate::util::units::{mHz, Frequency, Hz};

/// Linear feedback shift register in Galois configuration
pub struct LFSR<T> {
    lfsr: T,
    mask: T,
}

impl Default for LFSR<u32> {
    fn default() -> Self {
        Self {
            lfsr: 0xCAFEBABE,
            mask: 0xA3000000,
        }
    }
}

/// 32-bit linear feedback shift register
impl LFSR<u32> {
    /// Returns the next value
    pub fn next(&mut self) -> u32 {
        let lsb: bool = (self.lfsr & 0x01) != 0;
        self.lfsr >>= 1;
        if lsb {
            self.lfsr ^= self.mask;
        }
        self.lfsr
    }
}

impl Default for LFSR<u16> {
    fn default() -> Self {
        Self {
            lfsr: 0xBABE,
            mask: 0xB400,
        }
    }
}

/// 32-bit linear feedback shift register
impl LFSR<u16> {
    /// Returns the next value
    #[inline]
    pub fn next(&mut self) -> u16 {
        let lsb: bool = (self.lfsr & 0x01) != 0;
        self.lfsr >>= 1;
        if lsb {
            self.lfsr ^= self.mask;
        }
        self.lfsr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lfsr16() {
        let mut lfsr16 = LFSR::<u16>::default();
        let start = lfsr16.next();
        let mut period: u32 = 0;
        while lfsr16.next() != start {
            period += 1;
        }
        assert_eq!(period, 65534);
    }

    #[test]
    #[ignore = "takes very long."]
    fn test_lfsr32() {
        let mut lfsr32 = LFSR::<u32>::default();
        let start = lfsr32.next();
        let mut period: u32 = 0;
        while lfsr32.next() != start {
            period += 1;
        }
        assert_eq!(period, 4294967294);
    }
}

pub struct WhiteNoise<T> {
    seed: T,
    lfsr: LFSR<u32>,
}

impl<T> WhiteNoise<T> {
    fn set_seed(&mut self, seed: T) {
        self.seed = seed;
    }
}

impl<T> WhiteNoise<T> where T: Integer {
    fn new() -> Self {
        let _lfsr = LFSR::<u32>::default();
        let _seed: T = 0xbabe as T;
        let s = Self {
            seed: _seed,
            lfsr: _lfsr,
        };
        s
    }
}

// impl WhiteNoise<i16> {
//     fn new() -> Self {
//         let _lfsr = LFSR::<u32>::default();
//         let _seed = 0xbabe;
//         let s = Self {
//             seed: _seed,
//             lfsr: _lfsr,
//         };
//         s
//     }
// }

// impl WhiteNoise<i32> {
//     fn new() -> Self {
//         let _lfsr = LFSR::<u32>::default();
//         let _seed = 0xc0febabe;
//         let s = Self {
//             seed: _seed,
//             lfsr: _lfsr,
//         };
//         s
//     }
// }

impl Iterator for WhiteNoise<i16> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            i16::MAX
                .overflowing_sub_unsigned((self.lfsr.next() & 0xFFFF) as u16)
                .0,
        )
    }
}

impl Iterator for WhiteNoise<i32> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(i32::MAX.overflowing_sub_unsigned(self.lfsr.next()).0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_white_noise16() {
        white_noise = WhiteNoise
        for i in ..10000 {

        }
    }
}

pub struct PinkNoise<T> {
    lfsr: LFSR<T>,
    msample_rate: mHz,
}

pub struct BitFlipNoise<T> {
    lfsr: LFSR<T>,
    msample_rate: mHz,
}

pub struct CrackleNoise<T> {
    lfsr: LFSR<T>,
    msample_rate: mHz,
}
