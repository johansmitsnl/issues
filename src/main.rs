use futures::StreamExt;
use serde::Deserialize;
use std::{sync::LazyLock, time::Duration};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Notification, Surreal,
};
use tokio::time::interval;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);
const KEEP_ALIVE_INTERVAL: u64 = 1;
const THREADS: u64 = 20;

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Person {
    id: Thing,
    name: String,
    address: String,
    email: String,
}

fn handle(result: Result<Notification<Person>, surrealdb::Error>) {
    println!("Received notification: {:?}", result);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connect database");
    DB.connect::<Ws>("localhost:8001").await.expect("Did you start the database with: surreal start --user root --pass root --bind 0.0.0.0:8001 surrealkv://db");

    println!("Login to database");
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    println!("Create database");
    DB.use_ns("test").use_db("client_lockup").await?;

    println!("Set record");
    DB.query(
        r#"
        CREATE person:tobie SET
	name = "Tobie",
	address = "1 Bagshot Row",
	email = "tobie@surrealdb.com";
;"#,
    )
    .await?;

    println!("Query threads");
    for i in 0..THREADS {
        let db_thread = DB.clone();
        tokio::spawn(async move {
            println!("Task {} start select", i);
            let mut stream = db_thread.select(("person", "tobie")).live().await.unwrap();

            let query: Option<Person> = db_thread
                .select(("person", "tobie"))
                .await
                .expect("This query should work");
            println!("Task {} start selected and ready with data: {:?}", i, query);

            // Process updates as they come in.
            while let Some(result) = stream.next().await {
                // Do something with the notification
                handle(result);
            }
        });
    }

    println!("Main thread should be kept alive");

    let mut interval = interval(Duration::from_secs(KEEP_ALIVE_INTERVAL));
    loop {
        println!("Still alive");
        interval.tick().await;
    }
}
