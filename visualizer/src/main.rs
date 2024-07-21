use ab_glyph::FontRef;
use nn_structure::{image::{imageops::overlay, RgbImage}, imagecolors::{MASTER_BACKGROUND, TEXT}, imageproc::{drawing::{draw_filled_rect_mut, draw_text_mut}, rect::Rect}, Loc, NNStructure, NNStructureVisualize};
use rand::{thread_rng, Rng};
use vp_tree::VpTree;

fn main() {
    let scattered_100_points = gen_scattered_points(100);
    let images = feed_points(scattered_100_points);

    let mut master_image = RgbImage::new(20 + 520 * images.len() as u32, 600);
    draw_filled_rect_mut(&mut master_image, Rect::at(0, 0).of_size(10000, 600), MASTER_BACKGROUND);
    let font = FontRef::try_from_slice(include_bytes!("../assets/fire-code.otf")).unwrap();

    for (i, (name, image)) in images.iter().enumerate() {
        overlay(&mut master_image, image, 20, 20 + i as i64 * 120);
        draw_text_mut(&mut master_image, TEXT, 20 + i as i32 * 120, 520, 40., &font, name);
    }

    master_image.save("renders/100_scattered_point_comparison.png").unwrap();
}

fn gen_scattered_points(num: u32) -> Vec<Loc> {
    let mut rng = thread_rng();
    (0..num).map(|_| Loc(rng.gen_range(-250.0..250.0), rng.gen_range(-250.0..250.0))).collect()
}

fn feed_points(points: Vec<Loc>) -> Vec<(&'static str, RgbImage)> {
    let mut visualizations: Vec<(&str, RgbImage)> = vec![];
    let num_points = points.len();
    let elements: Vec<(Loc, usize)> = points.into_iter().zip(0..num_points).collect();
    visualizations.push(("Vantage Point Tree", VpTree::construct(elements.clone()).visualize()));
    visualizations
}
