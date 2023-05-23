use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use rbda::sva::{Inertia, Matrix, Motion, Vector, Xform};
use rbda_bevy::build::{Base, Joint};

use crate::physics::{DrivenWheel, Steering, Suspension, TireContact, Engine};


// #[derive(Component)]
// pub struct FollowCamera
// {
// }

/// Builds the car model, utilizing other functions to create the chassis, suspensions, and wheels. 
pub fn build_model(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    // create base
    let base = Joint::base(Motion::new([0., 0., 9.81], [0., 0., 0.]));
    let base_id = commands
        .spawn(base)
        .insert(Base)
        .insert(SpatialBundle {
            // sets visibility and position of the base
            ..Default::default()
        })
        .id();

    // chassis
    let dimensions = [3.0_f32, 1.5, 0.4]; // approximate dimensions of a car
    let (chassis_id, rotation_id) = build_chassis(commands, &assets, dimensions, base_id);

    let translation = Vec3::new(-15.0, 0.0, 5.0);
    let look_at = Vec3::new(20.0, 0.0, 0.0);
    
    let camera_id = commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation)
                .looking_at(look_at, Vec3::Z),
            ..Default::default()
        },
    ))
    .id();

    commands.entity(rotation_id).push_children(&[camera_id]);

    // create suspension and wheels
    let corner_locations = [
        [1.325, 0.75, 0.],  // fl
        [1.325, -0.75, 0.], // fr
        [-1.25, 0.75, 0.],  // rl
        [-1.25, -0.75, 0.], // rr
    ];
    let corner_names = ["fl", "fr", "rl", "rr"];
    let mut parent_id: Entity;
    let mut suspension_location: [f32; 3];
    let mut driven_wheel: bool;
    // loop through corners and build suspension, steering, and wheels
    for (ind, location) in corner_locations.iter().enumerate() {
        if ind < 2 {
            // add steering to front wheels
            let steering_id = build_steer(commands, *location, chassis_id, corner_names[ind]);
            parent_id = steering_id; // suspension is attached to steering
            suspension_location = [0., 0., 0.]; // location of suspension relative to steering
            driven_wheel = false;
        } else {
            parent_id = chassis_id; // suspension is attached to chassis
            suspension_location = *location; // location of suspension relative to chassis
            driven_wheel = true;
        }
        // add suspension and wheel
        let id_susp = build_suspension(commands, suspension_location, parent_id, corner_names[ind]);
        build_wheel(
            commands,
            meshes,
            materials,
            &assets,
            id_susp,
            driven_wheel,
            corner_names[ind],
        );
    }
}

