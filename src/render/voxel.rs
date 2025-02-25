use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

#[derive(Debug)]
pub struct Vertex {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl MeshBuilder for Vertex {
    fn build(&self) -> Mesh {
        // for (idx, pos) in self.positions.chunks(4).enumerate() {
        //     bevy::log::info!("Normal {:?}", self.normals[idx * 4]);
        //     bevy::log::info!("{:?}", pos);
        //     bevy::log::info!("{:?}", &self.indices[idx * 6..idx * 6 + 6])
        // }

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions.clone())
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone())
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.clone())
        .with_inserted_indices(Indices::U32(self.indices.clone()))
    }
}
