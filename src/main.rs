extern crate clap;

mod config;
mod processor;

use serde::Deserialize;
use serde_json::{json, Value};
use axum::{
  routing::post,
  Router,
  extract::State,
  Json
};
use anyhow::Context;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio;
use std::sync::Arc;
use std::fs;

#[derive(Deserialize)]
struct PostParams {
  list_of_files: Vec<String> 
}

async fn run(State(cfg): State<Arc<config::Config>>, 
             Json(payload): Json<PostParams>) 
             -> Json<Value> {

    let result = processor::run(
                                          payload.list_of_files,
                                          cfg.working_dir.clone(),
                                          cfg.output_dir.as_str(),
                                          cfg.min_chars .. cfg.max_chars,
                                          cfg.is_verbose,
                                          cfg.prfx_replacement.as_str(),
                                          cfg.strip_prefix.as_str());

    match result {
      Ok(_) => Json(json!({ "result": { "success": true } })),
      Err(e) => Json(json!({ "result": { "success": false, "error": e.to_string() } })),
    }
}

#[tokio::main]
async fn main() {

  let cfg = config::get_args().expect("Could not read config");  
  assert!(cfg.min_chars < cfg.max_chars); 
   // Validate working_dir
  assert!(
    cfg.working_dir.exists(),
    "Working directory does not exist: {:?}",
    cfg.working_dir
    );
  assert!(
        cfg.working_dir.is_dir(),
        "Working directory is not a directory: {:?}",
        cfg.working_dir
    );

    // Validate output_dir
  let output_dir = PathBuf::from(&cfg.output_dir);
  println!("Output directory absolute path: {:?}", output_dir.canonicalize());
  assert!(
        output_dir.exists(),
        "Output directory does not exist: {:?}",
        output_dir
    );
  assert!(
        output_dir.is_dir(),
        "Output directory is not a directory: {:?}",
        output_dir
  );

  let input_file = cfg.input_files.clone();
  
  if cfg.web {
    // Create the Axum router
    let app = Router::new()
                    .route("/v1/run", post(run))
                    .with_state(Arc::new(cfg));
 
    // Start the Axum server  
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app)
         .await.context("axum serve failed")
         .unwrap();
    
  } else {

    let content = fs::read_to_string(input_file).unwrap();      
    let list_of_files: Vec<_> = content.split('\n').map(|s| s.to_string()).collect();
    assert!(!list_of_files.is_empty());

    let _ = processor::run(list_of_files, cfg.working_dir
               , cfg.output_dir.as_str() 
               , cfg.min_chars .. cfg.max_chars
               , cfg.is_verbose, cfg.prfx_replacement.as_str()
               , cfg.strip_prefix.as_str());
    
  };
     
}

