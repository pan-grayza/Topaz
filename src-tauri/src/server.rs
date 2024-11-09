use crate::local_dir::read_private_linked_paths;
use crate::types::{LinkedPath, LocalServerResponse, ServerMode, ServerWrapper};
use actix_files::Files;
use actix_web::{get, web, App, HttpServer};
use get_if_addrs::get_if_addrs;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn start_file_server_command(
    server_mode: ServerMode,
    linked_paths: Vec<LinkedPath>,
    server_wrapper: State<'_, Arc<Mutex<ServerWrapper>>>,
) -> Result<LocalServerResponse, String> {
    if linked_paths.is_empty() {
        return Ok(LocalServerResponse {
            status: "Choose linked paths to share".into(),
            addresses: vec![],
        });
    }
    let server_mode = server_mode.clone();
    let server_wrapper_clone = server_wrapper.inner().clone();
    let (tx, rx) = tokio::sync::oneshot::channel::<Vec<String>>();
    tokio::spawn(async move {
        match file_server(server_mode, linked_paths, server_wrapper_clone, tx).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Server error: {}", e);
            }
        }
    });
    println!("Waiting for the signal");
    match rx.await {
        Ok(addresses) => {
            println!("Signal received");
            for address in &addresses {
                println!("{}", address);
            }

            return Ok(LocalServerResponse {
                status: "Server started!".into(),
                addresses: addresses,
            });
        }
        Err(_) => {
            return Ok(LocalServerResponse {
                status: "Server started, but couldn't get addresses".into(),
                addresses: vec![],
            });
        }
    }
}

#[tauri::command]
pub async fn stop_file_server_command(
    server_wrapper: State<'_, Arc<Mutex<ServerWrapper>>>,
) -> Result<String, String> {
    let wrapper = server_wrapper.lock().await;
    if let Some(handle) = &wrapper.handle {
        // Stop gracefully (true parameter means wait for existing connections)
        handle.stop(true).await;
        return Ok("Server stopped!".to_string());
    } else {
        return Ok("Error while stopping server".to_string());
    }
}

#[get("/")]
async fn index_handler() -> actix_web::Result<impl actix_web::Responder> {
    let all_linked_paths = read_private_linked_paths().unwrap();
    let names: Vec<String> = all_linked_paths
        .into_iter()
        .map(|linked_path| linked_path.name)
        .collect();
    Ok(web::Json(names))
}

pub async fn file_server(
    server_mode: ServerMode,
    linked_paths: Vec<LinkedPath>,
    server_wrapper: Arc<Mutex<ServerWrapper>>,
    addresses_tx: tokio::sync::oneshot::Sender<Vec<String>>,
) -> tokio::io::Result<()> {
    log::info!("starting HTTP server at http://localhost:8080");
    match server_mode {
        ServerMode::LocalHost => {
            let server = HttpServer::new(move || {
                let mut app = App::new().service(index_handler);
                for linked_path in &linked_paths {
                    let route = format!("/{}", linked_path.name); // Each directory has a unique route

                    // Add a Files service for the directory with file listing
                    app = app
                        .service(Files::new(&route, linked_path.path.clone()).show_files_listing());
                }
                app
            })
            .workers(4)
            .bind(("0.0.0.0", 8080))?;

            let mut addresses: Vec<String> = vec![];
            println!("Server is accessible at the following addresses:");

            // Retrieve and print all interface IP addresses with the server port
            if let Ok(if_addrs) = get_if_addrs() {
                for iface in if_addrs {
                    if iface.ip().is_ipv4() {
                        println!("http://{}:{}", iface.ip(), 8080);
                        addresses.push(format!("http://{}:{}", iface.ip(), 8080));
                    }
                }
            }
            let _ = addresses_tx.send(addresses);

            let server = server.run();
            let handle = server.handle();
            {
                let mut wrapper = server_wrapper.lock().await;
                wrapper.handle = Some(handle);
            }
            server.await?;
            return Ok(());
        }
        ServerMode::Internet => {
            let server = HttpServer::new(move || {
                let mut app = App::new().service(index_handler);
                for linked_path in &linked_paths {
                    let route = format!("/{}", linked_path.name); // Each directory has a unique route

                    // Add a Files service for the directory with file listing
                    app = app
                        .service(Files::new(&route, linked_path.path.clone()).show_files_listing());
                }
                app
            })
            .workers(4)
            .bind(("0.0.0.0", 8080))?;

            let mut addresses: Vec<String> = vec![];
            println!("Server is accessible at the following addresses:");

            // Retrieve and print all interface IP addresses with the server port
            if let Ok(if_addrs) = get_if_addrs() {
                for iface in if_addrs {
                    if iface.ip().is_ipv4() {
                        println!("http://{}:{}", iface.ip(), 8080);
                        addresses.push(format!("http://{}:{}", iface.ip(), 8080));
                    }
                }
            }
            let _ = addresses_tx.send(addresses);

            let server = server.run();
            let handle = server.handle();
            {
                let mut wrapper = server_wrapper.lock().await;
                wrapper.handle = Some(handle);
            }
            server.await?;
            return Ok(());
        }
        ServerMode::DarkWeb => {
            let server = HttpServer::new(move || {
                let mut app = App::new().service(index_handler);
                for linked_path in &linked_paths {
                    let route = format!("/{}", linked_path.name); // Each directory has a unique route

                    // Add a Files service for the directory with file listing
                    app = app
                        .service(Files::new(&route, linked_path.path.clone()).show_files_listing());
                }
                app
            })
            .workers(4)
            .bind(("0.0.0.0", 8080))?;

            let mut addresses: Vec<String> = vec![];
            println!("Server is accessible at the following addresses:");

            // Retrieve and print all interface IP addresses with the server port
            if let Ok(if_addrs) = get_if_addrs() {
                for iface in if_addrs {
                    if iface.ip().is_ipv4() {
                        println!("http://{}:{}", iface.ip(), 8080);
                        addresses.push(format!("http://{}:{}", iface.ip(), 8080));
                    }
                }
            }
            let _ = addresses_tx.send(addresses);

            let server = server.run();
            let handle = server.handle();
            {
                let mut wrapper = server_wrapper.lock().await;
                wrapper.handle = Some(handle);
            }
            server.await?;
            return Ok(());
        }
    }
}
