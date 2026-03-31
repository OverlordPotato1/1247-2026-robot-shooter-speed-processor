use std::collections::HashMap;
use uom::num::Zero;
use uom::si::f32::Length;
use uom::si::f32::AngularVelocity;
use crate::search_manager::types::DiscriminatedResult;
use crate::simulation_manager::simulation_manager::SimulationManager;
use crate::simulation_manager::types::{SimulationConfig, SimulationResult};

pub struct SearchManager {
  results: HashMap<Length, AngularVelocity>,
  sim_manager: SimulationManager,
  max_speed: AngularVelocity
}

impl SearchManager {
  pub fn new() -> SearchManager {
    let manager = SimulationManager::new();
    SearchManager {
      results: HashMap::new(),
      sim_manager: manager,
      max_speed: Default::default()
    }
  }
  
  pub fn update_config(&mut self, config: SimulationConfig) {
    let updated = self.sim_manager.update_config(config);
    if !updated { return; }
    self.results = HashMap::new();
  }
  
  pub fn update_max_speed(&mut self, max_speed: AngularVelocity) {
    self.max_speed = max_speed;
  }
  
  pub fn query_distance(&mut self, distance: Length, min_height: Length, max_height: Length) -> AngularVelocity {
    if self.results.get(&distance).is_some() {
      return *self.results.get(&distance).unwrap();
    }
    
    let max_distance_result = self.get_max_distance();
    let discrimiated = self.discriminate_height(self.get_result_height_at_distance(max_distance_result, distance), min_height, max_height);
    if matches!(discrimiated, DiscriminatedResult::Low) { self.results.insert(distance, AngularVelocity::new(-1.0)); }
    else { self.results.insert(distance, self.bsearch_step(distance, min_height, max_height, AngularVelocity::zero(), self.max_speed)); }
    
    
    *self.results.get(&distance).unwrap()    
  }
  
  fn bsearch_step(&self, distance: Length, min_height: Length, max_height: Length, min_speed: AngularVelocity, max_speed: AngularVelocity) -> AngularVelocity {
    let speed: AngularVelocity = (min_speed + max_speed) / 2;
    let result = self.get_discriminated_at_speed(speed, distance, min_height, max_height);
    match result {
      DiscriminatedResult::Hit => speed,
      DiscriminatedResult::High => self.bsearch_step(distance, min_height, max_height, min_speed, speed),
      DiscriminatedResult::Low => self.bsearch_step(distance, min_height, max_height, speed, max_speed),
    }
  }
  
  fn get_discriminated_at_speed(&self, speed: AngularVelocity, distance: Length, min_height: Length, max_height: Length) -> DiscriminatedResult {
    let result = self.sim_manager.clone().query_speed(speed);
    let height = self.get_result_height_at_distance(result, distance);
    self.discriminate_height(height, min_height, max_height)
  }
  
  fn get_max_distance(&self) -> SimulationResult { self.sim_manager.clone().query_speed(self.max_speed) }
  fn get_result_height_at_distance(&self, result: SimulationResult, distance: Length) -> Length {
    let result_x = result.x;
    if result_x[result_x.len() - 1] < distance {
      return Length::new(-1.0);
    }
    let mut value_below = Length::new(0.0);
    let mut i_below = -1;
    let mut value_above = Length::new(0.0);
    let mut i_above = -1;
    
    for i in 0..result_x.len() {
      let value = result_x[i];
      if value <= distance {
        value_below = value;
        i_below = i;
      }
      if value_above == Length::zero() && value >= distance {
        value_above = value;
        i_above = i;
      }
      
    }
    let difference: Length = value_above - value_below;
    if difference == Length::new(0.0) {
      return result.y[i_below];
    }
    
    let scalar = (distance - value_below) / difference;
    
    result.y[i_below] + (result.y[i_above] - result.y[i_below]) * scalar
  }
  
  fn discriminate_height(&self, height: Length, min_height: Length, max_height: Length) -> DiscriminatedResult {
    if height < min_height {
      return DiscriminatedResult::Low
    }
    if height > max_height {
      return DiscriminatedResult::High
    }
    DiscriminatedResult::Hit
  }
}