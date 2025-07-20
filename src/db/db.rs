use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};

// Get the MongoDB client
pub async fn get_client() -> mongodb::error::Result<Client> {
  let mut client_options =
    ClientOptions::parse("mongodb+srv://ascenzen11:9lHkOwYjsioinHsO@cluster0.os4pokc.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0")
      .await?;
  let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
  client_options.server_api = Some(server_api);
  Ok(Client::with_options(client_options)?)
}


pub async fn test_connection() -> mongodb::error::Result<()> {
  let client = get_client().await?;

  // Ping the server to see if you can connect to the cluster
  client
    .database("admin")
    .run_command(doc! {"ping": 1})
    .await?;
  println!("Pinged your deployment. You successfully connected to MongoDB!");

  Ok(())
}
