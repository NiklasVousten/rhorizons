use chrono::Utc;
use rhorizons::{ephemeris, major_bodies};

#[tokio::main]
async fn main() {
    env_logger::init();

    for body in major_bodies().await {
        eprintln!("{:?}", body);
    }

    if let Some(body) = major_bodies()
        .await
        .iter()
        .find(|body| body.name == "Earth")
    {
        eprintln!("{:?}", body);
        for vectors in ephemeris(body.id, Utc::now() - chrono::Duration::days(1), Utc::now()).await
        {
            eprintln!("{:?}", vectors);
        }
    }
}