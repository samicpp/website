use std::time::Duration;

use rust_http::common::{HttpResult, HttpSocket};

use crate::structs::SharedData;



pub async fn api_hand<S:HttpSocket>(_shared: &SharedData, path: &str, mut res: S) -> HttpResult<()>{
    res.set_header("Content-Type", "text/plain")?;
    res.write(format!("test write {}\n",path).as_bytes()).await?;
    tokio::time::sleep(Duration::from_millis(1000)).await;
    res.close(b"test close").await?;
    // dbg!(&res);
    dbg!(path);
    Ok(())
}

