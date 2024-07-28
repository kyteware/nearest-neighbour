use std::{hint::black_box, time::Instant};

use basic_list::BasicList;
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

    let (construction_1000, construction_10000, construction_100000) = construction_benches(&mut rng);
    println!("Construction with 1000 nodes:\n{}", construction_1000.formatted());
    println!("Construction with 10000 nodes:\n{}", construction_10000.formatted());
    println!("Construction with 100000 nodes:\n{}", construction_100000.formatted());

    let (nn_oneshot_1000, nn_oneshot_10000, nn_oneshot_100000) = nn_oneshot_benches(&mut rng);
    println!("Nearest neighbour oneshot with 1000 nodes:\n{}", nn_oneshot_1000.formatted());
    println!("Nearest neighbour oneshot with 10000 nodes:\n{}", nn_oneshot_10000.formatted());
    println!("Nearest neighbour oneshot with 100000 nodes:\n{}", nn_oneshot_100000.formatted());
}

fn construction_benches(rng: &mut StdRng) -> (Comparison, Comparison, Comparison) {
    let mut construction_1000s = vec![Comparison::default(); 100];
    for i in 0..100 {
        let nodes = gen_scattered_points(1000, rng);

        construction_1000s[i] = Comparison::bench_construction(nodes);
    }

    let construction_1000 = avg_comp(&construction_1000s);

    let mut construction_10000s = vec![Comparison::default(); 100];
    for i in 0..100 {
        let nodes = gen_scattered_points(10000, rng);

        construction_10000s[i] = Comparison::bench_construction(nodes);
    }

    let construction_10000 = avg_comp(&construction_10000s);

    let mut construction_100000s = vec![Comparison::default(); 100];
    for i in 0..100 {
        let nodes = gen_scattered_points(100000, rng);

        construction_100000s[i] = Comparison::bench_construction(nodes);
    }

    let construction_100000 = avg_comp(&construction_100000s);

    (construction_1000, construction_10000, construction_100000)
}

fn nn_oneshot_benches(rng: &mut StdRng) -> (Comparison, Comparison, Comparison) {
    let mut nn_oneshot_1000s = vec![Comparison::default(); 100];
    for i in 0..100 {
        let nodes = gen_scattered_points(1000, rng);

        let x = rng.gen_range(-100.0..100.0);
        let y = rng.gen_range(-100.0..100.0);
        let target = Loc(x, y);

        nn_oneshot_1000s[i] = Comparison::bench_nn_oneshot(nodes, target);
    }

    let nn_oneshot_1000 = avg_comp(&nn_oneshot_1000s);

    let mut nn_oneshot_10000s = vec![Comparison::default(); 100];
    for i in 0..100 {
        let nodes = gen_scattered_points(10000, rng);

        let x = rng.gen_range(-100.0..100.0);
        let y = rng.gen_range(-100.0..100.0);
        let target = Loc(x, y);

        nn_oneshot_10000s[i] = Comparison::bench_nn_oneshot(nodes, target);
    }

    let nn_oneshot_10000 = avg_comp(&nn_oneshot_10000s);

    let mut nn_oneshot_100000s = vec![Comparison::default(); 100];
    for i in 0..100 {
        let nodes = gen_scattered_points(100000, rng);

        let x = rng.gen_range(-100.0..100.0);
        let y = rng.gen_range(-100.0..100.0);
        let target = Loc(x, y);

        nn_oneshot_100000s[i] = Comparison::bench_nn_oneshot(nodes, target);
    }

    let nn_oneshot_100000 = avg_comp(&nn_oneshot_100000s);

    (nn_oneshot_1000, nn_oneshot_10000, nn_oneshot_100000)
}

const NUM_ALGORITHMS: usize = 2;

/// Basic list: 0
/// VP-Tree: 1
#[derive(Clone, Debug, Default)]
struct Comparison {
    times: [Option<f64>; NUM_ALGORITHMS]
}

impl Comparison {
    fn bench_construction(nodes: Vec<(Loc, ())>) -> Self {
        let mut times: [Option<f64>; NUM_ALGORITHMS] = Default::default();
        times[0] = Some(black_box(bench_function(BasicList::construct, nodes.clone())));
        times[1] = Some(black_box(bench_function(VpTree::construct, nodes.clone())));

        Self { times }
    }

    fn bench_nn_oneshot(nodes: Vec<(Loc, ())>, target: Loc) -> Self {
        let mut times: [Option<f64>; NUM_ALGORITHMS] = Default::default();
        let basic_list = BasicList::construct(nodes.clone());
        times[0] = Some(black_box(bench_function2(BasicList::nearest_neighbour, &basic_list, target)));
        let vp_tree = VpTree::construct(nodes.clone());
        times[1] = Some(black_box(bench_function2(VpTree::nearest_neighbour, &vp_tree, target)));

        Self { times }
    }

    fn formatted(&self) -> String {
        let mut res = String::new();
        if let Some(time) = self.times[0] {
            res.push_str(&format!("Basic list:         {:.4}ms\n", time * 1000.));
        }
        if let Some(time) = self.times[1] {
            res.push_str(&format!("Vantage Point Tree: {:.4}ms\n", time * 1000.));
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

fn gen_scattered_points(num: usize, rng: &mut StdRng) -> Vec<(Loc, ())> {
    let mut nodes = Vec::with_capacity(num);
    for _ in 0..num {
        let x = rng.gen_range(-100.0..100.0);
        let y = rng.gen_range(-100.0..100.0);
        nodes.push((Loc(x, y), ()))
    }

    nodes
}

fn bench_function<I, O>(f: impl FnOnce(I) -> O, input: I) -> f64 {
    let start = Instant::now();
    black_box(f(black_box(input)));
    let end = Instant::now();

    end.duration_since(start).as_secs_f64()
}

fn bench_function2<I1, I2, O>(f: impl FnOnce(I1, I2) -> O, i1: I1, i2: I2) -> f64 {
    let start = Instant::now();
    black_box(f(black_box(i1), black_box(i2)));
    let end = Instant::now();

    end.duration_since(start).as_secs_f64()
}