/// Builds the chassis from a series of joints.
fn build_chassis(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    dimensions: [f32; 3],
    parent_id: Entity,
) -> (Entity, Entity) {
    // x degree of freedom (absolute coordinate system, not relative to car)
    let mut px = Joint::px(Inertia::zero(), Xform::identity());
    px.name = "chassis_px".to_string();
    let mut px_e = commands.spawn(px);
    px_e.insert(SpatialBundle {
        ..Default::default()
    });
    px_e.set_parent(parent_id);
    let px_id = px_e.id();

    // y degree of freedom (absolute coordinate system, not relative to car)
    let mut py = Joint::py(Inertia::zero(), Xform::identity());
    py.name = "chassis_py".to_string();
    let mut py_e = commands.spawn(py);
    py_e.insert(SpatialBundle {
        ..Default::default()
    });
    py_e.set_parent(px_id);
    let py_id = py_e.id();

    // z degree of freedom (always points "up", relative to absolute coordinate system)
    let mut pz = Joint::pz(Inertia::zero(), Xform::identity());
    pz.name = "chassis_pz".to_string();
    pz.q = 0.3 + 0.25; // start the car above the ground (this should be done somewhere else)
    let mut pz_e = commands.spawn(pz);
    pz_e.insert(SpatialBundle {
        ..Default::default()
    });
    pz_e.set_parent(py_id);
    let pz_id = pz_e.id();

    // yaw degree of freedom (rotation around z axis)
    let mut rz = Joint::rz(Inertia::zero(), Xform::identity());
    rz.name = "chassis_rz".to_string();
    let mut rz_e = commands.spawn(rz);
    rz_e.insert(SpatialBundle {
        ..Default::default()
    });
    rz_e.set_parent(pz_id);
    let rz_id = rz_e.id();

    // pitch degree of freedom (rotation around y axis)
    let mut ry = Joint::ry(Inertia::zero(), Xform::identity());
    ry.name = "chassis_ry".to_string();
    let mut ry_e = commands.spawn(ry);
    ry_e.insert(SpatialBundle {
        ..Default::default()
    });
    ry_e.set_parent(rz_id);
    let ry_id = ry_e.id();

    // roll degree of freedom (rotation around x axis)
    // this is the body of the car!
    let mass = 1000.; // 1000kg ~2200lbs
    let cg_position = [0., 0., 0.]; // center of gravity position
    let inertia = Inertia::new(
        mass,
        Vector::new(cg_position[0], cg_position[1], cg_position[2]),
        mass * (1. / 12.)
            //next meeting: do we need to substitute this with value to supplement 3D model?
            * Matrix::from_diagonal(&Vector::new(
                dimensions[1].powi(2) + dimensions[2].powi(2),
                dimensions[2].powi(2) + dimensions[0].powi(2),
                dimensions[0].powi(2) + dimensions[1].powi(2),
            )),
    );

    let mut rx = Joint::rx(inertia, Xform::identity());
    rx.name = "chassis_rx".to_string();
    let mut rx_e = commands.spawn(rx);
    rx_e.set_parent(ry_id);
    rx_e.insert(SpatialBundle {
        ..Default::default()
    });
    let rx_id = rx_e.id();
    add_car_mesh(commands, rx_id, assets);

    // return id the last joint in the chain. It will be the parent of the suspension / wheels
    (rx_id, rz_id)
}

/// Builds steering system. Similar to build_suspension, but with an rz joint, and no mesh and no contact.
fn build_steer(
    commands: &mut Commands,
    location: [f32; 3],
    parent_id: Entity,
    name: &str,
) -> Entity {
    let xt = Xform::new(
        Vector::new(location[0], location[1], location[2]),
        Matrix::identity(),
    );

    // create steering joint
    let mut steer = Joint::rz(Inertia::zero(), xt);
    steer.name = ("steer_".to_owned() + name).to_string();

    // create steering entity
    let mut steer_e = commands.spawn(steer);
    steer_e
        .insert(SpatialBundle {
            ..Default::default()
        })
        .insert(Steering);

    // set parent
    let steering_id = steer_e.id();
    commands.entity(parent_id).push_children(&[steering_id]);

    // return steering entity id
    steering_id
}

/// Builds suspension.
fn build_suspension(
    commands: &mut Commands,
    location: [f32; 3],
    parent_id: Entity,
    name: &str,
) -> Entity {
    let xt = Xform::new(
        Vector::new(location[0], location[1], location[2] - 0.3), // location of suspension relative to chassis
        Matrix::identity(),
    );
    // suspension mass
    let suspension_mass = 10.;
    let inertia = Inertia::new(
        suspension_mass,
        Vector::new(0., 0., 0.), // center of mass
        (2. / 3.) * suspension_mass * 0.25_f32.powi(2) * Matrix::identity(), // inertia
    );

    // create suspension joint
    let mut susp = Joint::pz(inertia, xt);
    susp.name = ("susp_".to_owned() + name).to_string();

    // suspension parameters
    let stiffness: f32 = 1000. * 9.81 / 4. / 0.1; // weight / 4 / spring travel
    let damping = 0.5 * 2. * (stiffness * (1000. / 4.)).sqrt(); // some fraction of critical damping

    // create suspension entity
    let mut susp_e = commands.spawn(susp);
    susp_e.insert(Suspension::new(stiffness, damping));
    susp_e.insert(SpatialBundle {
        ..Default::default()
    });
    susp_e.set_parent(parent_id);

    susp_e.id()
}

