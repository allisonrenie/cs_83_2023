// //! This is our driving simulation. It's an example of how engineering simulations could be made
// //! with Rust and the Rust based game engine Bevy. See README.txt for more information on how
// //! to run and use. 

// use bevy::prelude::*;

// mod controller;

// /// Builds the car as a series of joints and draws meshes.
// mod build;
// //mod camera_az_el;

// /// Builds a basic environment with lighting.
// mod enviornment;

// /// Contains all driving simulation specific physics systems as well
// /// as systems to handle controller and keyboard input. Also contains
// /// systems to simulate an engine with a gear box.
// pub mod physics;

// /// Defines the physics schedule: the order in which all physics, controller, 
// /// and engine systems run in. 
// mod schedule;

// // camera_az_el::camera_builder;

// use build::build_model;
// use enviornment::build_environment;
// use physics::{controller_system};
// use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

// // use controller::gamepad_ordered_events;

// use rbda_bevy::build::bevy_joint_positions;
// use schedule::{create_schedule, physics_schedule, PhysicsSchedule};

// /// Set a larger timestep if the animation lags.
// const FIXED_TIMESTEP: f32 = 0.002; // 500 fps!!! ( and it can go faster! )

// // Fps Text Struct
// #[derive(Component)]
// struct FpsText;

// // RPM Text Struct
// #[derive(Component)]
// pub struct RPMText;

// // Speed Text Struct
// #[derive(Component)]
// pub struct SpeedText;

// /// Main function. Creates app, sets up window, specifies startup tasks (building 
// /// environment and car), sets fixed timestep, then adds physics schedule, controller
// /// system, and system that updates joint positions.
// fn main() {
//     /// Create the physics schedule
//     let schedule = create_schedule();
//     /// Create App
//     /// Sets up car model and environment, sets timestep, adds physics schedule, handles controller inputs, 
//     /// and then updates Bevy joint positions. 
//     App::new()
//         .add_plugins(DefaultPlugins.set(WindowPlugin {
//             primary_window: Some(Window {
//                 resolution: (1200., 900.).into(),
//                 title: "Linear".to_string(),
//                 resizable: true,
//                 ..default()
//             }),
//             ..default()
//         }))
//         .add_plugin(FrameTimeDiagnosticsPlugin::default())
//         //set up the car model and environment
//         .add_startup_system(setup_system)
//         // set the fixed timestep
//         .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
//         // add the physics schedule
//         .add_schedule(PhysicsSchedule, schedule) 
//         //handles controller inputs
//         .add_system(controller_system)
//         // run the physics schedule in the fixed timestep loop
//         .add_system(physics_schedule.in_schedule(CoreSchedule::FixedUpdate)) 
//         // update the bevy joint positions
//         .add_system(bevy_joint_positions) 
//         .add_system(fps_updater)
//         // .add_system(rpm_updater)
//         // .add_system(speed_updater)
//         .run();
// }

// /// Builds environment, model, and user interface.
// pub fn setup_system(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     assets: Res<AssetServer>,
// ) {
//     /*******************************************
//      * Name of Code Block: UI Code Block
//      * Function Purpose: Creates and updates the UI.
//      * Developer: Cameron Vandenberg
//      * Date Last Updated: 2-12-23
//      * Sources: Bevy cheatbook
//     *******************************************/

//     // Display the FPS to the screen. 
//     commands.spawn((
//         TextBundle::from_sections([
//             TextSection::new(
//                 "FPS: ",
//                 TextStyle {
//                     font: assets.load("fonts/FiraSans-Bold.ttf"),
//                     font_size: 30.0,
//                     color: Color::WHITE,
//                 },
//             ),
            
