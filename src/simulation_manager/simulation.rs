use std::f32::consts::PI;
use std::ops::Mul;
use uom::si::f32::{Acceleration, Angle, AngularVelocity, Area, Force, Length, Mass, Pressure, Time, Velocity};
use uom::num::pow;
use uom::si::time::second;
use linspace::{Linspace, Linspaced};
use crate::hypot;
use crate::simulation_manager::types::{SimulationConfig, SimulationResult};

pub struct Simulation {
  wheel_tangential_speed: Velocity,
  ball_initial_speed: Velocity,
  ball_angular_speed: AngularVelocity,
  initial_horizontal_speed: Velocity,
  initial_vertical_speed: Velocity,
  
  cross_sectional_area: Area,
  
  config: SimulationConfig,
  wheel_speed: AngularVelocity,
  time: Linspaced<f32>,
  dt: f32,
}

impl Simulation {
  pub fn new(wheel_speed: AngularVelocity, config: SimulationConfig) -> Simulation {
    let wheel_tangential_speed: Velocity = wheel_speed * config.wheel_radius.mul(2.0).mul(PI);
    let ball_initial_speed: Velocity = wheel_tangential_speed * config.rotation_coefficient;
    let ball_angular_speed: AngularVelocity = wheel_tangential_speed / config.ball_radius * (1.0 - config.rotation_coefficient);
    let initial_horizontal_speed: Velocity = ball_initial_speed * config.launch_angle.cos();
    let initial_vertical_speed: Velocity = ball_initial_speed * config.launch_angle.sin();
    
    let cross_sectional_area: Area = config.ball_radius * pow(PI, 2);
    
    let end_time: f32 = config.simulation_time.get::<second>();
    let time = (0.0f32..end_time).linspace(config.iterations);
    
    let dt = config.simulation_time / config.iterations;
    
    
    Simulation {
      wheel_tangential_speed,
      ball_initial_speed,
      ball_angular_speed,
      initial_horizontal_speed,
      initial_vertical_speed,
      
      cross_sectional_area,
      
      config,
      wheel_speed,
      time,
      
      dt
    }
  }
  
  fn calculate_ideal_vertical_velocity(&self, t: f32) -> Velocity {
    self.initial_vertical_speed - self.config.gravity * t
  }
  
  fn calculate_ideal_vertical_position(&self, t: f32) -> Length {
    let ideal_vertical_velocity = self.calculate_ideal_vertical_velocity(t);
    let n: Length = self.config.launch_height + ideal_vertical_velocity * t + (self.initial_vertical_speed - ideal_vertical_velocity) * t / 2;
    n.max(Length::new(0.0))
  }
  
  fn calculate_ideal_height(&self, t: f32) -> Length {
    self.calculate_ideal_vertical_position(t)
  }
  
  fn calculate_ideal_horizontal_distance(&self, t: f32) -> Length {
    self.initial_horizontal_speed * t
  }
  
  pub fn run_ideal(&self) -> SimulationResult {
    let mut ideal_heights: Vec<Length> = vec![];
    let mut ideal_distances: Vec<Length> = vec![];
    for t in self.time {
      let height = self.calculate_ideal_height(t);
      let distance = self.calculate_ideal_horizontal_distance(t);
      ideal_heights.push(height);
      ideal_distances.push(distance);
      if height <= Length::new(0.0) {
        break;
      }
    }
    
    let time = self.time.into_iter().map(|value| Time::new::<second>(value)).collect::<Vec<Time>>();
    
    SimulationResult {
      x: ideal_distances,
      y: ideal_heights,
      time
    }
  }
  
  fn drag_acceleration(&self, speed: Velocity) -> Velocity {
    let drag_force = 0.5 * self.config.drag_coefficient * self.config.air_density * self.cross_sectional_area * pow(speed, 2);
    drag_force / self.config.ball_mass
  }
  
  fn lift_acceleration(&self, speed: Velocity) -> Velocity {
    if speed < Velocity::new(0.0) || self.config.lift_coefficient == 0.0 {
      return Velocity::new(0.0)
    }
    let lift_force: Force = 0.5 * self.config.air_density * self.cross_sectional_area * self.config.ball_radius * self.ball_angular_speed * speed * self.config.lift_coefficient;
    lift_force / self.config.ball_mass
  }
  
  pub fn run_friction(&self) -> SimulationResult {
    let mut heights: Vec<Length> = vec![];
    let mut distances: Vec<Length> = vec![];
    let mut current_horizontal_velocity = self.initial_horizontal_speed;
    let mut current_vertical_velocity = self.initial_vertical_speed;
    let mut current_height = self.config.launch_height;
    let mut current_distance = Length::new(0.0);
    for t in self.time {
      let speed: Velocity = hypot!(current_horizontal_velocity, current_vertical_velocity);
      if speed > 0 {
        let drag_accel = self.drag_acceleration(speed);
        let lift_accel = self.lift_acceleration(speed);
        let angle = current_vertical_velocity.atan2(current_horizontal_velocity);
        let drag_x: Acceleration = angle.cos().mul(-1) * drag_accel * self.dt;
        let drag_z: Acceleration = angle.sin().mul(-1) * drag_accel * self.dt;
        let lift_x = (angle + Angle::HALF_TURN / 2).cos() * lift_accel * self.dt;
        let lift_z = (angle + Angle::HALF_TURN / 2).sin() * lift_accel * self.dt;
        current_horizontal_velocity += drag_x + lift_x;
        current_vertical_velocity += drag_z + lift_z - self.config.gravity * self.dt;
      } else {
        current_vertical_velocity -= self.config.gravity * self.dt;
      }
      current_distance += current_horizontal_velocity * self.dt;
      current_height += current_vertical_velocity * self.dt;
      current_height = current_height.max(0);
      heights.push(current_height);
      distances.push(current_distance);
      if current_height <= Length::new(0.0) {
        break;
      }
    }
    
    let time = self.time.into_iter().map(|value| Time::new::<second>(value)).collect::<Vec<Time>>();
    
    SimulationResult {
      x: distances,
      y: heights,
      time
    }
  }
}