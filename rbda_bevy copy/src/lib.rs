pub mod build;
pub mod simulate;
pub mod solvers;
pub mod state;
pub mod structure;
pub mod system;

use bevy::{prelude::*, time::Stopwatch};
use rbda::model::Model;

#[derive(Component, Debug)]
pub struct Joint;

#[derive(Component, Debug)]
pub struct ModelCmp {
    pub time: Stopwatch,
    pub model: Model,
    pub joints: Vec<Entity>,
}
