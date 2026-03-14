#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex>,
}

impl Mesh {
    pub fn new(name: impl Into<String>, vertices: Vec<Vertex>) -> Self {
        Self {
            name: name.into(),
            vertices,
        }
    }

    pub fn triangle(name: impl Into<String>) -> Self {
        Self::new(
            name,
            vec![
                Vertex {
                    x: 0.0,
                    y: 0.5,
                    z: 0.0,
                },
                Vertex {
                    x: -0.5,
                    y: -0.5,
                    z: 0.0,
                },
                Vertex {
                    x: 0.5,
                    y: -0.5,
                    z: 0.0,
                },
            ],
        )
    }
}
