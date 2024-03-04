extern crate clap;
use core::ops::Range;
use std::fs;
use std::path::PathBuf;
use clap::{Arg, App};
use text_splitter::TextSplitter;
use tokenizers::Tokenizer;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use serde_json::json;

fn main() {

  let matches = get_params();

  let working_dir = PathBuf::from(matches.value_of("working-dir").unwrap().to_string());
  let output_dir = matches.value_of("output-dir").unwrap().to_string();
  let input = matches.value_of_lossy("input-files-list").unwrap().to_string();
  let min_chars: usize = matches.value_of("minchar").unwrap().parse().expect("not a number!");
  let max_chars: usize = matches.value_of("maxchar").unwrap().parse().expect("not a number!");
  assert!(min_chars < max_chars);

  let is_verbose: bool = matches.is_present("verbose");

  let chunk_size_range = min_chars .. max_chars;
  let content = fs::read_to_string(input).unwrap();
  let list_of_files : Vec<&str> = content.split("\n").collect();
  assert!(list_of_files.len()>0);  

  let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None).unwrap();  
  let splitter = TextSplitter::new(tokenizer)
    .with_trim_chunks(true);
        
  for filename in list_of_files {
    let relative_path = PathBuf::from(filename);
    let path = working_dir.join(relative_path);
    let full_path = path.to_str().unwrap();   
    let _ = process_file(&splitter, full_path, &output_dir,
               "json", chunk_size_range.clone(),
               is_verbose);
  } 

}

fn get_params() -> clap::ArgMatches<'static> {
    let matches = App::new("semantic splitter")
      .version("0.1")
      .author("Ulan Sametov <usametov@gmail.com>")
      .about("performs semantic split")  
      .arg(Arg::with_name("input-files-list")        
            .short("i") 
            .long("input-files")
            .help("file containing list of relative paths to documents to process")
            .required(true)
            .min_values(1)
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
            .max_values(1))
      .arg(Arg::with_name("maxchar")                    
            .long("maxchar")
            .help("maximum chars in chunk")
            .required(true)
            .min_values(1)
            .max_values(1))                        
      .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("verbose output")
            .required(false)
            .takes_value(false))      
      .get_matches();
    matches
}

fn get_chunks<'a>(splitter: &'a TextSplitter<Tokenizer>, max_characters: std::ops::Range<usize>, txt: &'a str) -> impl Iterator<Item = &'a str> {
    
    // Optionally can also have the splitter trim whitespace for you    
    let chunks = splitter.chunks(txt, max_characters);
    chunks
}

fn process_file<'a>(splitter: &'a TextSplitter<Tokenizer>, 
                    input_path: &str, output: &str, new_extension: &str, 
                    chunk_chars_range: Range<usize>, is_verbose: bool) -> io::Result<()> {

    if is_verbose {
      println!("processing file {}", &input_path);
    }                  
    // Create a Path from the input_path string.    
    let path = Path::new(input_path);

    //let src_dir = PathBuf::from(path.parent().unwrap().to_str().unwrap());
    let output_dir = PathBuf::from(output);
    let filename = path.file_name().unwrap().to_str().unwrap();

    // Create a new path for the output file by changing the extension.   
    let output_path = output_dir.join(filename)
                                         .with_extension(new_extension);

    // Open the file in read-only mode.
    let mut input_file = File::open(&input_path)?;

    // Read the file's contents into a string.
    let mut content = String::new();    
    input_file.read_to_string(&mut content)?;
        
    let _chunks = get_chunks(splitter, chunk_chars_range, &content);
            
    let chunks = _chunks.collect::<Vec<_>>();
    let mut json_objects = vec![];
    
    for s in chunks {
      
      let object = json!({
        "src": &input_path,
        "chunk": s
      });
      json_objects.push(object);
    }

    let output = serde_json::to_string(&json_objects).unwrap();    
    // Open the output file in write mode.
    let mut output_file = File::create(&output_path)?;

    if is_verbose {
      println!("saving output to {:?}", &output_file);
    }
    // Write the contents to the output file.
    output_file.write_all(output.as_bytes())?;

    Ok(())
}

