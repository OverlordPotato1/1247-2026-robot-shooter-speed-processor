use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::OnceLock;
use tokio::task::JoinHandle;
// use database::Database;
use crate::nt::v4::Client;

pub mod nt;
pub mod database;

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
  
  // let published = get_client().publish_topic("/ShooterCompute", nt::v4::Type::Int, None).await?;
  
  let mut subscription = get_client().subscribe(&["/ShooterComputeInput"]).await?;
  
  // let task_client = client.clone();
  let mut running_task: Option<JoinHandle<anyhow::Result<()>>> = None;
  
  while let Some(message) = subscription.next().await {
    if running_task.is_some() {
      let task = running_task.unwrap();
      if !task.is_finished() {
        task.abort();
      }
    }
    running_task = Some(tokio::spawn(task_processor()));
  }
  
  
  Ok(())
}

async fn task_processor() -> anyhow::Result<()> {
  Ok(())
}