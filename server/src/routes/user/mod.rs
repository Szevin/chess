use axum::response::IntoResponse;
use postgres::{Client, NoTls};

pub async fn get_user_handler() -> impl IntoResponse {
  let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

  client
    .batch_execute(
      "
    CREATE TABLE person (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        data    BYTEA
    )
",
    )
    .unwrap();

  let name = "Ferris";
  let data = None::<&[u8]>;
  client
    .execute(
      "INSERT INTO person (name, data) VALUES ($1, $2)",
      &[&name, &data],
    )
    .unwrap();

  let clients = client
    .query("SELECT id, name, data FROM person", &[])
    .unwrap();

  for row in clients {
    let id: i32 = row.get(0);
    let name: &str = row.get(1);
    let data: Option<&[u8]> = row.get(2);

    println!("found person: {} {} {:?}", id, name, data);
  }

  "Hello, World!"
}
