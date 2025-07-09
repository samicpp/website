mod mime_map;
mod handlers;
mod structs;
mod javascript;
mod module_loader;
mod lua;

use rust_http::{
    http1::handler::Http1Socket, listener, /*traits::HttpSocket*/
};

use std::{
    env,
    path::Path,
    sync::Arc, time::{Instant},
};

use crate::{mime_map::mime_map, structs::SharedData};
// use futures::future::BoxFuture;
// use futures::FutureExt;


#[tokio::main]
async fn main()->std::io::Result<()> {
    let start=Instant::now();
    // println!("listenening http://0.0.0.0:4096/");
    // rust_http::listener::http_listener("0.0.0.0:4096",listener).await.unwrap();
    // let dotenv_successfull = dotenv().ok();
    let _dotenvy_successfull = dotenvy::from_path(Path::new(".env"));

    let mut serve_dir: String = env::var("serve_dir").unwrap_or("./public".to_string());
    let mut port: u16 = env::var("port").unwrap_or("3000".to_string()).parse().unwrap_or(3000);
    let mut host = { 
        // let mut host: [u8; 4] = [0, 0, 0, 0];
        // let host_str = env::var("host").unwrap_or("0.0.0.0".to_string());
        // let parts: Vec<&str> = host_str.split('.').collect();
        // if parts.len() == 4 {
        //     for (i, part) in parts.iter().enumerate() {
        //         host[i] = part.parse::<u8>().unwrap_or(0);
        //     }
        // }

        // host
        env::var("host").unwrap_or("0.0.0.0".to_string())
    };
    let args: Vec<String> = env::args().collect();

    // Check arguments that the user provided
    if args.len() > 1 {
        serve_dir = args[1].clone();
    } if args.len() > 2 {
        port = args[2].parse::<u16>().unwrap_or(3000);
    } if args.len() > 3 {
        let host_str = args[3].clone();
        host=host_str;
        // let parts: Vec<&str> = host_str.split('.').collect();
        // if parts.len() == 4 {
        //     for (i, part) in parts.iter().enumerate() {
        //         host[i] = part.parse::<u8>().unwrap_or(0);
        //     }
        // } else {
        //     eprintln!("Invalid host address. Using default");
        // }
    }

    // dbg!(dotenv_successfull);
    // dbg!(dotenvy_successfull);
    // dbg!(env::var("serve_dir"));


    println!(
        "Parameters of the server are\n\x1b[32mport = {}\n\x1b[33mhost = {:?}\n\x1b[34mdirectory = {}\x1b[0m",
        port, host, serve_dir
    );

    let deno_snapshot=Arc::new(Vec::<u8>::new());//Arc::new(javascript::create_snapshot().expect("couldnt create snapshot"));

    // let serve_dir=Arc::new(serve_dir);
    let shared=Arc::new(SharedData{
        mime: mime_map(), 
        serve_dir, 
        deno_snapshot,
    });

    let listener = {
        // let serve_dir = Arc::clone(&serve_dir);
        let shared=Arc::clone(&shared);
        move |conn: Http1Socket| {
            // let serve_dir = Arc::clone(&serve_dir);
            let shared=Arc::clone(&shared);
            async move {
                let now=Instant::now();
                let res = handlers::handler(&shared, conn).await;
                println!("\x1b[36mhandler took {}ms\x1b[0m",now.elapsed().as_nanos() as f64 /1000000.0);
                match res {
                    Ok(())=>println!("\x1b[32mhandler didnt error\x1b[0m"),
                    Err(err)=>eprintln!("\x1b[31mhandler errored\n{:?}\x1b[0m",err),
                };
            }
        }
    };

    let full_addr=host+":"+&port.to_string();

    // ctrlc::set_handler(move||{
    //     println!("SIG_INT received\nprocess exit after {}s",&start.elapsed().as_millis()/1000);
    //     std::process::exit(0);
    // }).expect("couldnt set ctrl+c handler");

    println!("http://{}/",&full_addr);
    listener::http_listener(&full_addr, listener).await.unwrap();
    
    // loop{}

    println!("process exit after {}s",&start.elapsed().as_millis()/1000);

    Ok(())
}



// async fn test_listener(mut conn: Http1Socket){
//     if let Err(err)=conn.update_client().await{
//         conn.status=500;
//         conn.status_msg="Internal Server Error".to_owned();
//         conn.set_header("Content-Type", "text/plain");
//         let _=conn.close(format!("Internal Server Error occoured:\n{:?}",err).as_bytes()).await;
//     };
//     conn.close(b"after a long time").await.unwrap();
// }