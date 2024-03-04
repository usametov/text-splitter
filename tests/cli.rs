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

// uncomment this test, if you wan to run with src set to absolute path
// #[test]
// fn runs_nostrip() {
//     let mut cmd = Command::cargo_bin("text-splitter").unwrap();
//     let current_user = env::var("USER").unwrap();

//     cmd.args(["--input-files", &format!("/media/{}/ssd1/aiken/files2process.txt", current_user), 
//                "--dir", &format!("/media/{}/ssd1/aiken/aiken-lang-docs/src/", current_user),
//                "-o", &format!("/media/{}/ssd1/aiken/aiken-docs-processed/", current_user), 
//                "--minchar", "400", "--maxchar", "1200", "-v"]).assert().success();

// }

#[test]
fn runs_with_strip() {
    let mut cmd = Command::cargo_bin("text-splitter").unwrap();
    let current_user = env::var("USER").unwrap();

    cmd.args(["--input-files", &format!("/media/{}/ssd1/aiken/files2process.txt", current_user), 
               "--dir", &format!("/media/{}/ssd1/aiken/aiken-lang-docs/src/", current_user),
               "-o", &format!("/media/{}/ssd1/aiken/aiken-docs-processed/", current_user), 
               "--minchar", "400", "--maxchar", "1200", "-v",
               "--strip-prefix", &format!("/media/{}/ssd1/aiken/aiken-lang-docs/src/pages/", current_user),
               "--prefix-replace", "https://aiken-lang.org/"])
               .assert().success();

}



