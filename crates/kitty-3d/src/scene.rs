use crate::Mesh;

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub id: u64,
    pub mesh: Mesh,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Scene {
    pub entities: Vec<Entity>,
}

impl Scene {
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn vertex_count(&self) -> usize {
        self.entities.iter().map(|e| e.mesh.vertices.len()).sum()
    }
}
