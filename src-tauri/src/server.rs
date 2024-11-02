use crate::local_dir::read_private_linked_paths;
use crate::types::{LinkedPath, ServerMode, ServerState};
use actix_files::Files;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, middleware, web, App, Error, HttpRequest, HttpServer};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tokio::sync::mpsc::{self, Receiver};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn start_file_server_command(
    server_mode: ServerMode,
    linked_paths: Vec<LinkedPath>,
    state: State<'_, Arc<Mutex<ServerState>>>,
) -> Result<String, String> {
    if linked_paths.is_empty() {
        return Ok("Choose linked paths to share".into());
    }
    let mut server_state = state.lock().await;

    // If the server is already running, return an error
    if server_state.shutdown_tx.is_some() {
        return Err("Server is already running.".into());
    }
    let (shutdown_tx, shutdown_rx) = mpsc::channel(100);
    server_state.shutdown_tx = Some(shutdown_tx);
    std::thread::spawn(|| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let _ = file_server(server_mode, linked_paths).await;
        });
    });
    Ok("Server started!".into())
}

#[tauri::command]
pub async fn stop_file_server_command(
    state: State<'_, Arc<Mutex<ServerState>>>,
) -> Result<String, String> {
    let mut server_state = state.lock().await;

    if let Some(shutdown_tx) = server_state.shutdown_tx.take() {
        shutdown_tx
            .send(())
            .await
            .expect("Failed to send shutdown signal");
        Ok("Server stopped.".into())
    } else {
        Err("Server is not running.".into())
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
    _server_mode: ServerMode,
    linked_paths: Vec<LinkedPath>,
    // mut shutdown_rx: Receiver<()>,
) -> tokio::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");
    println!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        let mut app = App::new().service(index_handler);
        for linked_path in &linked_paths {
            let route = format!("/{}", linked_path.name); // Each directory has a unique route

            // Add a Files service for the directory with file listing
            app = app.service(Files::new(&route, linked_path.path.clone()).show_files_listing());
        }
        app
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    // match server_mode {
    //     ServerMode::LocalHost => {

    //     }
    //     ServerMode::Internet => {

    //     }
    //     ServerMode::DarkWeb => {

    //     }
    // }
}
