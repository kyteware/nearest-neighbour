use nn_structure::{Loc, NNStructure};
pub mod visualize;

#[derive(Debug, )]
pub struct VpTree<T> {
    pub(crate) root: Node,
    elements: Vec<(Loc, T)>
}

impl<T> NNStructure<T> for VpTree<T> {
    fn construct(mut elements: Vec<(Loc, T)>) -> Self {
        let root = build_tree(0, &mut elements);
        VpTree { root, elements }
    }

    fn nearest_neighbour<'a>(&'a self, loc: Loc) -> (&'a T, Loc) {
        todo!()
    }
}

fn build_tree<T>(offset: usize, mut elements: &mut [(Loc, T)]) -> Node {
    if elements.len() == 1 {
        return Node { loc: elements[0].0, index: offset, branch: None }
    }

    if elements.len() == 2 {
        return Node {
            loc: elements[0].0,
            index: offset,
            branch: Some(Branch {
                radius: elements[0].0.dist(elements[1].0),
                inner_outer: Box::new((Node {
                    loc: elements[1].0, index: offset + 1, branch: None
                }, None))
            })
        }
    }

    let (vp_loc, _) = elements[0];
    elements = &mut elements[1..];

    let median = elements.len() / 2;
    elements.select_nth_unstable_by(median, |a, b| vp_loc.dist(b.0).partial_cmp(&vp_loc.dist(a.0)).unwrap());

    let radius = elements[median].0.dist(vp_loc);
    let (inner, outer) = elements.split_at_mut(median);

    Node {
        loc: vp_loc,
        index: offset,
        branch: Some(Branch { radius, inner_outer: Box::new((build_tree(offset + 1, inner), Some(build_tree(offset + median + 1, outer)))) })
    }
}

// fn search<T>(target: Loc, node: Node<T>) -> (T, f32) {
//     if let Some(inner_outer) = node.inner_outer {
//         let [inner, outer] = *inner_outer;
        
//         if target.dist(&node.loc) > node.
//     } else {
//         (node.item, node.loc.dist(&target))
//     }
// }

#[derive(Debug)]
pub struct Node {
    loc: Loc,
    index: usize,
    branch: Option<Branch>
}

#[derive(Debug)]
pub struct Branch {
    radius: f32,
    inner_outer: Box<(Node, Option<Node>)>
}
