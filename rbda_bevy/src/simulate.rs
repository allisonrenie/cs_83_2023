use bevy::{prelude::*, time::Stopwatch};
use rbda::{mesh::BoxMesh, model::Model};

use crate::{Joint, ModelCmp};

pub fn build_model(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    model: Model,
) {
    // add meshes to joints
    let mut joints = Vec::new();
    for joint in model.joints.iter() {
        // println!("{:?}", joint);
        let mut joint_entity = commands.spawn(Joint);
        joints.push(joint_entity.id());
        for mesh in joint.meshes.iter() {
            match mesh {
                rbda::mesh::Mesh::Box(BoxMesh {
                    min_x,
                    max_x,
                    min_y,
                    max_y,
                    min_z,
                    max_z,
                }) => {
                    joint_entity.insert(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box {
                            max_x: *max_x,
                            min_x: *min_x,
                            max_y: *max_y,
                            min_y: *min_y,
                            max_z: *max_z,
                            min_z: *min_z,
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::RED,
                            ..default()
                        }),
                        transform: Transform::from_xyz(0.0, 0.0, 0.),
                        ..default()
                    });
                } // rbda::Mesh::Cylinder => {
                  //     joint_entity.insert(Transform::from_xyz(0.0, 0.0, 0.));
                  // }
            }
        }
    }

    // add model component to Bevy ECS
    commands.spawn(ModelCmp {
        model,
        joints: joints,
        time: Stopwatch::new(),
    });
}

pub fn simulate_system(
    time: Res<Time>,
    mut model_query: Query<&mut ModelCmp>,
    mut transform_query: Query<&mut Transform>,
) {
    for mut model in model_query.iter_mut() {
        // time stuff
        model.time.tick(time.delta());

        let dt = time.delta().as_secs_f32();
        if false {
            // Forward Euler
            let state_1 = model.model.get_state();
            let dstate_1 = model.model.get_dstate();

            let state = state_1 + dstate_1 * dt;
            model.model.set_state(state);
            model.model.update();
        } else {
            // Runge-Kutta 4th order integration (https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods)
            let state_1 = model.model.get_state();
            let dstate_1 = model.model.get_dstate();

            let state_2 = &state_1 + dt / 2. * &dstate_1;
            model.model.set_state(state_2);
            model.model.update();
            let dstate_2 = model.model.get_dstate();

            let state_3 = &state_1 + dt / 2. * &dstate_2;
            model.model.set_state(state_3);
            model.model.update();
            let dstate_3 = model.model.get_dstate();

            let state_4 = &state_1 + dt * &dstate_3;
            model.model.set_state(state_4);
            model.model.update();
            let dstate_4 = model.model.get_dstate();

            let state =
                &state_1 + 1. / 6. * (dstate_1 + 2. * dstate_2 + 2. * dstate_3 + dstate_4) * dt;
            model.model.set_state(state);
            model.model.update();
        }

        bevy_joint_positions(&mut transform_query, &model)
    }
}

pub fn bevy_joint_positions(transform_query: &mut Query<&mut Transform>, model: &ModelCmp) {
    for (joint_ind, joint_id) in model.joints.iter().enumerate() {
        if let Ok(mut transform) = transform_query.get_mut(*joint_id) {
            let joint = model.model.joints.get(joint_ind).unwrap();

            transform.translation = Vec3::from_slice(joint.xl.position.data.as_slice());
            let mat = Mat3::from_cols_slice(joint.x.rotation.data.as_slice());
            transform.rotation = Quat::from_mat3(&mat);
        };
    }
}
