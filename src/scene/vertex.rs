#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vertex {
    pub fn new(x: f64, y: f64, z: f64) -> Vertex {
        Vertex { x, y, z }
    }
}
