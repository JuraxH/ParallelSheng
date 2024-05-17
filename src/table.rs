use crate::dfa::TransitionTable;
// clasic dfas using arrays to store the successor states

// successors are grouped by states, the address of successor is calculated
// as: table_offset + state * 256 + symbol
pub(crate) struct Table1 {
    table: [[u8; 256]; 16],
}

// successors are grouped by symbols, the address of successor is calculated
// as: table_offset + symbol * 256 * 16 + state
// this tends to be faster with small dfas, especially when unrolled
// the future symbols are known and the part of address computation
// involving them can be done before the previous state is computed
pub(crate) struct Table2 {
    table: [[u8; 16]; 256],
}

impl Table1 {
    pub fn new(transitions: &[[u8; 256]; 16]) -> Self {
        let mut table = [[0u8; 256]; 16];
        for s in 0..16 {
            for c in 0..256 {
                table[s][c] = transitions[s][c];
            }
        }
        Self { table }
    }
}

impl Table2 {
    pub fn new(transitions: &[[u8; 256]; 16]) -> Self {
        let mut table = [[0u8; 16]; 256];
        for s in 0..16 {
            for c in 0..256 {
                table[c][s] = transitions[s][c];
            }
        }
        Self { table }
    }
}

impl TransitionTable for Table1 {
    #[cfg(not(feature="unroll"))]
    fn run(&self, mut s: u8, input: &[u8]) -> u8 {
        assert!(s < 16);

        for &byte in input {
            s = self.table[s as usize][byte as usize];
        }

        s
    }

    #[cfg(feature="unroll")]
    fn run(&self, mut s: u8, input: &[u8]) -> u8 {
        assert!(s < 16);
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
            s = self.table[s as usize][b0 as usize];
            s = self.table[s as usize][b1 as usize];
            s = self.table[s as usize][b2 as usize];
            s = self.table[s as usize][b3 as usize];
            s = self.table[s as usize][b4 as usize];
            s = self.table[s as usize][b5 as usize];
            s = self.table[s as usize][b6 as usize];
            s = self.table[s as usize][b7 as usize];
            i += 8;
        }

        while i < len {
            s = self.table[s as usize][input[i] as usize];
            i += 1;
        }

        s
    }
}

impl TransitionTable for Table2 {
    #[cfg(not(feature="unroll"))]
    fn run(&self, s: u8, input: &[u8]) -> u8 {
        assert!(s < 16);
        let mut cur = s;

        for &byte in input {
            cur = self.table[byte as usize][cur as usize];
        }

        cur
    }

    #[cfg(feature="unroll")]
    fn run(&self, mut s: u8, input: &[u8]) -> u8 {
        assert!(s < 16);
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
            s = self.table[b0 as usize][s as usize];
            s = self.table[b1 as usize][s as usize];
            s = self.table[b2 as usize][s as usize];
            s = self.table[b3 as usize][s as usize];
            s = self.table[b4 as usize][s as usize];
            s = self.table[b5 as usize][s as usize];
            s = self.table[b6 as usize][s as usize];
            s = self.table[b7 as usize][s as usize];
            i += 8;
        }

        while i < len {
            s = self.table[input[i] as usize][s as usize];
            i += 1;
        }

        s
    }
}

