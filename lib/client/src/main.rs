pub mod client;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "push" => return client::push().await,
            "create" => return client::create().await,
            _ => (),
        }
    }

    println!("usage: {} [create | push] ADDRESS", args[0]);
    Ok(())
}
