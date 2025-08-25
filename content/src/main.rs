mod mime_map;
mod handlers;
mod structs;
mod custom;

use rust_http::{
    http1::handler::Http1Socket, /*listener, common::HttpSocket,*/ common::HttpConstructor,
    common::Stream,
};
// use tokio::net::TcpStream;

use std::{
    env,
    path::Path,
    sync::Arc, time::{Instant},
};

use crate::{mime_map::mime_map, structs::SharedData};

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use std::{fs::File, io::BufReader};
use tokio::{
    // io::{AsyncRead, AsyncWrite},
    net::TcpListener,
};
use tokio_rustls::{/*server::TlsStream,*/ TlsAcceptor};

// impl Stream for tokio_rustls::TlsStream<TcpStream>{}

fn load_certs(path: &str) -> std::io::Result<Vec<Certificate>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let certs = certs(&mut reader)?;
    Ok(certs.into_iter().map(Certificate).collect())
}

fn load_private_key(path: &str) -> std::io::Result<PrivateKey> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);

    // pkcs8
    if let Ok(mut keys) = pkcs8_private_keys(&mut reader) {
        if !keys.is_empty() {
            return Ok(PrivateKey(keys.remove(0)));
        }
    }

    // rsa
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    if let Ok(mut keys) = rsa_private_keys(&mut reader) {
        if !keys.is_empty() {
            return Ok(PrivateKey(keys.remove(0)));
        }
    }

    Err(std::io::Error::new(std::io::ErrorKind::Unsupported,format!("no private keys found in {}", path)))
}

fn load_key_cert(key_path:&str,cert_path:&str)->Option<(PrivateKey,Vec<Certificate>)>{
    let key=match load_private_key(key_path){
        Ok(k)=>k,
        Err(e)=>{
            eprintln!("reading private key failed {e:?}");
            return None;
        }
    };
    let certs=match load_certs(cert_path){
        Ok(cs)=>cs,
        Err(e)=>{
            eprintln!("reading certificates failed {e:?}");
            return None;
        }
    };
    println!("successfully read private key and certificates");
    Some((key,certs))
}

#[tokio::main]
async fn main()->std::io::Result<()> {
    let start=Instant::now();

    match dotenvy::from_path(Path::new(".env")){
        Err(e)=>eprintln!("WARNING: couldnt load .env file {:?}",e),
        Ok(_)=>(),
    };
    let serve_dir: String = env::var("serve_dir").unwrap_or("./public".to_string());
    let address = env::var("address").unwrap_or("0.0.0.0:1024".to_string());
    
    let cert_path = env::var("cert").unwrap_or("localhost.crt".to_string());
    let key_path = env::var("key").unwrap_or("localhost.key".to_string());

    let key_cert=load_key_cert(&key_path, &cert_path);


    let tls_config=if let Some((key,certs))=key_cert.clone(){
        let sco=ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs,key).ok();
        match sco{
            Some(mut sc)=>{
                sc.alpn_protocols=vec![b"http/1.1".to_vec()];
                let acc=TlsAcceptor::from(Arc::new(sc));
                Some(acc)
            },
            None=>None,
        }
    }else{None};

    if key_cert.is_some()&&tls_config.is_some(){ println!("succesfully loaded tls config") }
    else if key_cert.is_some()&&tls_config.is_none(){ eprintln!("couldnt load tls. using plain tcp") }

    let shared=Arc::new(SharedData{
        mime: mime_map(), 
        serve_dir,
        tls_acceptor: tls_config,
    });

    // let listener = {
    //     let shared=Arc::clone(&shared);
    //     move |conn: Http1Socket<TcpStream>| {
    //         let shared=Arc::clone(&shared);
    //         async move {
    //             let now=Instant::now();
    //             let res = handlers::handler(&shared, conn).await;
    //             println!("\x1b[36mhandler took {}ms\x1b[0m",now.elapsed().as_nanos() as f64 /1000000.0);
    //             match res {
    //                 Ok(())=>println!("\x1b[32mhandler didnt error\x1b[0m"),
    //                 Err(err)=>eprintln!("\x1b[31mhandler errored\n{:?}\x1b[0m",err),
    //             };
    //         }
    //     }
    // };

    ctrlc::set_handler(move||{
        println!("\x1b[31mSIG_INT received\x1b[0m\n\x1b[36mprocess exit after {}s\x1b[0m",&start.elapsed().as_millis()/1000);
        std::process::exit(0);
    }).expect("couldnt set ctrl+c handler");

    println!("http://{}/",&address);
    // listener::http_listener(&address, listener).await.unwrap();
    let server = TcpListener::bind(&address).await?;
    loop{
        let (socket, addr) = server.accept().await?;
        let shared=Arc::clone(&shared);
        //let listener=listener.clone();
        if let Some(acc)=&shared.tls_acceptor{
            let acceptor = acc.clone();
            tokio::spawn(async move {
                match acceptor.accept(socket).await{
                    Ok(tls_sock)=>{
                        let hand=Http1Socket::new(tls_sock,addr);
                        listener(shared, hand).await;
                    },
                    Err(err)=>{
                        eprintln!("tls handshake failed {:?}",err);
                    }
                }
            });
        } else if shared.tls_acceptor.is_none(){
            let hand=Http1Socket::new(socket,addr);
            tokio::spawn(async move {
                listener(shared,hand).await;
            });
        }
    }
    

    // println!("process exit after {}s",&start.elapsed().as_millis()/1000);

    // Ok(())
}

async fn listener<S>(shared:Arc<SharedData>,hand:Http1Socket<S>)
where S: Stream
{
    let shared=Arc::clone(&shared);
        
    let now=Instant::now();
    let res = handlers::handler(&shared, hand).await;
    println!("\x1b[36mhandler took {}ms\x1b[0m",now.elapsed().as_nanos() as f64 /1000000.0);
    match res {
        Ok(())=>println!("\x1b[32mhandler didnt error\x1b[0m"),
        Err(err)=>eprintln!("\x1b[31mhandler errored\n{:?}\x1b[0m",err),
    };
}