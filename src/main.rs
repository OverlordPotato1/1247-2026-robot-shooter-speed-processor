use std::net::{Ipv4Addr, SocketAddrV4};
use database::Database;

pub mod nt;
pub mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = nt::v4::Client::try_new_w_config(
    SocketAddrV4::new(Ipv4Addr::new(10, 12, 47, 1), 5810),
    nt::v4::client_config::Config {
      ..Default::default()
    }
  ).await?;
  let database = Database::new(":cache:")?;
  
  let published = client.publish_topic("/ShooterCompute", nt::v4::Type::Int, None).await?;
  
  Ok(())
  
}