/// Builds the wheels. 
fn build_wheel(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &Res<AssetServer>,
    parent_id: Entity,
    driven: bool,
    name: &str,
) -> Entity {
    let wheel_mass = 10.;
    let moi_xz = 1. / 12. * wheel_mass * (3. * 0.25_f32.powi(2));
    let moi_y = wheel_mass * 0.25_f32.powi(2);
    let inertia = Inertia::new(
        wheel_mass,
        Vector::new(0., 0., 0.),
        wheel_mass * Matrix::from_diagonal(&Vector::new(moi_xz, moi_y, moi_xz)),
    );
    let mut ry = Joint::ry(inertia, Xform::identity());
    ry.name = ("wheel_".to_owned() + name).to_string();

    //set tire according to name label
    let mut wheel_e = commands.spawn(ry);

    wheel_e.set_parent(parent_id);
    wheel_e.insert(SpatialBundle {
        ..Default::default()
    });
    add_tire_contact(&mut wheel_e);
    add_engine(&mut wheel_e); //adding in the engine here for now

    if driven {
        wheel_e.insert(DrivenWheel);
    }

    let wheel_id = wheel_e.id();
    match name {
        "fl" => add_wheel_scene(
            "AE86_FRONT_LEFT.glb#Scene0".to_string(),
            commands,
            wheel_id,
            assets,
            1,
        ),
        "fr" => add_wheel_scene(
            "AE86_FRONT_RIGHT.glb#Scene0".to_string(),
            commands,
            wheel_id,
            assets,
            -1,
        ),
        "rl" => add_wheel_scene(
            "AE86_REAR_LEFT.glb#Scene0".to_string(),
            commands,
            wheel_id,
            assets,
            1,
        ),
        "rr" => add_wheel_scene(
            "AE86_REAR_RIGHT.glb#Scene0".to_string(),
            commands,
            wheel_id,
            assets,
            -1,
        ),
        _ => (),
    }

    wheel_id
}

/// Adds a tire contact to an entity.
fn add_tire_contact(entity: &mut EntityCommands) {
    let stiffness = 1000. * 9.81 / 4. / 0.005;
    let damping = 0.25 * 2. * (1000.0_f32 / 4. * stiffness).sqrt();
    entity.insert(TireContact::new(0.325, stiffness, damping, 0.2, 0.5));
}

/// Basic add engine function that adds an engine to an entity.
fn add_engine(entity: &mut EntityCommands)
{
    entity.insert(Engine::new(2000.0, 0.0, 0.0));
}

/// Adds car mesh. 
fn add_car_mesh(commands: &mut Commands, chassis_joint_id: Entity, assets: &Res<AssetServer>) {
    let car_mesh = assets.load("AE86_BODY.glb#Scene0");
    let mut car_mesh_entity = commands.spawn(SceneBundle {
        scene: car_mesh,
        transform: Transform::from_xyz(0.1, 0.0, -0.2)
            .with_scale(Vec3::new(0.25, 0.25, 0.25))
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                (0.0_f32).to_radians(),
                (0.0_f32).to_radians(),
                (0.0_f32).to_radians(),
            )),
        ..default()
    });
    car_mesh_entity.set_parent(chassis_joint_id);
}

/// Adds wheel meshes.
fn add_wheel_scene(
    mesh_name: String,
    commands: &mut Commands,
    wheel_joint_id: Entity,
    assets: &Res<AssetServer>,
    side: i32,
) {
    let wheel_mesh = assets.load(mesh_name);
    let mut wheel_mesh_entity = commands.spawn(SceneBundle {
        scene: wheel_mesh,
        transform: Transform::from_xyz(0.0, -0.15 * side as f32, 0.0)
            .with_scale(Vec3::new(0.25, 0.25, 0.25))
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                (0.0_f32).to_radians(),
                (0.0_f32).to_radians(),
                (0.0_f32).to_radians(),
            )),
        ..default()
    });

    wheel_mesh_entity.set_parent(wheel_joint_id);
}
