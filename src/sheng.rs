// Sheng DFA was created by authors of Hyperscan. My implementations was
// inspired mainly by Hyperscan and by blog post of Geoff Langdale
// [https://branchfree.org/2018/05/25/say-hello-to-my-little-friend-sheng-a-small-but-fast-deterministic-finite-automaton/]
//
// The idead of using PSHUFB computation of successor states to make parallel
// DFA was taken from Data-Parallel Finite-State Machines paper from
// Microsoft Research [https://dl.acm.org/doi/10.1145/2654822.2541988]

use std::{
    arch::x86_64::{__m128i, _mm_set_epi8, _mm_setzero_si128, _mm_shuffle_epi8},
    num::NonZeroUsize,
    sync::{Arc, mpsc},
    thread,
};

use crate::dfa::TransitionTable;

const MIN_CHUNK_PER_THREAD: usize = 1024;

pub(crate) struct Sheng {
    threads: NonZeroUsize,
    masks: Arc<[__m128i; 256]>,
    inputs: Vec<mpsc::Sender<&'static [u8]>>,
    results: Vec<mpsc::Receiver<__m128i>>,
}

impl Sheng {
    #[cfg(not(feature = "unroll"))]
    #[target_feature(enable = "ssse3")]
    pub unsafe fn run(masks: &[__m128i; 256], mut s: __m128i, input: &[u8]) -> __m128i {
        for &byte in input {
            s = unsafe { _mm_shuffle_epi8(masks[byte as usize], s) };
        }
        s
    }

    #[cfg(feature = "unroll")]
    #[target_feature(enable = "ssse3")]
    pub unsafe fn run(masks: &[__m128i; 256], mut s: __m128i, input: &[u8]) -> __m128i {
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
            s = _mm_shuffle_epi8(masks[b0 as usize], s);
            s = _mm_shuffle_epi8(masks[b1 as usize], s);
            s = _mm_shuffle_epi8(masks[b2 as usize], s);
            s = _mm_shuffle_epi8(masks[b3 as usize], s);
            s = _mm_shuffle_epi8(masks[b4 as usize], s);
            s = _mm_shuffle_epi8(masks[b5 as usize], s);
            s = _mm_shuffle_epi8(masks[b6 as usize], s);
            s = _mm_shuffle_epi8(masks[b7 as usize], s);
            i += 8;
        }
        while i < len {
            s = _mm_shuffle_epi8(masks[input[i] as usize], s);
            i += 1;
        }
        s
    }

    fn run_signle(&self, s: u8, input: &[u8]) -> u8 {
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

        s = unsafe { Self::run(self.masks.as_ref(), s, input) };

        let as_bytes: [u8; 16] = unsafe { std::mem::transmute(s) };
        as_bytes[0]
    }

    fn run_parallel(&self, mut s: u8, input: &'static [u8]) -> u8 {
        let threads: usize = self.threads.into();
        if input.len() / MIN_CHUNK_PER_THREAD < threads {
            return self.run_signle(s, input);
        }
        let chunk_size: usize = input.len() / threads + 1;

        for (i, chunk) in input.chunks(chunk_size).enumerate() {
            self.inputs[i].send(chunk).unwrap();
        }
        for i in 0..threads {
            let res = self.results[i].recv().unwrap();
            unsafe {
                let as_bytes: [u8; 16] = std::mem::transmute(res);
                s = as_bytes[s as usize];
            }
        }

        s
    }

    pub fn new(transitions: &[[u8; 256]; 16], threads: NonZeroUsize) -> Self {
        let zero_m128 = unsafe { _mm_setzero_si128() };
        let mut masks = [zero_m128; 256];
        for c in 0..256 {
            unsafe {
                let mut bytes: [u8; 16] = std::mem::transmute(masks[c]);
                for s in 0..16 {
                    bytes[s] = transitions[s][c];
                }
                masks[c] = std::mem::transmute(bytes);
            }
        }
        let masks = Arc::new(masks);
        let mut inputs = vec![];
        let mut results = vec![];
        if threads.get() > 1 {
            for _ in 0..threads.get() {
                let (input_tx, input_rx) = mpsc::channel();
                let (resut_tx, result_rx) = mpsc::channel();
                inputs.push(input_tx);
                results.push(result_rx);
                let masks = Arc::clone(&masks);
                thread::spawn(move || {
                    for input in input_rx {
                        let mut s = unsafe { _mm_set_epi8(15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0) };
                        s = unsafe { Sheng::run(masks.as_ref(), s, input) };
                        resut_tx.send(s).unwrap();
                    }
                });
            }
        }

        Sheng { threads, masks, inputs, results }
    }
}

impl TransitionTable for Sheng {
    fn run(&self, s: u8, input: &'static [u8]) -> u8 {
        assert!(s < 16, "sheng has only 16 states");
        match self.threads.get() {
            1 => self.run_signle(s, input),
            _ => self.run_parallel(s, input),
        }
    }
}
