use std::error::Error;

use rbmini::connection::RbManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Creating a new RbConnecting handler");
    let mut rb = match RbManager::new().await {
        Err(e) => {
            panic!("{}", e);
        }
        Ok(rb) => rb,
    };

    println!("connecting to racebox mini");
    match rb.connect().await {
        Err(e) => {
            panic!("{}", e);
        }
        Ok(conn) => {
            println!("connected");
            conn.stream().await;
        }
    }

    Ok(())
}
