use vp_tree::*;
use nn_structure::*;

fn main() {
    let tree = VpTree::construct(vec![(Loc(1.0, 0.0), 1), (Loc(2.0, 0.0), 2), (Loc(3.0, 0.0), 3), (Loc(4.0, 0.0), 4), (Loc(5.0, 0.0), 5)]);
    dbg!(tree);
}