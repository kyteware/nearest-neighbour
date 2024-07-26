use nn_structure::{Loc, NNStructure};

#[derive(Debug)]
pub struct BasicList<T> {
    elements: Vec<(Loc, T)>
}

impl<T> NNStructure<T> for BasicList<T> {
    fn construct(elements: Vec<(Loc, T)>) -> Self {
        BasicList { elements }
    }

    fn nearest_neighbour(&self, loc: Loc) -> (&T, Loc, f32) {
        let mut nearest = (0, std::f32::INFINITY);
        for (i, element) in self.elements.iter().enumerate() {
            let dist = loc.dist(element.0);
            if dist < nearest.1 {
                nearest.0 = i;
                nearest.1 = dist;
            }
        }

        (&self.elements[nearest.0].1, self.elements[nearest.0].0, nearest.1)
    }
}
