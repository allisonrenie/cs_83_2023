use std::f32::consts::PI;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use rbda::{
    mesh::Mesh as RBDA_Mesh,
    sva::{Force, Inertia, InertiaAB, Matrix, Motion, Vector, Xform},
};

use crate::state::State;
#[derive(Default, Debug)]
pub enum JointType {
    Base,
    #[default]
    Rx,
    Ry,
    Rz,
    Px,
    Py,
    Pz,
}

#[derive(Component, Default, Debug)]
pub struct Base;

#[derive(Component, Default, Debug)]
pub struct Joint {
    pub joint_type: JointType,
    pub name: String,

    // joint definition
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
    pub f_ext: Force,
    pub dd: f32,
    pub u: f32,
    pub uu: Force,
    pub meshes: Vec<RBDA_Mesh>,
    pub stiffness: f32,
    pub damping: f32,
}

impl Joint {
    pub fn base(a: Motion) -> Self {
        Self {
            a,
            joint_type: JointType::Base,
            ..Default::default()
        }
    }
    pub fn rx(inertia: Inertia, xt: Xform) -> Self {
        let s = Motion::new([0., 0., 0.], [1., 0., 0.]);

        Self {
            i: inertia,
            xt,
            s,
            joint_type: JointType::Rx,
            ..Default::default()
        }
    }

    pub fn ry(inertia: Inertia, xt: Xform) -> Self {
        let s = Motion::new([0., 0., 0.], [0., 1., 0.]);

        Self {
            i: inertia,
            xt,
            s,
            joint_type: JointType::Ry,
            ..Default::default()
        }
    }

    pub fn rz(inertia: Inertia, xt: Xform) -> Self {
        let s = Motion::new([0., 0., 0.], [0., 0., 1.]);

        Self {
            i: inertia,
            xt,
            s,
            joint_type: JointType::Rz,
            ..Default::default()
        }
    }
    pub fn px(inertia: Inertia, xt: Xform) -> Self {
        let s = Motion::new([1., 0., 0.], [0., 0., 0.]);

        Self {
            i: inertia,
            xt,
            s,
            joint_type: JointType::Px,
            ..Default::default()
        }
    }
    pub fn py(inertia: Inertia, xt: Xform) -> Self {
        let s = Motion::new([0., 1., 0.], [0., 0., 0.]);

        Self {
            i: inertia,
            xt,
            s,
            joint_type: JointType::Py,
            ..Default::default()
        }
    }
    pub fn pz(inertia: Inertia, xt: Xform) -> Self {
        let s = Motion::new([0., 0., 1.], [0., 0., 0.]);

        Self {
            i: inertia,
            xt,
            s,
            joint_type: JointType::Pz,
            ..Default::default()
        }
    }

    pub fn get_state(&self) -> State {
        State {
            q: self.q,
            qd: self.qd,
        }
    }

    pub fn set_state(&mut self, state: State) {
        self.q = state.q;
        self.qd = state.qd;
    }

    pub fn get_dstate(&self) -> State {
        State {
            q: self.qd,
            qd: self.qdd,
        }
    }

    pub fn set_dstate(&mut self, dstate: State) {
        self.qd = dstate.q;
        self.qdd = dstate.qd;
    }
}

pub fn build_model(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // create base
    let base = Joint::base(Motion::new([0., 0., 9.81], [0., 0., 0.]));
    let base_id = commands
        .spawn(base)
        .insert(Base)
        // sets visibility and position of the base
        .insert(SpatialBundle {
            ..Default::default()
        })
        .id();

    // first pendulum link
    let xt_0 = Xform::identity();
    let rx_0_id = create_pendulum_link(commands, meshes, materials, xt_0, 90.0);

    // second pendulum link
    let xt_1 = Xform::new(Vector::new(0., 0., -1.), Matrix::identity());
    let rx_1_id = create_pendulum_link(commands, meshes, materials, xt_1, 0.0);

    // bevy parent-child hierarchy
    commands.entity(base_id).push_children(&[rx_0_id]);
    commands.entity(rx_0_id).push_children(&[rx_1_id]);
}

fn create_pendulum_link(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    xt: Xform,
    initial_angle: f32,
) -> Entity {
    let inertia = Inertia::new(
        1.,
        Vector::new(0., 0., -0.5),
        (1. / 12.) * Matrix::from_diagonal(&Vector::new(1., 1., 0.04)),
    );
    let mut rx = Joint::rx(inertia, xt);
    rx.q = initial_angle.to_radians();
    let mut rx_e = commands.spawn(rx);
    add_pendulum_mesh(&mut rx_e, meshes, materials);
    rx_e.id()
}

fn add_pendulum_mesh(
    entity: &mut EntityCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    entity.insert(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box {
            max_x: 0.1,
            min_x: -0.1,
            max_y: 0.1,
            min_y: -0.1,
            max_z: 0.,
            min_z: -1.0,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::RED,
            ..default()
        }),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

pub fn build_default_environment(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    // add point light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(1.0, 2.0, 2.0),
        point_light: PointLight {
            intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // add ground plane
    let ground_transform = Transform::from_translation(Vec3 {
        x: 0.,
        y: 0.,
        z: -3.0,
    }) * Transform::from_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.));
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 10.0,
            subdivisions: 10,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        }),
        transform: ground_transform,
        ..default()
    });
}

pub fn bevy_joint_positions(mut joint_transform_query: Query<(&mut Joint, &mut Transform)>) {
    for (joint, mut transform) in joint_transform_query.iter_mut() {
        transform.translation = Vec3::from_slice(joint.xl.position.data.as_slice());
        let mat = Mat3::from_cols_slice(joint.xl.rotation.data.as_slice()).transpose();
        transform.rotation = Quat::from_mat3(&mat);
    }
}
