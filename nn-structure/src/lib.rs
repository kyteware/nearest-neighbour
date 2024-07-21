use image::RgbImage;

pub trait NNStructure<T> {
    fn construct(elements: Vec<(Loc, T)>) -> Self;
    fn nearest_neighbour(&self, loc: Loc) -> (&T, Loc);
}

pub trait NNStructureVisualize {
    /// Returns a visualization of the structure in a 500 by 500 image.
    fn visualize(&self) -> RgbImage;
}

pub use image;
pub use imageproc;

pub mod imagecolors {
    use image::Rgb;

    pub const BACKGROUND: Rgb<u8> = Rgb([40, 40, 40]);
    pub const MASTER_BACKGROUND: Rgb<u8> = Rgb([30, 30, 30]);
    pub const NODE: Rgb<u8> = Rgb([25, 125, 25]);
    pub fn node_of_depth(depth: u32) -> Rgb<u8> {
        let depth = depth as f32;
        Rgb([(200.*depth/(depth + 3.)) as u8, 125, 25])
    }
    pub const SEPERATOR: Rgb<u8> = Rgb([25, 75, 125]);
    pub const TEXT: Rgb<u8> = Rgb([180, 180, 180]);
}

#[derive(Clone, Copy, Debug)]
pub struct Loc(pub f32, pub f32);

impl Loc {
    pub fn dist(&self, other: Loc) -> f32 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }

    pub fn dist_rough(&self, other: Loc) -> f32 {
        (self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)
    }
}