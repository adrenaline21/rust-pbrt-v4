use crate::Float;

use super::{
    float::Num,
    math::{
        compensated_float::{inner_product_internal_12, inner_product_internal_6},
        square_matrix::SquareMatrix,
    },
    vecmath::{
        vector::{cross, normalize},
        Point3f, Tuple3, Vector3f,
    },
};

#[derive(Debug)]
pub struct Transform {
    m: SquareMatrix<4>,
    m_inv: SquareMatrix<4>,
}

pub fn look_at(pos: Point3f, look: Point3f, up: Vector3f) -> Transform {
    let mut world_from_camera = SquareMatrix::<4>::new();
    world_from_camera[0][3] = pos.x();
    world_from_camera[1][3] = pos.y();
    world_from_camera[2][3] = pos.z();
    world_from_camera[3][3] = 1.0;

    let dir = normalize(look - pos);
    // unimplemented!(lookat, up same direction)
    let right = normalize(cross(normalize(up), dir));
    let new_up = cross(dir, right);

    world_from_camera[0][0] = right.x();
    world_from_camera[1][0] = right.y();
    world_from_camera[2][0] = right.z();
    world_from_camera[3][0] = 0.0;
    world_from_camera[0][1] = new_up.x();
    world_from_camera[1][1] = new_up.y();
    world_from_camera[2][1] = new_up.z();
    world_from_camera[3][1] = 0.0;
    world_from_camera[0][2] = dir.x();
    world_from_camera[1][2] = dir.y();
    world_from_camera[2][2] = dir.z();
    world_from_camera[3][2] = 0.0;

    let camera_from_world = inverse(&world_from_camera).unwrap();
    Transform {
        m: world_from_camera,
        m_inv: camera_from_world,
    }
}

#[inline]
pub fn inverse(m: &SquareMatrix<4>) -> Option<SquareMatrix<4>> {
    let s0 = Float::difference_of_products(m[0][0], m[1][1], m[1][0], m[0][1]);
    let s1 = Float::difference_of_products(m[0][0], m[1][2], m[1][0], m[0][2]);
    let s2 = Float::difference_of_products(m[0][0], m[1][3], m[1][0], m[0][3]);

    let s3 = Float::difference_of_products(m[0][1], m[1][2], m[1][1], m[0][2]);
    let s4 = Float::difference_of_products(m[0][1], m[1][3], m[1][1], m[0][3]);
    let s5 = Float::difference_of_products(m[0][2], m[1][3], m[1][2], m[0][3]);

    let c0 = Float::difference_of_products(m[2][0], m[3][1], m[3][0], m[2][1]);
    let c1 = Float::difference_of_products(m[2][0], m[3][2], m[3][0], m[2][2]);
    let c2 = Float::difference_of_products(m[2][0], m[3][3], m[3][0], m[2][3]);

    let c3 = Float::difference_of_products(m[2][1], m[3][2], m[3][1], m[2][2]);
    let c4 = Float::difference_of_products(m[2][1], m[3][3], m[3][1], m[2][3]);
    let c5 = Float::difference_of_products(m[2][2], m[3][3], m[3][2], m[2][3]);

    // unimplemented!(compensated sum)
    let determinant =
        inner_product_internal_12(s0, c5, -s1, c4, s2, c3, s3, c2, s5, c0, -s4, c1).to_float();
    if determinant == 0.0 {
        return None;
    }
    let s = 1.0 / determinant;

    Some(SquareMatrix::<4>([
        [
            s * inner_product_internal_6(m[1][1], c5, m[1][3], c3, -m[1][2], c4).to_float(),
            s * inner_product_internal_6(-m[0][1], c5, m[0][2], c4, -m[0][3], c3).to_float(),
            s * inner_product_internal_6(m[3][1], s5, m[3][3], s3, -m[3][2], s4).to_float(),
            s * inner_product_internal_6(-m[2][1], s5, m[2][2], s5, -m[2][3], s4).to_float(),
        ],
        [
            s * inner_product_internal_6(-m[1][0], c5, m[1][2], c2, -m[1][3], c1).to_float(),
            s * inner_product_internal_6(m[0][0], c5, m[0][3], c1, -m[0][2], c2).to_float(),
            s * inner_product_internal_6(-m[3][0], s5, m[3][2], s2, -m[3][3], s1).to_float(),
            s * inner_product_internal_6(m[2][0], s5, m[2][3], s1, -m[2][2], s2).to_float(),
        ],
        [
            s * inner_product_internal_6(m[1][0], c4, m[1][3], c0, -m[1][1], c2).to_float(),
            s * inner_product_internal_6(-m[0][0], c4, m[0][1], c2, -m[0][3], c0).to_float(),
            s * inner_product_internal_6(m[3][0], s4, m[3][3], s0, -m[3][1], s2).to_float(),
            s * inner_product_internal_6(-m[2][0], s4, m[2][1], s2, -m[2][3], s0).to_float(),
        ],
        [
            s * inner_product_internal_6(-m[1][0], c3, m[1][1], c1, -m[1][2], c0).to_float(),
            s * inner_product_internal_6(m[0][0], c3, m[0][2], c0, -m[0][1], c1).to_float(),
            s * inner_product_internal_6(-m[3][0], s3, m[3][1], s1, -m[3][2], s0).to_float(),
            s * inner_product_internal_6(m[2][0], s3, m[2][2], s0, -m[2][1], s1).to_float(),
        ],
    ]))
}

mod test {}
