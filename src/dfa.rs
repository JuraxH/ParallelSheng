pub trait TransitionTable {
    fn new(defaut_state: u8) -> Self;
    fn set_succ(&mut self, src: u8, symbol: u8, dst: u8);
    fn run(&self, s: u8, input: &[u8]) -> u8;
}

pub struct DFA<T: TransitionTable> {
    pub init: u8,
    transitions: T,
}

impl<T: TransitionTable> DFA<T> {
    pub fn new() -> Self {
        Self {
            init: 1,
            transitions: T::new(0),
        }
    }

    pub fn run(&self, s: u8, input: &[u8]) -> u8 {
        self.transitions.run(s, input)
    }

    pub fn set_succ(&mut self, src: u8, symbol: u8, dst: u8) {
        self.transitions.set_succ(src, symbol, dst);
    }
}

// r'hello.*world'
pub fn example_dfa<T: TransitionTable>() -> DFA<T> {
    let mut dfa = DFA::<T>::new();
    dfa.set_succ(1, 'h' as u8, 2);
    dfa.set_succ(2, 'e' as u8, 3);
    dfa.set_succ(3, 'l' as u8, 4);
    dfa.set_succ(4, 'l' as u8, 5);
    dfa.set_succ(5, 'o' as u8, 6);
    for i in 0..=255 {
        dfa.set_succ(6, i as u8, 6);
        dfa.set_succ(7, i as u8, 6);
        dfa.set_succ(8, i as u8, 6);
        dfa.set_succ(9, i as u8, 6);
        dfa.set_succ(10, i as u8, 6);
        dfa.set_succ(11, i as u8, 6);
        if i == 'w' as u8 {
            dfa.set_succ(6, i as u8, 7);
            dfa.set_succ(7, i as u8, 7);
            dfa.set_succ(8, i as u8, 7);
            dfa.set_succ(9, i as u8, 7);
            dfa.set_succ(10, i as u8, 7);
            dfa.set_succ(11, i as u8, 7);
        } else if i == 'o' as u8 {
            dfa.set_succ(7, i as u8, 8);
        } else if i == 'r' as u8 {
            dfa.set_succ(8, i as u8, 9);
        } else if i == 'l' as u8 {
            dfa.set_succ(9, i as u8, 10);
        } else if i == 'd' as u8 {
            dfa.set_succ(10, i as u8, 11);
        }
    }
    dfa
}

