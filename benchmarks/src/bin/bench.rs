use std::{hint::black_box, time::Instant};

use nn_structure::{Loc, NNStructure};
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use vp_tree::VpTree;

fn main() {
    let mut rng = thread_rng();
    println!("warming up...");

    // just random shit that hopefully wont get optimized away
    let mut starting_nums: [u8; 5] = rng.gen();
    let start = Instant::now();
    loop {
        starting_nums.reverse();
        starting_nums[1] = starting_nums[2].wrapping_add(1);
        starting_nums[2] = starting_nums[3].wrapping_add(2);
        starting_nums[3] = starting_nums[1].wrapping_add(3);
        if start.elapsed().as_secs_f64() > 5. {
            black_box(starting_nums);
            break;
        }
    }

    let mut rng = StdRng::seed_from_u64(1000);

    // construction
    let mut construction_1000s = vec![Comparison::default(); 100];
    for i in 0..100 {
        let mut nodes = Vec::with_capacity(1000);
        for _ in 0..1000 {
            let x = rng.gen_range(-100.0..100.0);
            let y = rng.gen_range(-100.0..100.0);
            nodes.push((Loc(x, y), ()))
        }

        construction_1000s[i] = Comparison::bench_construction(nodes);
    }

    let construction_1000 = avg_comp(&construction_1000s);

    println!("Construction with 1000 nodes:\n{}", construction_1000.formatted());

    let mut construction_10000s = vec![Comparison::default(); 100];
    for i in 0..100 {
        let mut nodes = Vec::with_capacity(10000);
        for _ in 0..10000 {
            let x = rng.gen_range(-100.0..100.0);
            let y = rng.gen_range(-100.0..100.0);
            nodes.push((Loc(x, y), ()))
        }

        construction_10000s[i] = Comparison::bench_construction(nodes);
    }

    let construction_10000 = avg_comp(&construction_10000s);

    println!("Construction with 10000 nodes:\n{}", construction_10000.formatted());
}

const NUM_ALGORITHMS: usize = 1;

/// VP-Tree: 1
#[derive(Clone, Debug, Default)]
struct Comparison {
    times: [Option<f64>; NUM_ALGORITHMS]
}

impl Comparison {
    fn bench_construction(nodes: Vec<(Loc, ())>) -> Self {
        let mut times: [Option<f64>; NUM_ALGORITHMS] = Default::default();
        times[0] = Some(black_box(bench_function(VpTree::construct, nodes.clone())));

        Self { times }
    }

    fn formatted(&self) -> String {
        let mut res = String::new();
        if let Some(time) = self.times[0] {
            res.push_str(&format!("Vantage Point Tree: {:.2}ms\n", time * 1000.));
        }
        res
    }
}

/// assumes all comps have the same some/none
fn avg_comp(comps: &[Comparison]) -> Comparison {
    let mut avg_times: [Option<f64>; NUM_ALGORITHMS] = Default::default();
    for i in 0..NUM_ALGORITHMS {
        if comps[0].times[i].is_some() {
            let mut avg = 0.;
            for comp in comps {
                avg += comp.times[i].unwrap();
            }
            avg /= comps.len() as f64;
            avg_times[i] = Some(avg);
        }
    }

    Comparison { times: avg_times }
}

fn bench_function<I, O>(f: impl FnOnce(I) -> O, input: I) -> f64 {
    let start = Instant::now();
    black_box(f(black_box(input)));
    let end = Instant::now();

    end.duration_since(start).as_secs_f64()
}
