use crate::scene::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: Vec<Vertex>,
}

impl Polygon {
    pub fn new(v1: Vertex, v2: Vertex, v3: Vertex) -> Polygon {
        Polygon {
            vertices: vec![v1, v2, v3],
        }
    }
    
    pub fn vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }
}
