// allow unused code for now
#![allow(dead_code)]

use bevy::prelude::*;

use crate::build::{Base, Joint};

use crate::state::{get_model_state, set_model_state};
use crate::structure::all_loops;

pub fn runge_kutta(
    time: Res<Time>,
    base_query: Query<Entity, With<Base>>,
    joint_children_query: Query<&Children, With<Joint>>,
    mut joint_query: Query<&mut Joint>,
) {
    let dt = time.delta().as_secs_f32();

    let state0 = get_model_state(&joint_query);
    let dstate0 = all_loops(
        &state0,
        &base_query,
        &joint_children_query,
        &mut joint_query,
    );

    let state1 = state0.clone() + dstate0.clone() * dt * (1. / 2.);
    let dstate1 = all_loops(
        &state1,
        &base_query,
        &joint_children_query,
        &mut joint_query,
    );

    let state2 = state0.clone() + dstate1.clone() * dt * (1. / 2.);
    let dstate2 = all_loops(
        &state2,
        &base_query,
        &joint_children_query,
        &mut joint_query,
    );

    let state3 = state0.clone() + dstate2.clone() * dt;
    let dstate3 = all_loops(
        &state3,
        &base_query,
        &joint_children_query,
        &mut joint_query,
    );

    let state = state0 + (dstate0 + dstate1 * 2. + dstate2 * 2. + dstate3) * (dt / 6.0);
    set_model_state(&mut joint_query, &state);
}

pub fn euler(
    time: Res<Time>,
    base_query: Query<Entity, With<Base>>,
    joint_children_query: Query<&Children, With<Joint>>,
    mut joint_query: Query<&mut Joint>,
) {
    let steps = 2;
    let dt = time.delta().as_secs_f32() / steps as f32;

    for _i in 0..steps {
        let state0 = get_model_state(&joint_query);
        let dstate0 = all_loops(
            &state0,
            &base_query,
            &joint_children_query,
            &mut joint_query,
        );

        let state = state0.clone() + dstate0.clone() * dt;
        set_model_state(&mut joint_query, &state);
    }
}
