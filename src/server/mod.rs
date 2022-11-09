// Copied from : https://github.com/robertohuertasm/microserver/blob/master/src/server.rs
// Please check them out if you are interested :D
// Copyright (c) 2018 Roberto Huertas

use std::net::{IpAddr, SocketAddr};

use warp::Filter;

//noinspection RsTypeCheck
pub async fn start(port: u16, path: String, is_spa: bool, spa_index: &str, address: &str) {
    let spa_index_path = format!("{}/{}", path, spa_index);
    println!(
      "MicroServer running on http://{}:{}!",
      address, port
    );
    println!("Serving {}", path);
    println!(
      "Spa support: {}. Root: {}", is_spa, spa_index
    );

    let files = warp::fs::dir(path);
    let spa = warp::any()
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(spa_index_path))
        .map(|_, file| file);

    let routes = files.or(spa);

    let ip: Result<IpAddr, _> = address.parse();
    match ip {
        Ok(ip) => {
            let socket_adr: SocketAddr = (ip, port).into();
            warp::serve(routes.map(|file| {
                println!("{:?}", file);
                file
            }))
            .run(socket_adr)
            .await;
        }
        Err(e) => {
            println!(
                "Something went wrong: {}", e
            );
        }
    }
}