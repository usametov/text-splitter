extern crate clap;

mod config;
mod processor;

use axum::{
  routing::post,
  Router,
};
use std::net::SocketAddr;
use tokio;

async fn run(list_of_files: Vec<String>, cfg: &config::Config) {

  processor::run(
    list_of_files,
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
  
  let list_of_files = cfg.input_files;
  assert!(cfg.web || !list_of_files.is_empty());

  if(cfg.web) {
    // Create the Axum router
    //let app = Router::new().route("/v1/run", post(run));
 
    // Start the Axum server  
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    //axum::serve(listener, app).await.unwrap();
  } else {
    processor::run(list_of_files, cfg.working_dir
               , cfg.output_dir.as_str() 
               , cfg.min_chars .. cfg.max_chars
               , cfg.is_verbose, cfg.prfx_replacement.as_str()
               , cfg.strip_prefix.as_str())   
  }
  
}

