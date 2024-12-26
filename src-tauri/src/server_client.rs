use crate::local_dir::read_private_linked_paths;
use crate::types::{LinkedPath, LocalServerResponse, ServerMode, ServerWrapper};
use actix_files::Files;
use actix_web::{get, web, App, HttpServer};
use get_if_addrs::get_if_addrs;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn get_host_linked_paths(
    address: String,
) -> Result<String, String> {
    Ok(format!("It's working for now {}", address))
}