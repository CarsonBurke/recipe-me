use db::{db_conn, sample_data::create_sample_data};

mod db;

#[tokio::main]
async fn main() {
    /* tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(launch_server()); */
    launch_server().await;
}

async fn launch_server() {
    // Connect to dioxus' logging infrastructure
    
    let db = db_conn().await.unwrap();

    create_sample_data().await.unwrap();
}