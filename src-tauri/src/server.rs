use crate::local_dir::read_private_linked_paths;
use crate::types::{LinkedPath, ServerMode, ServerWrapper};
use actix_files::Files;
use actix_web::{get, web, App, HttpServer};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn start_file_server_command(
    server_mode: ServerMode,
    linked_paths: Vec<LinkedPath>,
    server_wrapper: State<'_, Arc<Mutex<ServerWrapper>>>,
) -> Result<String, String> {
    if linked_paths.is_empty() {
        return Ok("Choose linked paths to share".into());
    }
    let server_wrapper_clone = server_wrapper.inner().clone();
    let server_task =
        tokio::spawn(
            async move { file_server(server_mode, linked_paths, server_wrapper_clone).await },
        );
    Ok("Server started!".into())
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
            .bind(("127.0.0.1", 8080))?
            .run();
            let handle = server.handle();
            {
                let mut wrapper = server_wrapper.lock().await;
                wrapper.handle = Some(handle);
            }
            server.await?;
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
            .bind(("127.0.0.1", 8080))?
            .run();
            let handle = server.handle();
            {
                let mut wrapper = server_wrapper.lock().await;
                wrapper.handle = Some(handle);
            }
            server.await?;
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
            .bind(("127.0.0.1", 8080))?
            .run();
            let handle = server.handle();
            {
                let mut wrapper = server_wrapper.lock().await;
                wrapper.handle = Some(handle);
            }
            server.await?;
        }
    }
    Ok(())
}
