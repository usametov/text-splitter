extern crate clap;

mod config;
mod processor;

use serde::Deserialize;
use axum::{
  routing::post,
  Router,
  extract::State,
  Json
};
use std::net::SocketAddr;
use tokio;
use std::sync::Arc;
use std::fs;

#[derive(Deserialize)]
struct PostParams {
  list_of_files: Vec<String> 
}

async fn run(State(cfg): State<Arc<config::Config>>, 
             Json(payload): Json<PostParams>) {

  processor::run(
    payload.list_of_files,
    cfg.working_dir.clone(),
    cfg.output_dir.as_str(),
    cfg.min_chars .. cfg.max_chars,
    cfg.is_verbose,
    cfg.prfx_replacement.as_str(),
    cfg.strip_prefix.as_str())
}

#[tokio::main]
async fn main() {

  let cfg = config::get_args().expect("Could not read config");  
  assert!(cfg.min_chars < cfg.max_chars); 
  
  let input_file = cfg.input_files.clone();
  
  if cfg.web {
    // Create the Axum router
    let app = Router::new()
                                        .route("/v1/run", post(run))
                                        .with_state(Arc::new(cfg));
 
    // Start the Axum server  
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
  } else {

    let content = fs::read_to_string(input_file).unwrap();      
    let list_of_files: Vec<_> = content.split('\n').map(|s| s.to_string()).collect();
    assert!(!list_of_files.is_empty());

    processor::run(list_of_files, cfg.working_dir
               , cfg.output_dir.as_str() 
               , cfg.min_chars .. cfg.max_chars
               , cfg.is_verbose, cfg.prfx_replacement.as_str()
               , cfg.strip_prefix.as_str())   
  }
  
}

