use sqlite::{Connection, State, Statement};

pub struct Database {
  connection: Connection,
}

impl Database {
  pub fn new(location: impl AsRef<std::path::Path>) -> anyhow::Result<Database> {
    let connection = sqlite::open(location)?;
    let database = Database {
      connection,
    };
    
    database.ensure_schema()?;
    
    Ok(database)
  }
  
  fn ensure_schema(&self) -> anyhow::Result<()> {
    self.connection.execute("
      CREATE TABLE IF NOT EXISTS results (
      id       INT PRIMARY KEY AUTO_INCREMENT
      distance INT UNIQUE
      rpm      INT NOT NULL
    ")?;
    
    Ok(())
  }
  
  fn prepare_distance_query(&self) -> anyhow::Result<Statement> {
    let query = "SELECT * FROM result WHERE distance = ? LIMIT 1";
    let statement = self.connection.prepare(query)?;
    Ok(statement)
  }
  
  pub fn query_distance(&self, distance: i32) -> anyhow::Result<i32> {
    let mut statement = self.prepare_distance_query()?;
    statement.bind::<(usize, &str)>((1, distance.to_string().as_str()))?;
    
    let response = statement.next()?;
    if let State::Row = response {
      Ok(statement.read::<String, _>("rpm")?.parse()?)
    } else {
      Err(anyhow::Error::msg("No data"))
    }
  }
}