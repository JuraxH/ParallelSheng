pub trait TransitionTable {
    fn run(&self, s: u8, input: &'static [u8]) -> u8;
}

pub struct DFA<T: TransitionTable> {
    pub init: u8,
    transitions: T,
}

impl<T: TransitionTable> DFA<T> {
    pub fn new(transitions: T) -> Self {
        Self {
            init: 1,
            transitions,
        }
    }

    pub fn run(&mut self, s: u8, input: &'static [u8]) -> u8 {
        self.transitions.run(s, input)
    }
}

// r'hello.*world'
pub fn example_dfa() -> [[u8; 256]; 16] {
    let mut transitions: [[u8; 256]; 16] = [[0u8; 256]; 16];
    transitions[1]['h' as usize] = 2;
    transitions[2]['e' as usize] = 3;
    transitions[3]['l' as usize] = 4;
    transitions[4]['l' as usize] = 5;
    transitions[5]['o' as usize] = 6;
    for i in 0..=255 {
        transitions[6][i as usize] = 6;
        transitions[7][i as usize] = 6;
        transitions[8][i as usize] = 6;
        transitions[9][i as usize] = 6;
        transitions[10][i as usize] = 6;
        transitions[11][i as usize] = 6;
        if i == 'w' as u8 {
            transitions[5][i as usize] = 7;
            transitions[6][i as usize] = 7;
            transitions[7][i as usize] = 7;
            transitions[8][i as usize] = 7;
            transitions[9][i as usize] = 7;
            transitions[10][i as usize] = 7;
            transitions[11][i as usize] = 7;
        } else if i == 'o' as u8 {
            transitions[7][i as usize] = 8;
        } else if i == 'r' as u8 {
            transitions[8][i as usize] = 9;
        } else if i == 'l' as u8 {
            transitions[9][i as usize] = 10;
        } else if i == 'd' as u8 {
            transitions[10][i as usize] = 11;
        }
    }
    transitions
}

