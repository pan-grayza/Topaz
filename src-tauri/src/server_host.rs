use crate::types::{  ServerMode, ShutdownServerMap, ServerIdState, NetworkName, Address, Network, ServerGroup, ServerGroupSerde};
use tauri::State;
use tokio::sync::mpsc;
use axum::{ routing::get, Router,
    response::Json
};
use std::net::SocketAddr;
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
};

#[tauri::command]
pub async fn start_file_server_command(
    server_mode: ServerMode,
    network: Network,
    shutdown_map: State<'_, ShutdownServerMap>,
    server_id_state: State<'_, ServerIdState>,
) -> tauri::Result<()> {
    let shutdown_map = shutdown_map.inner().clone();
    let server_id_state = server_id_state.inner().clone();
    tokio::spawn(async move {
        match file_server(server_mode, network, shutdown_map, server_id_state).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Server error: {}", e);
            }
        }
    });
    Ok(())
}

#[tauri::command]
pub async fn stop_file_server_command(
    network_name: NetworkName,
    id:u64,
    shutdown_map: State<'_, ShutdownServerMap>,
) -> tauri::Result<()> {
    let mut map = shutdown_map.write().await;

    // Check if the server ID exists
    if let Some(server_groups) = map.get_mut(&network_name) {
        if let Some(index) = server_groups.iter().position(|sg| sg.id == id) {
            let server_group = server_groups.remove(index);
            
            // Send shutdown signal to the server
            if let Err(_) = server_group.tx.send(()).await {
                println!("Failed to send shutdown signal to server ID {}", id);
            } else {
                println!("Server with ID {} is shutting down...", id);
            }
        } else {
            println!("No server found with ID {}", id);
        }
    } else {
        println!("No server groups found for network {}", network_name);
    }
    Ok(())
}

pub async fn file_server(
    server_mode: ServerMode,
    network: Network,
    shutdown_map: ShutdownServerMap,
    server_id_state: ServerIdState,
) -> tokio::io::Result<()> {
    log::info!("starting HTTP server at http://localhost:8080");
    let mut addresses = Vec::new();
    let port = 8080;

    match server_mode {
        ServerMode::LocalHost => {
            let linked_paths_names: Vec<String> = network.linked_paths.iter().map(|linked_path| linked_path.name.clone()).collect();
            let serve_linked_paths_names = get(Json(linked_paths_names));
            

            let mut app = Router::new()
            .route("/", serve_linked_paths_names);


            for linked_path in &network.linked_paths {
                println!("{:?}",linked_path.path.clone());
                let dir = ServeDir::new(linked_path.path.clone());
                app = app.nest_service(&format!("/{}", linked_path.name),dir);
            }

            let addr = SocketAddr::from(([0, 0, 0, 0], port));
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            println!("listening on {}", listener.local_addr().unwrap());
            let (tx, mut rx) = mpsc::channel::<()>(1);

            println!("Server is accessible at the following addresses:");

            // Retrieve IP addresses linked to this server
            if let Ok(if_addrs) = get_if_addrs::get_if_addrs() {
                for iface in if_addrs {
                    if iface.ip().is_ipv4() {
                        let ip = iface.ip().to_string();
                        println!("http://{}:{}", ip, port);
                        addresses.push(Address {
                            ip,
                            port,
                        });
                    }
                }
            }

            let id =  server_id_state.generate_server_id().await;

            {
                let mut map = shutdown_map.write().await;
                let mut server_group_vector = Vec::new();
                server_group_vector.push(ServerGroup {
                    id,
                    addresses,
                    tx
                });
                map.insert(network.name, server_group_vector);
            }
            
            axum::serve(listener, app.layer(TraceLayer::new_for_http()))
                .with_graceful_shutdown(async move {
                    rx.recv().await; // Wait for shutdown signal
                })
                .await
                .unwrap();
            return Ok(());
        }
        ServerMode::Internet => {
            return Ok(());
        }
        ServerMode::DarkWeb => {
            return Ok(());
        }
    }
}

#[tauri::command]
pub async fn get_servers(
    network_name: NetworkName,
    shutdown_map: State<'_, ShutdownServerMap>,
) -> Result<Vec<ServerGroupSerde>, String> {
    let map = shutdown_map.read().await;

    if let Some(server_groups) = map.get(&network_name) {
        let server_groups_serde: Vec<ServerGroupSerde> = server_groups.iter()
            .map(|sg| ServerGroupSerde {
                id: sg.id,
                addresses: sg.addresses.clone(),
            })
            .collect();
        Ok(server_groups_serde)
    } else {
        println!("No server groups found for network {}", network_name);
        Ok(Vec::new())
    }
}