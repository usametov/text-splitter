use core::ops::Range;
use std::fs;
use std::path::PathBuf;

use text_splitter::TextSplitter;
use tokenizers::Tokenizer;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use serde_json::json;


fn get_chunks<'a>(splitter: &'a TextSplitter<Tokenizer>, 
                  max_characters: std::ops::Range<usize>, 
                  txt: &'a str) -> impl Iterator<Item = &'a str> {
    
    // Optionally can also have the splitter trim whitespace for you    
    let chunks = splitter.chunks(txt, max_characters);
    chunks
}

fn process_file(splitter: &TextSplitter<Tokenizer>, 
                    input_path: &str, output: &str, new_extension: &str, 
                    chunk_chars_range: Range<usize>, is_verbose: bool, 
                    strp_prfx: &str, prfx_replacement: &str) -> io::Result<()> {

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

    let content = fs::read_to_string(input_path)?;
        
    let _chunks = get_chunks(splitter, chunk_chars_range, &content);
            
    let chunks = _chunks.collect::<Vec<_>>();
    let mut json_objects = vec![];

    let src = get_src(strp_prfx, prfx_replacement, input_path);
    
    for (index, &s) in chunks.iter().enumerate() {      
      let object = json!({
        "src": src,
        "seq_id": index,
        "chunk": s
      });
      json_objects.push(object);
    }

    let output = serde_json::to_string(&json_objects).unwrap();    
    // Open the output file in write mode.
    let mut output_file = File::create(output_path)?;

    if is_verbose {
      println!("saving output to {:?}", &output_file);
    }
    // Write the contents to the output file.
    output_file.write_all(output.as_bytes())?;

    Ok(())
}

fn get_src(strp_prfx: &str, prfx_replacement: &str, input_path: &str) -> String {
  
    if !strp_prfx.is_empty() && !prfx_replacement.is_empty() {
                      format!("{}{}", prfx_replacement, input_path.to_string().strip_prefix(strp_prfx).unwrap())
                    } 
                    else {
                      input_path.to_string()
                  }    
}

pub(crate) fn run(list_of_files: Vec<String>
    , working_dir: PathBuf
    , output_dir: &str
    , chunk_size_range: Range<usize>
    , is_verbose: bool
    , prfx_replacement: &str
    , strip_prefix: &str) {
  
  let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None).unwrap();  
  let splitter = TextSplitter::new(tokenizer)
    .with_trim_chunks(true);
        
  for filename in list_of_files {
    let relative_path = PathBuf::from(filename);
    let path = working_dir.join(relative_path);
    let full_path = path.to_str().unwrap();   
    let _ = process_file(&splitter, full_path, &output_dir,
               "json", chunk_size_range.clone(),
               is_verbose, strip_prefix, prfx_replacement);
  }
} 

//the idea is that we may want to strip the part of path and replace it with something else
//for example, you do not want to show the name of your home directory 
#[test]
fn test_src() {    
    let input = "/media/user/aiken/pages/p1.md";
    let strip_prfx = "/media/user/aiken";
    let replacement = "github.com/aiken";

    assert_eq!(get_src(strip_prfx, replacement, input), "github.com/aiken/pages/p1.md");
}
