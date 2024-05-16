use std::time::Instant;

use dfa::TransitionTable;

use crate::{dfa::example_dfa, sheng::Sheng, table::{Table1, Table2}};
use rand::Rng;

mod dfa;
mod sheng;
mod table;

pub fn run_bench() {
    let input = generate_input(16_384);
    let repeats = 100_000;
    println!("Runing the benchmarks:");
    bench_dfa::<Sheng>("Sheng", input.as_slice(), repeats); 
    bench_dfa::<Table1>("DFA1 [state][symbol]:", input.as_slice(), repeats); 
    bench_dfa::<Table2>("DFA2 [symbol][state]:", input.as_slice(), repeats); 
}

fn generate_input(size: usize) -> Vec<u8> {
    let rng = rand::thread_rng;
    let mut bytes = vec![0u8; size];
    rng().fill(&mut bytes[..]);
    bytes
}

fn bench_dfa<T: TransitionTable>(name: &str, input: &[u8], reps: usize) {
    let dfa = example_dfa::<T>();
    let mut state = dfa.init;
    let start = Instant::now();
    for _ in 0..reps {
        state = dfa.run(state, input);
    }
    let end = Instant::now();
    let time = end - start;
    let bytes_scanned = input.len() * reps;
    let byte_per_ns = bytes_scanned as f64 / time.as_nanos() as f64;
    println!("\n=== {name} ===");
    println!("final state: {state}");
    println!("bytes scanned: {bytes_scanned}");
    println!("miliseconds: {}", time.as_millis());
    println!("bytes per ns: {byte_per_ns}");
}

#[cfg(test)]
mod test {
    use crate::dfa::{TransitionTable, DFA};
    use crate::sheng::Sheng;
    use crate::table::{Table1, Table2};

    fn ab<T: TransitionTable>() {
        // build
        let mut dfa = DFA::<T>::new();
        dfa.set_succ(1, 'a' as u8, 2);
        dfa.set_succ(2, 'b' as u8, 1);

        assert!(dfa.run(dfa.init, "abababab".as_bytes()) == 1);
        assert!(dfa.run(dfa.init, "abababa".as_bytes()) == 2);
        assert!(dfa.run(dfa.init, "abababb".as_bytes()) == 0);
    }

    #[test]
    fn ab_test() {
        ab::<Sheng>();
        ab::<Table1>();
        ab::<Table2>();
    }
}
