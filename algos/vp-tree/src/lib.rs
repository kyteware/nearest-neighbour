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

    fn nearest_neighbour(&self, loc: Loc) -> (&T, Loc, f32) {
        let (index, dist) = search(loc, &self.root);
        (&self.elements[index].1, self.elements[index].0, dist)
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

fn search(target: Loc, node: &Node) -> (usize, f32) {
    if let Some(Branch { radius, inner_outer }) = &node.branch {
        let (inner, outer) = &**inner_outer;
        let mut res = (node.index, node.loc.dist(target));
        if res.1 > radius / 2. && outer.is_some() {
            let outside_res = search(target, &outer.as_ref().unwrap());
            if outside_res.1 < res.1 {
                res = outside_res;
            }
        }
        let inside_res = search(target, &inner);
        if inside_res.1 < res.1 {
            res = inside_res;
        }
        res
    } else {
        (node.index, node.loc.dist(target))
    }
}

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

#[cfg(test)]
mod tests {
    use nn_structure::{Loc, NNStructure};

    use crate::VpTree;

    #[test]
    fn vp_nearest_neighbour_search_easy() {
        let points = vec![
            (Loc(0., 0.), 0usize),
            (Loc(2., 2.), 1),
            (Loc(4., 4.), 2),
            (Loc(5., 5.), 3),
            (Loc(6., 0.), 4)
        ];
        let vp_tree = VpTree::construct(points.clone());

        assert_eq!(*vp_tree.nearest_neighbour(Loc(2.5, 2.5)).0, 1);
        assert_eq!(*vp_tree.nearest_neighbour(Loc(100., -100.)).0, 4);
        assert_eq!(*vp_tree.nearest_neighbour(Loc(5.5, 6.)).0, 3);
    }

    #[test]
    fn vp_nearest_neighbour_search_hard() {
        let points = vec![
            (Loc(10.5, 10.2), 0usize),
            (Loc(11.1, 12.6), 1),
            (Loc(16.7, 16.5), 2),
            (Loc(10.4, 8.5), 3),
            (Loc(11.9, 11.7), 4),
            (Loc(12.2, 14.5), 5),
            (Loc(16.5, 9.8), 6),
            (Loc(9.2, 4.5), 7),
        ];
        let vp_tree = VpTree::construct(points.clone());

        for i in 5..15 {
            let mut res = (8, 100000000000.);
            let loc = Loc(i as f32, 10.);
            for j in 0..8 {
                let dist = loc.dist(points[j].0);
                println!("j: {}, dist: {}", j, dist);
                if dist < res.1 {
                    res = (j, dist);
                }
            }
            let vp_res = vp_tree.nearest_neighbour(loc).0;
            assert_eq!(*vp_res, res.0, "on x = {}", i)
        }
    }
}
