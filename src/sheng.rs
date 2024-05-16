use std::arch::x86_64::{__m128i, _mm_setzero_si128, _mm_shuffle_epi8, _mm_set_epi8};

use crate::dfa::TransitionTable;

pub(crate) struct Sheng {
    masks: [__m128i; 256],
}

impl Sheng {
    #[cfg(not(feature="unroll"))]
    #[target_feature(enable = "ssse3")]
    pub unsafe fn run(&self, mut s: __m128i, input: &[u8]) -> __m128i {
        for &byte in input {
            s = unsafe { _mm_shuffle_epi8(self.masks[byte as usize], s) };
        }
        s
    }

    #[cfg(feature="unroll")]
    #[target_feature(enable = "ssse3")]
    pub unsafe fn run(&self, mut s: __m128i, input: &[u8]) -> __m128i {
        let len = input.len();
        let mut i = 0;
        while i + 7 < len {
            let b0 = input[i];
            let b1 = input[i + 1];
            let b2 = input[i + 2];
            let b3 = input[i + 3];
            let b4 = input[i + 4];
            let b5 = input[i + 5];
            let b6 = input[i + 6];
            let b7 = input[i + 7];
            s = _mm_shuffle_epi8(self.masks[b0 as usize], s);
            s = _mm_shuffle_epi8(self.masks[b1 as usize], s);
            s = _mm_shuffle_epi8(self.masks[b2 as usize], s);
            s = _mm_shuffle_epi8(self.masks[b3 as usize], s);
            s = _mm_shuffle_epi8(self.masks[b4 as usize], s);
            s = _mm_shuffle_epi8(self.masks[b5 as usize], s);
            s = _mm_shuffle_epi8(self.masks[b6 as usize], s);
            s = _mm_shuffle_epi8(self.masks[b7 as usize], s);
            i += 8;
        }
        while i < len {
            s = _mm_shuffle_epi8(self.masks[input[i] as usize], s);
            i += 1;
        }
        s
    }
}

impl TransitionTable for Sheng {
    fn new(defaut_state: u8) -> Self {
        assert!(defaut_state < 16, "sheng has only 16 states");
        let zero_m128 = unsafe { _mm_setzero_si128() };
        let masks = [zero_m128; 256];

        Sheng { masks }
    }

    fn set_succ(&mut self, src: u8, symbol: u8, dst: u8) {
        assert!(src < 16, "sheng has only 16 states");
        assert!(dst < 16, "sheng has only 16 states");

        unsafe {
            let mut bytes: [u8; 16] = std::mem::transmute(self.masks[symbol as usize]);
            bytes[src as usize] = dst;
            self.masks[symbol as usize] = std::mem::transmute(bytes);
        }
    }

    fn run(&self, s: u8, input: &[u8]) -> u8 {
        assert!(s < 16, "sheng has only 16 states");
        let mut s = unsafe {
            _mm_set_epi8(
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                s.try_into().unwrap(),
            )
        };

        s = unsafe { self.run(s, input) };

        let as_bytes: [u8; 16] = unsafe { std::mem::transmute(s) };
        as_bytes[0]
    }
}
