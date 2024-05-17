use std::num::NonZeroUsize;
use std::time::{Instant, Duration};
use std::vec;

use dfa::{TransitionTable, DFA};

use crate::{dfa::example_dfa, sheng::Sheng, table::{Table1, Table2}};
use rand::Rng;

mod dfa;
mod sheng;
mod table;

struct BenchResult {
    block_size: usize,
    bytes_scanned: usize,
    dfa_times_ms: Vec<Duration>,
}

struct Bench {
    dfas: Vec<String>,
    results: Vec<BenchResult>,
}

pub fn run_bench() {
    let bytes_to_scan = 2usize.pow(31u32);
    let input = Box::leak(generate_input(bytes_to_scan).into_boxed_slice());
    let min_block_size = 1024;
    let max_threads = 4;
    let transitions = example_dfa();
    let mut shengs = vec![];
    let mut bench = Bench {
        dfas: vec![String::from("Table1"), String::from("Table2")],
        results: vec![]
    };
    for i in 0..max_threads {
        shengs.push(DFA::new(Sheng::new(&transitions, NonZeroUsize::new(i + 1).unwrap())));
        bench.dfas.push(format!("Sheng{}", i + 1));
    }
    let mut table1 = DFA::new(Table1::new(&transitions));
    let mut table2 = DFA::new(Table2::new(&transitions));

    eprintln!("Runing the benchmarks:");
    let mut block_size = min_block_size;
    while block_size < input.len() {
        eprintln!("block size: {block_size}");
        let mut result = BenchResult {
            block_size,
            bytes_scanned: input.len(),
            dfa_times_ms: vec![]
        };
        let chunks: Vec<_> = input.chunks(block_size).collect();
        result.dfa_times_ms.push(bench_dfa(&chunks, &mut table1)); 
        result.dfa_times_ms.push(bench_dfa(&chunks, &mut table2)); 
        for sheng in shengs.iter_mut() {
            result.dfa_times_ms.push(bench_dfa(&chunks, sheng));
        }
        bench.results.push(result);
        block_size *= 2;
    }
    print_results(bench);
}

fn print_results(bench: Bench) {
    // header
    print!("bytes_scanned;block_size");
    for dfa in bench.dfas {
        print!(";{}_ms", &dfa);
        print!(";{}_bytes_per_ns", &dfa);
    }
    print!("\n");
    // body
    for res in bench.results {
        print!("{}", res.bytes_scanned);
        print!(";{}", res.block_size);
        for time in res.dfa_times_ms {
            print!(";{}", time.as_millis());
            print!(";{:.3}", res.bytes_scanned as f64 / time.as_nanos() as f64);
        }
        print!("\n");
    }
}

fn generate_input(size: usize) -> Vec<u8> {
    let rng = rand::thread_rng;
    let mut bytes = vec![0u8; size];
    rng().fill(&mut bytes[..]);
    bytes
}

fn bench_dfa<T: TransitionTable>(input: &Vec<&'static [u8]>, dfa: &mut DFA<T>) -> Duration {
    let start = Instant::now();
    for _ in 0..10 {
        let mut state = dfa.init;
        for chunk in input {
            state = dfa.run(state, chunk);
        }
    }
    let end = Instant::now();
    let time = end - start;
    time / 10
}

#[cfg(test)]
mod test {
    use std::num::NonZeroUsize;

    use crate::dfa::{TransitionTable, DFA, example_dfa};
    use crate::generate_input;
    use crate::sheng::Sheng;
    use crate::table::{Table1, Table2};

    fn ab<T: TransitionTable>(table: T) {
        // build
        let mut dfa = DFA::new(table);
        let mut input = generate_input(10_000);
        input[0] = b'h';
        input[1] = b'e';
        input[2] = b'l';
        input[3] = b'l';
        input[4] = b'o';

        let size = input.len();
        input[size - 5] = b'w';
        input[size - 4] = b'o';
        input[size - 3] = b'r';
        input[size - 2] = b'l';
        input[size - 1] = b'd';
        let input = Box::leak(input.into_boxed_slice());

        assert!(dfa.run(dfa.init, "helloworld".as_bytes()) == 11);
        assert!(dfa.run(dfa.init, "".as_bytes()) == 1);
        assert!(dfa.run(dfa.init, "hello........world".as_bytes()) == 11);
        assert_eq!(dfa.run(dfa.init, "hello........world ".as_bytes()), 6);
        assert_eq!(dfa.run(dfa.init, input), 11);
    }

    #[test]
    fn ab_test() {
        let transitions = example_dfa();
        ab(Sheng::new(&transitions, NonZeroUsize::new(1).unwrap()));
        ab(Table1::new(&transitions));
        ab(Table2::new(&transitions));
        ab(Sheng::new(&transitions, NonZeroUsize::new(2).unwrap()));
    }
}