//             TextSection::from_style(TextStyle {
//                 font: assets.load("fonts/FiraSans-Regular.ttf"),
//                 font_size: 30.0,
//                 color: Color::GOLD,
//             }),
//         ])
//         //Alignment.
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             position: UiRect {
//                 top: Val::Px(25.0),
//                 left: Val::Px(5.0),
//                 ..default()
//             },
//             ..default()
//         }),
//         FpsText
//     ));

//      //Display RPM to the screen
//     //Cameron TODO: Map to image texture
//     commands.spawn((
//         TextBundle::from_sections([
//             TextSection::new(
//                 "RPM: ",
//                 TextStyle {
//                     font: assets.load("fonts/FiraSans-Bold.ttf"),
//                     font_size: 30.0,
//                     color: Color::WHITE,
//                 },
//             ),
            
//             TextSection::from_style(TextStyle {
//                 font: assets.load("fonts/FiraSans-Regular.ttf"),
//                 font_size: 30.0,
//                 color: Color::GOLD,
//             }),
//         ])
//         //Alignment
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             position: UiRect {
//                 bottom: Val::Px(15.0),
//                 left: Val::Px(5.0),
//                 ..default()
//             },
//             ..default()
//         }),
//         RPMText
//     ));

//     commands.spawn((
//         TextBundle::from_sections([
//             TextSection::new(
//                 " ",
//                 TextStyle {
//                     font: assets.load("fonts/FiraSans-Bold.ttf"),
//                     font_size: 30.0,
//                     color: Color::WHITE,
//                 },
//             ),
            
//             TextSection::from_style(TextStyle {
//                 font: assets.load("fonts/FiraSans-Regular.ttf"),
//                 font_size: 65.0,
//                 color: Color::rgb(0.5, 0.2, 0.2),
//             }),
//         ])
//         //Alignment
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             position: UiRect {
//                 bottom: Val::Px(82.0),
//                 right: Val::Px(257.0),
//                 ..default()
//             },
//             ..default()
//         }),
//         SpeedText
//     ));
    
//     //Display the Speed to the screen in units.
//     //Cameron TODO: Find out units
//     commands.spawn((
//         TextBundle::from_sections([
//             TextSection::new(
//                 " ",
//                 TextStyle {
//                     font: assets.load("fonts/FiraSans-Bold.ttf"),
//                     font_size: 30.0,
//                     color: Color::WHITE,
//                 },
//             ),
            
//             TextSection::from_style(TextStyle {
//                 font: assets.load("fonts/FiraSans-Regular.ttf"),
//                 font_size: 65.0,
//                 color: Color::rgb(246.0, 139.0, 192.0),
//             }),
//         ])
//         //Alignment
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             position: UiRect {
//                 bottom: Val::Px(85.0),
//                 right: Val::Px(260.0),
//                 ..default()
//             },
//             ..default()
//         }),
//         SpeedText
//     ));

//     let dash_icon = assets.load("techdash.png");
//     commands.spawn(NodeBundle{
//         style: Style{
//             size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
//             //justify_content: JustifyContent::Center,
//             position: UiRect {
//                 top: Val::Percent(76.0),
//                 left: Val::Percent(63.0),
//                 ..default()
//             },
//             ..default()
//         },
//         //background_color: Color::rgb(0.65, 0.65, 0.65).into(),
//         //image: dash_icon.clone().into(),
//         ..default()
        
//     })
//     .with_children(|commands| {
//         commands.spawn(ImageBundle{
//             //transform: Transform::from_scale(Vec3::new(1.5, 1.5, 0.0)),
//             image: dash_icon.clone().into(),
//             ..default()
//         });
//     }); 
//     build_environment(&mut commands, &mut meshes, &mut materials, &assets);
//     build_model(&mut commands, &mut meshes, &mut materials, assets);
    
// }

// /** *******************************************
//  * Function Name: create_ui
//  * Function Purpose: Creates and updates the UI
//  * Developer: Cameron Vandenberg
//  * Date Last Updated: 3-3-23
//  * Prerequisites:
//  * Parameters:
//  * Output:
//  * Sources: Bevy cheatbook
// ********************************************/

// fn create_ui(mut commands: Commands, assets: Res<AssetServer>) {
//     //let dash_icon = assets.load("techdash.png");
    
//     commands.spawn(NodeBundle{
//         style: Style{
//             size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
//             //justify_content: JustifyContent::Center,
//             position: UiRect {
//                 top: Val::Percent(75.0),
//                 left: Val::Percent(55.0),
//                 ..default()
//             },
//             ..default()
//         },
//         //background_color: Color::rgb(0.65, 0.65, 0.65).into(),
//         //image: dash_icon.clone().into(),
//         ..default()
        
//     })
//     .with_children(|commands| {
//         commands.spawn(ImageBundle{
//             //transform: Transform::from_scale(Vec3::new(1.5, 1.5, 0.0)),
//             //image: dash_icon.clone().into(),
//             ..default()
//         });
//     }); 
//     //.insert(component: InitialDUIRoot) &mut EntityCommands
//     //.with_children(|commands: &mut ChildBuilder| {
//     //    for i: usize in 0..3 {
//     //        commands
//     //    }
//     //})
// }

// /*******************************************
//  * Function Name: fps_updater
//  * Function Purpose: Updates the text which displays the current FPS.
//  * Developer: Cameron Vandenberg
//  * Date Last Updated: 1-13-23
//  * Prerequisites:
//  * Parameters:
//  * Output: the UI
//  * Sources: Bevy cheatbook
// *******************************************/
// fn fps_updater(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
//     for mut text in &mut query {
//         if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
//             if let Some(value) = fps.smoothed() {
//                 // Update the value of the second section
//                 text.sections[1].value = format!("{value:.2}");
//             }
//         }
//     }
// }

use bevy::prelude::*;

mod controller;
mod build;
mod camera_az_el;
mod enviornment;
pub mod physics;
mod schedule;

use camera_az_el::camera_builder;

use build::build_model;
use enviornment::build_environment;
use physics::controller_system;

// use controller::gamepad_ordered_events;

use rbda_bevy::build::bevy_joint_positions;
use schedule::{create_schedule, physics_schedule, PhysicsSchedule};

// set a larger timestep if the animation lags
const FIXED_TIMESTEP: f32 = 0.002; // 500 fps!!! ( and it can go faster! )

// Main function
fn main() {
    // Create the physics schedule
    let schedule = create_schedule();
    // Create App
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1200., 900.).into(),
                title: "Linear".to_string(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        // .add_startup_system(camera_builder(
        //     Vec3 {
        //         x: 0.,
        //         y: 10.,
        //         z: 1.,
        //     },
        //     0.0_f32.to_radians(),
        //     10.0_f32.to_radians(),
        //     20.,
        //     camera_az_el::UpDirection::Z,
        // ))
        // .add_system(camera_az_el::az_el_camera) // setup the camera
        .add_startup_system(setup_system) // setup the car model and environment
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP)) // set the fixed timestep
        .add_schedule(PhysicsSchedule, schedule) // add the physics schedule
        .add_system(controller_system)
        .add_system(physics_schedule.in_schedule(CoreSchedule::FixedUpdate)) // run the physics schedule in the fixed timestep loop
        .add_system(bevy_joint_positions) // update the bevy joint positions
        .run();
}

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    build_environment(&mut commands, &mut meshes, &mut materials, &assets);
    build_model(&mut commands, &mut meshes, &mut materials, assets);
}

