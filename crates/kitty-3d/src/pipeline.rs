use crate::{Camera, Scene};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameStats {
    pub entities: usize,
    pub vertices: usize,
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub backend: String,
}

impl Pipeline {
    pub fn new(backend: impl Into<String>) -> Self {
        Self {
            backend: backend.into(),
        }
    }

    pub fn render(&self, scene: &Scene, _camera: &Camera) -> FrameStats {
        FrameStats {
            entities: scene.entities.len(),
            vertices: scene.vertex_count(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Camera, Entity, Mesh, Pipeline, Scene};

    #[test]
    fn pipeline_reports_frame_stats() {
        let pipeline = Pipeline::new("software-3d-v0");
        let mut scene = Scene::default();
        scene.add_entity(Entity {
            id: 1,
            mesh: Mesh::triangle("triangle"),
        });

        let stats = pipeline.render(&scene, &Camera::default());
        assert_eq!(stats.entities, 1);
        assert_eq!(stats.vertices, 3);
    }
}
