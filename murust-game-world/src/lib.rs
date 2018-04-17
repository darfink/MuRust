#[macro_use]
extern crate specs;
extern crate murust_data_model;

use self::components::Movement;
use self::systems::{MovementSystem, MovementPostSystem};

mod components;
mod systems;
