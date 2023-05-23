# Dynamics Simulations

Dynamics simulations is a project that shows how a rigid body dynamics library can be used in order to create a driving simulation. This simulation uses Rust and the Rust based
game-dev engine Bevy in order to create a driving simulation. This simulation also allows for use of either keyboard or controller inputs.

# Version Features

### v0.3.0 - Bug fixes and making previous functionality better             

   
      


### v0.2.0 - Added controller support and the UI
    - Controller support for xbox controllers added. Check start guide for controller mappings
    - UI re-added after changing to Bevy v0.10


### v0.1.0 - Initial version
    - Added driving capabilities of the simulation
        - driven wheel system
        - gear shifting
        - steering
        - acceleration via input
    - Car Model added to assets
    - scheduler to untie the physics from the visuals

# Driving Simulation

Predeveloped driving simulation to allow users to see what is possible with the rigid  body dynamics algorithms library in combination with the game engine Bevy. This simple simulation
allows a "realistic" driving experience with how the suspension and movmenet physics are applied. 
# Start Guide

## Installation

In order to be able to run our simulation the following is needed:

### 1. Install Rust
Rust can be installed by following the instructions in their github repo: https://github.com/rust-lang/rust

### 2. Copy Repo
Clone this repo to where ever you want. Choose someplace that is easy to access.

### 3. Compile Dependencies
Inside driving_sim/cargo.toml there are a few dependencies that need to be compiled in order for the project to work such as the Bevy game engine. 
To compile these dependences, navagate to driving_sim and run the following:
```
cargo run --release
```
After a few minutes Rust will have compiled and connected to all the needed dependencies in order to run the project and a window will appear with the simulation running.

### 4. Running Project
In order to run the project just run: 
```
cargo run --release
```
If code is added this may take a few seconds to recompile and run but will create a new window containing the simulation.

## Controls
        
### Keyboard
        - A & D      - steering
        - W          - acceleration (gas pedal)
        - Q          - upshift
        - E          - downshift
        - T          - neutral
        - R          - reverse
        - Left shift - soft braking
        - Space      - hard breaking
### Gamepad

        - Left Trigger      - Increase Gear
        - Right Trigger     - Decrease Gear
        - Right Trigger 2   - Neutral
        - South Button (A)  - acceleration
        - East Button  (B)  -  Reverse
        - North Button (Y)  - Hard Break
        - West Button  (X)  -  Break Slow
        - Left Stick        - Turn left right

NOTE: Gamepad has only been tested with an Xbox style of controller. Results may vary if used with Playstation or Nintendo styled controllers

# Documentation

<<<<<<< HEAD
Documentation can be found via rust doc in  
=======
Documentation can be found in `car_demo/target/doc/car_demo`. We created documentation using rustdoc, which means that our documentation can be viewed as a webpage. Open `car_demo/target/doc/car_demo/index.html` with the browser of your choice to start at our documentation's homepage. You can navigate through all of our documentation from there.
>>>>>>> 2b742dc6ae3c38215f8f7f5d264d9fdd409a769d

# Repository

Our repository contains all of our work on this project and is taken from a larger group repository which we share with a few other groups. In order to get access to this repository please contact the email in the contributing section.


# Contributing

Those who would like to contribute to this simulation can contact Chris Patton at: chris@pattondynamics.com


# License

Not all resources in this repository are licensed as specified in LICENSE.txt. Some assets (such as fonts) in this repository were created by third parties and are redistributed under their respective licenses. Each such resource is packaged in a directory with the applicable license for that resource. For more information about the license, [click here](https://github.com/crispyDyne/simulation_games/blob/main/LICENSE).
