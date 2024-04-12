mod run;

#[tokio::main]
async fn main() {
    run::run().ok();
}
