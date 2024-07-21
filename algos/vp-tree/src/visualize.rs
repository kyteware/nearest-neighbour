use nn_structure::{image::RgbImage, imagecolors::{node_of_depth, BACKGROUND, SEPERATOR}, imageproc::{drawing::{draw_filled_circle_mut, draw_filled_rect_mut, draw_hollow_circle_mut}, rect::Rect}, NNStructureVisualize};

use crate::{Branch, Node, VpTree};

impl<T> NNStructureVisualize for VpTree<T> {
    fn visualize(&self) -> RgbImage {
        let mut image = RgbImage::new(500, 500);
        draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(500, 500), BACKGROUND);
        draw_node(&mut image, &self.root, 0);
        image
    }
}

fn draw_node(image: &mut RgbImage, node: &Node, depth: u32) {
    let loc = (node.loc.0.round() as i32 + 250, node.loc.1.round() as i32 + 250);
    draw_filled_circle_mut(image, loc, 2, node_of_depth(depth));
    
    if let Some(Branch { radius, inner_outer }) = &node.branch {
        draw_hollow_circle_mut(image, loc, radius.round() as i32, SEPERATOR);

        let (inner, outer) = &**inner_outer;
        draw_node(image, inner, depth + 1);
        if let Some(outer) = outer {
            draw_node(image, outer, depth + 1);
        }
    }
}