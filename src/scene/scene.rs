use crate::scene::polygon::Polygon;

pub struct Scene {
    polygons: Vec<Polygon>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { polygons: vec![] }
    }

    pub fn add_polygon(&mut self, polygon: Polygon) {
        self.polygons.push(polygon);
    }

    pub fn polygons(&self) -> Vec<Polygon> {
        self.polygons.clone()
    }
}
