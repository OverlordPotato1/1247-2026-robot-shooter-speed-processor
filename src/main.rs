use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::OnceLock;
use rmpv::ext::from_value;
use serde::{Deserialize, Deserializer, Serialize};
use tokio::task::JoinHandle;
use uom::si::angle::radian;
use uom::si::f32::{Angle, Length, Mass, MassDensity, Pressure, Time, Velocity};
use uom::si::length::meter;
use uom::si::mass::kilogram;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::pressure::pascal;
use uom::si::time::second;
use uom::si::velocity::meter_per_second;
// use database::Database;
use crate::nt::v4::{Client, MessageData};

mod nt;
mod database;
mod search_manager;
pub mod constants;
pub mod simulation_manager;
mod macros;


pub use constants::MAX_DISTANCE;
pub use constants::MAX_RPM;
use crate::search_manager::search_manager::SearchManager;
use crate::simulation_manager::types::SimulationConfig;

static CLIENT: OnceLock<Client> = OnceLock::new();

fn get_client() -> Client {
  CLIENT.get().expect("NetworkTables client was not instantiated before accessing").clone()
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  CLIENT.set(Client::try_new_w_config(
    SocketAddrV4::new(Ipv4Addr::new(10, 12, 47, 1), 5810),
    nt::v4::client_config::Config {
      ..Default::default()
    }
  ).await?).or(Err(anyhow::Error::msg("Client was already locked")))?;
  
  // let database = Database::new(":cache:")?;
  
  let published = get_client().publish_topic("/ShooterCompute", nt::v4::Type::Int, None).await?;
  
  let mut subscription = get_client().subscribe(&["/ShooterComputeInput"]).await?;
  
  // let task_client = client.clone();
  let mut running_task: Option<JoinHandle<anyhow::Result<()>>> = None;
  
  let search_manager = SearchManager::new();
  
  while let Some(message) = subscription.next().await {
    if running_task.is_some() {
      let task = running_task.unwrap();
      if !task.is_finished() {
        task.abort();
      }
    }
    running_task = Some(tokio::spawn(task_processor(&search_manager, message)));
  }
  
  
  Ok(())
}

async fn task_processor(search_manager: &SearchManager, message_data: MessageData) -> anyhow::Result<()> {
  let data: InputData = from_value(message_data.data)?;
  let config = SimulationConfig {
    launch_angle:         Angle::new::<radian>(data.launch_angle.into()),
    
    launch_height:        Length::new::<meter>(data.launch_height.into()),
    wheel_radius:         Length::new::<meter>(data.wheel_radius.into()),
    ball_radius:          Length::new::<meter>(data.ball_radius.into()),
    
    drag_coefficient:     data.drag_coefficient.into(),
    lift_coefficient:     data.lift_coefficient.into(),
    rotation_coefficient: data.rotation_coefficient.into(),
    iterations:           data.iterations.into(),
    
    gravity:              Velocity::new::<meter_per_second>(data.gravity.into()),
    
    ball_mass:            Mass::new::<kilogram>(data.ball_mass.into()),
    
    air_density:          MassDensity::new::<kilogram_per_cubic_meter>(data.air_density.into()),
    
    simulation_time:      Time::new::<second>(data.simulation_time.into()),
  };
  let distance = Length::new::<meter>(data.distance.into());
  let min_height = Length::new::<meter>(data.min_height.into());
  let max_height = Length::new::<meter>(data.max_height.into());
  
  Ok(())
}

#[derive(Serialize, Deserialize)]
struct InputData {
  pub launch_angle: f64,
  pub launch_height: f64,
  pub drag_coefficient: f64,
  pub lift_coefficient: f64,
  pub rotation_coefficient: f64,
  pub wheel_radius: f64,
  
  pub ball_radius: f64,
  pub gravity: f64,
  pub ball_mass: f64,
  pub air_density: f64,
  
  pub simulation_time: f64,
  pub iterations: f64,
  
  pub distance: f64,
  pub min_height: f64,
  pub max_height: f64
}