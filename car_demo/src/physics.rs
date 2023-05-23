/*
*  CONTROLLER BUTTON MAPPING:
*       
        Left Trigger - Shift Up
        Right Trigger - Shift Down
*       Left Trigger 2 - 
*       Right Trigger 2 - Neutral
*       South Button - speed up
*       East Button -  Reverse
*       North Button - Hard Break
*       West Button -  Break Slow
        Left Stick - Turn car
*
 */



/*
*   KEYBOARD BUTTON MAPPING:
*       
        W - Speed up
        A - Turn left
*       D - Turn Right
*       Space - Hard Break
*       LShift - Slow Break
*       Q - Increase Gear
*       E - Decrease Gear
*       R -  Reverse
        T - Neutral
*
 */

use std::f32::consts::PI;

// use bevy::prelude::*;
use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadConnectionEvent, GamepadEvent, GamepadButton,
    },
    prelude::*,
};


use rbda::sva::{Force, Vector};
use rbda_bevy::build::{Joint};

#[derive(Component)]
pub struct Suspension {
    stiffness: f32,
    damping: f32,
}

impl Suspension {
    pub fn new(stiffness: f32, damping: f32) -> Self {
        Self { stiffness, damping }
    }
}

pub fn suspension_system(mut joints: Query<(&mut Joint, &Suspension)>) {
    for (mut joint, suspension) in joints.iter_mut() {
        joint.tau -= suspension.stiffness * joint.q + suspension.damping * joint.qd;
    }
}

#[derive(Component)]
pub struct TireContact {
    radius: f32,
    stiffness: f32,
    damping: f32,
    longitudinal_stiffness: f32,
    lateral_stiffness: f32,
}

impl TireContact {
    pub fn new(
        radius: f32,
        stiffness: f32,
        damping: f32,
        longitudinal_stiffness: f32,
        lateral_stiffness: f32,
    ) -> Self {
        Self {
            radius,
            stiffness,
            damping,
            longitudinal_stiffness,
            lateral_stiffness,
        }
    }
}

#[derive(Component)]
pub struct Engine 
{
    rpms: f32,
    gear: f32,
    breaking: f32,
}

impl Engine {
    pub fn new(
        rpms: f32,
        gear: f32,
        breaking: f32,
    ) -> Self {
        Self {
            rpms,
            gear,
            breaking,
        }
    }
}

#[derive(Component)]
pub struct Controller;
pub fn controller_system(
    mut joints: Query<(&mut Joint, &DrivenWheel,&mut Engine )>,
    key: Res<Input<KeyCode>>,keys: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>, 
    button_inputs: Res<Input<GamepadButton>>){

    //forward motion
    if keys.pressed(KeyCode::W)
    {
        for (_, _, mut engine) in joints.iter_mut()
        {
            if engine.rpms < 7300.0{
                engine.rpms += 5.0; //this may need to change
            }
        }
         
    }
    else
    {
        for (_, _, mut engine) in joints.iter_mut(){
            if engine.rpms > 0.0{
                engine.rpms -= 5.0; //this may need to change
            }
        }
    }

    //gamepad forward motion
    for gamepad in gamepads.iter(){
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::South)){
            for (_, _, mut engine) in joints.iter_mut(){
                if engine.rpms < 7300.0{
                    engine.rpms += 5.0; //this may need to change
                }
            }
        }else{
            for (_, _, mut engine) in joints.iter_mut(){
                if engine.rpms > 0.0{
                    engine.rpms -= 5.0; //this may need to change
                }
            }
            
        }
    }


    //gear box

    if keys.just_released(KeyCode::Q)
    {
        println!("Pressed Q");
        for (_, _, mut engine) in joints.iter_mut()
        {
            if engine.gear < 5.0
            {
                engine.gear = engine.gear + 1.0;
            }
        }
    //downshift
    }
    else if keys.just_released(KeyCode::E)
    {
        for (_, _, mut engine) in joints.iter_mut()
        {
            println!("Pressed E");
            if engine.gear > 0.0
            {
                engine.gear = engine.gear - 1.0;
            }
        }
    //reverse
    }
    else if  keys.just_released(KeyCode::R)
    {
        println!("Pressed R");
        for (_, _, mut engine) in joints.iter_mut()
        {
            engine.gear = -1.0;
        }
    //neutral
    }
    else if  keys.just_released(KeyCode::T)
    {
        println!("Pressed T");
        for (_, _, mut engine) in joints.iter_mut()
        {
            engine.gear = 0.0;
        }
    }

    //gamepad gear box
    for gamepad in gamepads.iter(){
        if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger)){
            for (_, _, mut engine) in joints.iter_mut()
            {
                if engine.gear < 5.0
                {
                    engine.gear = engine.gear + 1.0;
                }
            }
        }else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger)){
            for (_, _, mut engine) in joints.iter_mut()
            {
                if engine.gear > 0.0
                {
                    engine.gear = engine.gear - 1.0;
                }
            }
        }else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::East)){
            for (_, _, mut engine) in joints.iter_mut()
            {
                engine.gear = -1.0;
            }
        }else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger2)){
            for (_, _, mut engine) in joints.iter_mut()
            {
                engine.gear = 0.0;
            }
        }
    }

}

