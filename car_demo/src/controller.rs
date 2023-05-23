use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadConnectionEvent, GamepadEvent,
    },
    prelude::*,
};

// #[derive(Component)]
// pub struct ControllerInputs{
//     buttons:GamepadButtonType,
//     axis:GamepadAxisType,
// }

// #[derive(Component)]
// pub struct Controllers;
// impl Plugin for Controllers{
//     fn build(&self, app: &mut App){
//         app.add_system(gamepad_ordered_events);
//     }
// }


// If you require in-frame relative event ordering, you can also read the `Gamepad` event
// stream directly. For standard use-cases, reading the events individually or using the
// // `Input<T>` or `Axis<T>` resources is preferable.
// pub fn gamepad_ordered_events(mut gamepad_events: EventReader<GamepadEvent>) {
//     for gamepad_event in gamepad_events.iter() {
//         match gamepad_event {
//             GamepadEvent::Connection(connection_event) => info!("{:?}", connection_event),
//             GamepadEvent::Button(button_event) => button_handler(button_event),
//             GamepadEvent::Axis(axis_event) => axis_handler(axis_event),
//         }
//     }
// }


