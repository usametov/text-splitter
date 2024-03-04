use assert_cmd::Command; 
use predicates::prelude::*;
use std::env;

#[test]
fn dies_no_args() {

    let mut cmd = Command::cargo_bin("text-splitter").unwrap();
    cmd.assert()
       .failure()
       .stderr(predicate::str::contains("USAGE"));
    
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("text-splitter").unwrap();
    let current_user = env::var("USER").unwrap();

    cmd.args(["--input-files", &format!("/media/{}/ssd1/aiken/files2process.txt", current_user), 
               "--dir", &format!("/media/{}/ssd1/aiken/aiken-lang-docs/src/", current_user),
               "-o", &format!("/media/{}/ssd1/aiken/aiken-docs-processed/", current_user), 
               "--minchar", "200", "--maxchar", "700", "-v"]).assert().success();

}



