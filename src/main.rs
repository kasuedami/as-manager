use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    zug_orga::app::run_app().await;
}
