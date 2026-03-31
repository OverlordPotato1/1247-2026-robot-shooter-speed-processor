use uom::si::f32::{Angle, Length, Mass, MassDensity, Time, Velocity};

#[derive(Copy, Clone)]
pub struct SimulationResult {
  pub x: Vec<Length>,
  pub y: Vec<Length>,
  pub time: Vec<Time>,
}

#[derive(Copy, Clone, PartialEq)]
pub struct SimulationConfig {
  pub launch_angle: Angle,
  pub launch_height: Length,
  pub drag_coefficient: f32,
  pub lift_coefficient: f32,
  pub rotation_coefficient: f32,
  pub wheel_radius: Length,
  
  pub ball_radius: Length,
  pub gravity: Velocity,
  pub ball_mass: Mass,
  pub air_density: MassDensity,
  
  pub simulation_time: Time,
  pub iterations: usize,
}