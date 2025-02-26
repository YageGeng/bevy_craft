use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

#[derive(Default, Debug)]
pub struct Vertex {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl Vertex {
    pub fn merge(&mut self, other: Self) {
        let indice_offset = self.positions.len() as u32;
        self.positions.reserve(other.positions.len());
        self.positions.extend(other.positions);

        self.normals.reserve(other.normals.len());
        self.normals.extend(other.normals);

        self.uvs.reserve(other.uvs.len());
        self.uvs.extend(other.uvs);

        self.indices.reserve(other.indices.len());
        self.indices.extend(
            other
                .indices
                .into_iter()
                .map(|indice| indice + indice_offset),
        );
    }
}

impl Extend<Vertex> for Vertex {
    fn extend<T: IntoIterator<Item = Vertex>>(&mut self, vertexs: T) {
        for vertex in vertexs {
            self.merge(vertex);
        }
    }
}

impl From<Vertex> for Mesh {
    fn from(value: Vertex) -> Self {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, value.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, value.uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, value.normals)
        .with_inserted_indices(Indices::U32(value.indices))
    }
}
