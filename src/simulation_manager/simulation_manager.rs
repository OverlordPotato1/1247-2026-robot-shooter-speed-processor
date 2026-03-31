use std::collections::HashMap;
extern crate uom;
use uom::si::f32::*;
use crate::simulation_manager::simulation::{Simulation};
use crate::simulation_manager::types::{SimulationConfig, SimulationResult};

#[derive(Copy, Clone)]
pub struct SimulationManager {
  results: HashMap<AngularVelocity, SimulationResult>,
  sim_config: SimulationConfig,
}

impl SimulationManager {
  pub fn new() -> SimulationManager {
    SimulationManager {
      results: HashMap::new(),
      sim_config: SimulationConfig {
        launch_angle: Default::default(),
        launch_height: Default::default(),
        drag_coefficient: 0.0,
        lift_coefficient: 0.0,
        rotation_coefficient: 0.0,
        wheel_radius: Default::default(),
        ball_radius: Default::default(),
        gravity: Default::default(),
        ball_mass: Default::default(),
        air_density: Default::default(),
        simulation_time: Default::default(),
        iterations: 0,
      }
    }
  }
  
  pub fn update_config(&mut self, config: SimulationConfig) -> bool {
    if config == self.sim_config { return false; }
    self.sim_config = config;
    self.results = HashMap::new();
    true
  }
  
  pub fn query_speed(&mut self, wheel_speed: AngularVelocity) -> SimulationResult {
    let mut sim_results = self.results.get(&wheel_speed);
    if sim_results.is_none() {
      sim_results = Some(&self.compute_rpm(wheel_speed));
      self.results.insert(wheel_speed, *sim_results.unwrap());
    }
    *sim_results.unwrap()
  }
  
  fn compute_rpm(&self, wheel_speed: AngularVelocity) -> SimulationResult {
    let simulation = Simulation::new(wheel_speed, self.sim_config);
    simulation.run_friction()
  }
}