// a very simple tire model. Not very realistic, but it works well enough for this demo.
// it's also messy, but I/we can clean it up later
pub fn tire_contact_system(mut joints: Query<(&mut Joint, &TireContact)>) {
    for (mut joint, contact) in joints.iter_mut() {
        // x0 is the inverse of the joint transform (used several times later)
        let x0 = joint.x.inverse();

        // we care about two reference frames: the contact and the tire
        // the y axis of the tire reference frame is the axis of rotation of the tire
        // the y axis of the contact reference frame is the axis of rotation of the tire projected onto the ground (no z component)
        // the z axis of the contact reference frame is the up vector in absolute coordinates
        // the x axis of the contact reference frame is the cross product of the y and z axes
        // the x axis of the tire reference frame is the same as the x axis of the contact reference frame
        // the z axis of the tire reference frame is the cross product of the x and y axes

        // each of the reference frames can be written in absolute coordinates or local coordinates

        // y axis of tire reference frame in local coordinates
        let tire_lat_local = Vector::new(0., 1., 0.); // axis of tire rotation

        // y axis of contact reference frame in absolute coordinates
        let mut contact_lat_abs = x0 * tire_lat_local;
        contact_lat_abs.z = 0.; // axis of tire rotation projected onto the ground (no z component)
        let contact_lat_abs = contact_lat_abs.normalize();

        // z axis of contact reference frame in absolute coordinates
        let contact_up_abs = Vector::new(0., 0., 1.); // up vector in absolute coordinates

        // x axis of contact reference frame in absolute coordinates
        let contact_forward_abs = contact_lat_abs.cross(&contact_up_abs).normalize();

        // transform the x axis of the contact reference frame into local coordinates
        let contact_forward_local = joint.x * contact_forward_abs;

        // x axis of tire reference frame in local coordinates
        let tire_forward_local = contact_forward_local;
        // z axis of tire reference frame in local coordinates
        let tire_up_local = tire_forward_local.cross(&tire_lat_local).normalize();

        // calculate the lowest point of the tire in local coordinates
        let contact_local = -contact.radius * tire_up_local; // move down by the radius of the tire

        // find the position of the lowest point of the tire in absolute coordinates
        let contact_abs = x0.transform_point(contact_local); // transform the point into world coordinates
        let height = contact_abs.z; // take the z coordinate of the point

        // if the tire is in contact with the ground
        if height < 0. {
            // vertical forces
            let spring_force = -contact.stiffness * height;

            let v0 = x0 * joint.v;
            let vel_abs = v0.velocity_point(contact_abs);
            let damping_force = -contact.damping * vel_abs.vel.z;
            let vertical_force = spring_force + damping_force;

            // ground plane forces
            let forward_vel = vel_abs.vel.dot(&contact_forward_abs); // component of velocity in the forward direction
            let lat_vel = vel_abs.vel.dot(&contact_lat_abs); // component of velocity in the lateral direction
            let forward_force = -forward_vel * contact.longitudinal_stiffness * vertical_force;
            let lat_force = -lat_vel * contact.lateral_stiffness * vertical_force;
            let f_abs = Force::force_point(
                forward_force * contact_forward_abs
                    + lat_force * contact_lat_abs
                    + Vector::new(0., 0., vertical_force),
                contact_abs,
            );
            joint.f_ext += f_abs;
        }
    }
}

