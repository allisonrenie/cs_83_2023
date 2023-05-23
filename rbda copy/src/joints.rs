use crate::{
    mesh::Mesh,
    sva::{Force, Inertia, InertiaAB, Motion, Xform},
};

#[derive(Default, Debug)]
pub struct Joint {
    // joint definition
    pub parent: Option<usize>,
    pub s: Motion,
    pub i: Inertia,
    pub xt: Xform,

    // joint state (and solution)
    pub q: f32,
    pub qd: f32,
    pub qdd: f32,

    // common parameters
    pub xl: Xform,
    pub xj: Xform,
    pub x: Xform,
    pub v: Motion,
    pub vj: Motion,
    pub c: Motion,
    pub a: Motion,

    // algorithm specific parameters
    pub iaa: InertiaAB,
    pub paa: Force,
    pub tau: f32,
    pub dd: f32,
    pub u: f32,
    pub uu: Force,
    pub meshes: Vec<Mesh>,
}

impl Joint {
    pub fn base(a: Motion) -> Self {
        Self {
            a,
            parent: None,
            ..Default::default()
        }
    }
    pub fn rx(parent: Option<usize>, inertia: Inertia, xt: Xform) -> Self {
        let s = Motion::new([0.0, 0., 0.], [1.0, 0., 0.]);

        Self {
            parent,
            i: inertia,
            xt,
            s,
            ..Default::default()
        }
    }
}
