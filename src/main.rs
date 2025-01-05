extern crate clap;

mod config;
mod processor;

fn main() {

  let cfg = config::get_args().expect("Could not read config");  
  assert!(cfg.min_chars < cfg.max_chars);  

  let chunk_size_range = cfg.min_chars .. cfg.max_chars;    
  let list_of_files = cfg.input_files; 
  assert!(!list_of_files.is_empty());  

  //TODO: create 
  // let app = Router::new().route("/", get(calc_chunks));
          
  // let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
  //  axum::serve(listener, app).await.unwrap();

  processor::run(list_of_files, cfg.working_dir
               , cfg.output_dir.as_str(), chunk_size_range
               , cfg.is_verbose, cfg.prfx_replacement.as_str()
               , cfg.strip_prefix.as_str())   

}

