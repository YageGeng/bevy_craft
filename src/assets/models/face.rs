use bevy::math::{I8Vec2, Vec3};

use crate::assets::models::prelude::*;

pub const DEFAULT_ELEMENT_SIZE_F32: f32 = 16.0;
pub const DEFAULT_ELEMENT_SIZE_I8: i8 = 16;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct I8Rect {
    pub min: I8Vec2,
    pub max: I8Vec2,
}

pub struct Face<'a> {
    pub from: [i8; 3],
    pub to: [i8; 3],
    pub face: BlockFace,
    pub data: &'a ElementFace,
}

pub trait FaceAble {
    fn from(&self) -> [i8; 3];

    fn to(&self) -> [i8; 3];

    fn min(&self) -> Vec3 {
        let from = self.from();
        Vec3::new(
            from[0] as f32 / DEFAULT_ELEMENT_SIZE_F32,
            from[1] as f32 / DEFAULT_ELEMENT_SIZE_F32,
            from[2] as f32 / DEFAULT_ELEMENT_SIZE_F32,
        )
    }

    fn max(&self) -> Vec3 {
        let to = self.to();
        Vec3::new(
            to[0] as f32 / DEFAULT_ELEMENT_SIZE_F32,
            to[1] as f32 / DEFAULT_ELEMENT_SIZE_F32,
            to[2] as f32 / DEFAULT_ELEMENT_SIZE_F32,
        )
    }

    fn rect(&self, face: BlockFace) -> I8Rect {
        let min = self.from();
        let max = self.to();
        let (x1, y1, x2, y2) = match face {
            BlockFace::Down | BlockFace::Up => (min[0], min[2], max[0], max[2]),
            BlockFace::North | BlockFace::South => (min[0], min[1], max[0], max[1]),
            BlockFace::West | BlockFace::East => (min[1], min[2], max[1], max[2]),
        };

        I8Rect {
            min: I8Vec2::new(x1.min(x2), y1.min(y2)),
            max: I8Vec2::new(x1.max(x2), y1.max(y2)),
        }
    }

    // Only determine whether this surface is 16x16
    fn is_complete_face(&self, face: BlockFace) -> bool {
        self.rect(face) == I8Rect::STANDARD
    }

    // only determine whether this surface is start with 0/16
    fn is_normal_face(&self, face: BlockFace) -> bool {
        let value = match face {
            BlockFace::Down => self.from()[1],
            BlockFace::Up => self.to()[1],
            BlockFace::North => self.from()[2],
            BlockFace::South => self.to()[2],
            BlockFace::West => self.from()[0],
            BlockFace::East => self.to()[0],
        };

        face.default_size() == value
    }
}

impl FaceAble for Face<'_> {
    fn from(&self) -> [i8; 3] {
        self.from
    }

    fn to(&self) -> [i8; 3] {
        self.to
    }
}

impl I8Rect {
    pub const STANDARD: I8Rect = I8Rect {
        min: I8Vec2::ZERO,
        max: I8Vec2::splat(DEFAULT_ELEMENT_SIZE_I8),
    };

    #[inline]
    pub fn new(x1: i8, y1: i8, x2: i8, y2: i8) -> Self {
        Self {
            min: I8Vec2::new(x1, y1),
            max: I8Vec2::new(x2, y2),
        }
    }

    pub fn contains(&self, point: I8Vec2) -> bool {
        (point.cmpge(self.min) & point.cmple(self.max)).all()
    }
}