#[derive(Component)]
pub struct Steering;
pub fn steering_system(keys: Res<Input<KeyCode>>, mut joints: Query<(&mut Joint, &Steering)>, time: Res<Time>, gamepads: Res<Gamepads>, button_axes: Res<Axis<GamepadButton>>,axes: Res<Axis<GamepadAxis>>) {
    
    /* original  */
    // let t = time.elapsed_seconds();
    // let steer_angle = 15.0_f32.to_radians() + 10.0_f32.to_radians() * (2. * PI * t).sin();

    /* new -allison */
    let mut steer_angle = 0.;
    if keys.pressed(KeyCode::A)
    {
        steer_angle += 0.3;
    }
    else if keys.pressed(KeyCode::D)
    {
        steer_angle -= 0.3;
    }

    for gamepad in gamepads.iter(){

        let left_stick_x = axes.get(GamepadAxis::new(gamepad,GamepadAxisType::LeftStickX)).unwrap();

        if left_stick_x > 0.01{
            steer_angle -= 0.3;
        } else if left_stick_x < -0.01{
            steer_angle += 0.3;
        }
    }


    for (mut joint, _) in joints.iter_mut() {
        joint.q = steer_angle;
    }


}

#[derive(Component)]
pub struct Breaking;
pub fn breaking_system(mut joints: Query<(&mut Joint, &DrivenWheel)>, key: Res<Input<KeyCode>>,keys: Res<Input<KeyCode>>,gamepads: Res<Gamepads>, button_inputs: Res<Input<GamepadButton>>){

    let mut breaking = 0.0;
    if key.pressed(KeyCode::Space)
    {
        println!("Pressed Space");
        breaking = 100.0;
    }else if keys.pressed(KeyCode::LShift)
    {
        println!("Pressed LShift");
        breaking = 0.5;
    }


    
    for gamepad in gamepads.iter(){
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::North)){
            breaking = 100.0;
        } else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::West)){
           breaking = 0.5;
        }
    }

    for (mut joint, _) in joints.iter_mut() {
        if breaking == 100.0{
            joint.qd = 0.0;
        }else if joint.qd > 0.0{
            joint.qd -= breaking;
        }
    }



}



#[derive(Component)]
pub struct DrivenWheel;
pub fn driven_wheel_system(mut joints: Query<(&mut Joint, &DrivenWheel, &mut Engine)>) {

    /* added my me below - allison */
    let mut speed_target = 0.0;
    //let mut curr_rpms = 0.0;

    for (mut joint, _, mut engine) in joints.iter_mut()
    {  
        let mut engine_torque = 0.0;
        //this is all specific to the car we are using
        //getting torque
        match engine.rpms
        {
            0.0 => engine_torque = 0.0,
            1.0..=2999.0 => engine_torque = 85.0,
            3000.0..=3999.0 =>  engine_torque = 92.0,
            4000.0..=4999.0 =>  engine_torque = 98.0,
            5000.0..=5999.0 =>  engine_torque = 105.0,
            6000.0..=6999.0 =>  engine_torque = 109.0,
            7000.0..=7999.0 =>  engine_torque = 100.0,
            _ => engine_torque = 0.0,
        }

        //getting gear ratio
        let mut gear_ratio = 0.0;
        match engine.gear
        {
            1.0=> gear_ratio = 3.587,
            2.0=>  gear_ratio = 2.022,
            3.0=>  gear_ratio = 1.384,
            4.0=>  gear_ratio = 1.000,
            5.0=>  gear_ratio = 0.861,
            -1.0=> gear_ratio = -3.587,
            0.0 => gear_ratio = 0.000,
            _=>  gear_ratio = gear_ratio,
        }

        //added constant to manage speed target. may change with camera
        //this is sort of like a differential 
        let mut differential = 4.0; //CHANGE THIS to make the car more rective to speed, higher=less reactive/less fast
        speed_target = engine_torque/gear_ratio/differential; //speed being set by torque/gear ratio
        if engine.gear < 1.0
        {
            speed_target = 0.0;
        }
        if engine.gear == -1.0
        {
            speed_target *= -1.0;
        }

        //CHANGE THIS if you do not want print outs
        println!("gear: {0} speed target: {1} rpms: {2} joint.qd: {3}\n", engine.gear, speed_target, engine.rpms, joint.qd);

        
    
        let rad_per_seconds_target = speed_target / 0.25; // 25 m/s * 4 ticks per second
        let error_gain = 5.0; // 5 Nm per rad/s error

        let rad_per_second_error = rad_per_seconds_target - joint.qd;
        joint.tau += error_gain * rad_per_second_error; // add a torque to the joint to correct the speed error
        
    }
    
}
