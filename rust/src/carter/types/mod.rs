use nalgebra::{Vector2, Vector3};

//----------------------------------------------------------------------VERTEX
#[derive(Debug)]
pub struct Vertex<N: 'static + std::fmt::Debug + std::cmp::PartialEq + std::marker::Copy> {
    pub position: Vector3<N>,
    pub normal: Vector3<N>,
    pub texture_coords: Vector2<N>,
}

impl<N: 'static + std::fmt::Debug + std::cmp::PartialEq + std::marker::Copy> Vertex<N> {
    pub fn new(position: Vector3<N>, normal: Vector3<N>, texture_coords: Vector2<N>) -> Vertex<N> {
        Vertex {
            position,
            normal,
            texture_coords,
        }
    }
}

//----------------------------------------------------------------------TEXTURE
#[derive(Debug)]
pub enum TextureType {}

#[derive(Debug)]
pub struct Texture {
    id: usize,
    texture_type: TextureType,
}

impl Texture {
    pub fn new(id: usize, texture_type: TextureType) -> Texture {
        Texture { id, texture_type }
    }
}
