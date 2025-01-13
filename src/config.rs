use std::path::PathBuf;
use std::error::Error;
use clap::{App, Arg};

#[derive(Debug, Clone)]
pub struct Config {

    pub working_dir: PathBuf,
    pub output_dir: String,    
    pub input_files: String,    
    pub min_chars: usize,
    pub max_chars: usize,
    pub strip_prefix: String,
    pub prfx_replacement: String,
    pub is_verbose: bool,
    pub web: bool
}

pub fn get_args() -> Result<Config, Box<dyn Error>> { 

    let min_char = std::env::var("MIN_CHAR")
      .unwrap_or("200".to_string());

    let max_char = std::env::var("MAX_CHAR")
      .unwrap_or("500".to_string());   

    let matches = App::new("semantic splitter")
      .version("0.1")
      .author("Ulan Sametov <usametov@gmail.com>")
      .about("performs semantic split")  
      .arg(Arg::with_name("input-files-list")        
            .short("i") 
            .long("input-files")
            .help("file containing list of relative paths to documents to process")
            .required(false)
            .min_values(0)
            .max_values(1))
      .arg(Arg::with_name("working-dir")        
            .short("d")
            .long("dir")
            .help("working directory")
            .required(true)
            .min_values(1)
            .max_values(1))
      .arg(Arg::with_name("output-dir")        
            .short("o")
            .long("output-dir")
            .help("output directory")
            .required(true)
            .min_values(1)
            .max_values(1))            
      .arg(Arg::with_name("minchar")                    
            .long("minchar")
            .help("minimum chars in chunk")
            .required(true)
            .min_values(1)
            .max_values(1)
            .default_value(min_char.as_str()))
      .arg(Arg::with_name("maxchar")                    
            .long("maxchar")
            .help("maximum chars in chunk")
            .required(true)
            .min_values(1)
            .max_values(1)
            .default_value(max_char.as_str()))                        
      .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("verbose output")
            .required(false)
            .takes_value(false))      
      .arg(Arg::with_name("strip-prefix")
            .long("strip-prefix")                        
            .min_values(1)
            .max_values(1)
            .help("src prefix to strip"))
      .arg(Arg::with_name("prefix-replace")
            .long("prefix-replace")                        
            .min_values(1)
            .max_values(1)
            .help("replacement for src prefix"))
      .arg(Arg::with_name("web")
            .short("w")
            .long("web")
            .help("run web server")
            .required(false)
            .takes_value(false))            
      .get_matches();

      Ok(Config{
        working_dir : PathBuf::from(matches.value_of("working-dir").unwrap()),
        output_dir : matches.value_of("output-dir").unwrap().to_string(),
        min_chars : matches.value_of("minchar").map(parse_positive_int).unwrap()?,
        max_chars:  matches.value_of("maxchar").map(parse_positive_int).unwrap()?,
        strip_prefix: matches.value_of("strip-prefix").unwrap_or("").to_string(),
        prfx_replacement: matches.value_of("prefix-replace").unwrap_or("").to_string(),
        is_verbose: matches.is_present("verbose"),
        input_files: matches.value_of("input-files-list").unwrap_or("").to_string(),
        web: matches.is_present("web")      
      })

}

fn parse_positive_int(val: &str) -> Result<usize, Box<dyn Error>>{
      match val.parse() {
            Ok(n) if n>0 => Ok(n),
            _ => Err(From::from(val)),
      }
}

#[test]
fn test_parse_positive_int() {
      let res = parse_positive_int("1");
      assert!(res.is_ok());
      assert_eq!(res.unwrap(), 1);

      let res = parse_positive_int("no");
      assert!(res.is_err());
      assert_eq!(res.unwrap_err().to_string(), "no".to_string());

      let res = parse_positive_int("0");
      assert!(res.is_err());
      assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

