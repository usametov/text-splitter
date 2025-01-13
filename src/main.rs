extern crate clap;

mod config;
mod processor;
use tracing::{info, instrument /*, error */};
use tracing_subscriber::{fmt, EnvFilter};

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

#[instrument(skip(cfg, payload), fields(
  num_files = payload.list_of_files.len(),
  working_dir = %cfg.working_dir.display(),
  output_dir = %cfg.output_dir,
  chunk_range = format!("{}..{}", cfg.min_chars, cfg.max_chars)
))]
async fn run(State(cfg): State<Arc<config::Config>>, 
             Json(payload): Json<PostParams>) 
             -> Json<Value> {

    info!("Starting processing of files");

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
      Err(_) => Json(json!({ "result": { "success": false, "error": "processing error!" } })),
    }
}

fn validate_config(cfg: &config::Config) {
  // Validate chunk size range
  assert!(cfg.min_chars < cfg.max_chars, "min_chars must be less than max_chars");
  
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
}

#[tokio::main]
async fn main() {

  dotenv::dotenv().expect("Failed to read .env file");

  fmt().with_writer(std::io::stdout) // Log to console
       .with_env_filter(EnvFilter::from_default_env()
       .add_directive("text-splitter=info".parse().unwrap()))
       .init();

  info!("Starting application");

  let cfg = config::get_args().expect("Could not read config");  
  validate_config(&cfg);
  
  let input_file = cfg.input_files.clone();
  
  if cfg.web {
    // Create the Axum router
    let app = Router::new()
                    .route("/api/v1/run", post(run))
                    .with_state(Arc::new(cfg));
 
    let port = std::env::var("WEB_PORT")
                      .unwrap_or("8080".to_string())
                      .parse::<u16>()
                      .expect("Invalid port number");

    // Start the Axum server  
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
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
    
  }
     
}

