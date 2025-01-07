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
fn runs_nostrip() {
    let mut cmd = Command::cargo_bin("text-splitter").unwrap();
    cmd.args(["--input-files", "tests/inputs/files2process.txt", 
            "--dir", "tests/inputs/", "-o", "tests/outputs/", 
            "--minchar", "400", "--maxchar", "1200", "-v"]).assert().success();
}
    
#[test]
#[ignore]
fn runs_with_strip() {
    let mut cmd = Command::cargo_bin("text-splitter").unwrap();
    let current_user = env::var("USER").unwrap();

    cmd.args(["--input-files", &format!("/media/{}/ssd1/aiken/files2process.txt", current_user), 
                "--dir", &format!("/media/{}/ssd1/aiken/aiken-lang-docs/src/", current_user),
                "-o", &format!("/media/{}/ssd1/aiken/aiken-docs-processed/", current_user), 
                "--minchar", "400", "--maxchar", "1200",
                "--strip-prefix", &format!("/media/{}/ssd1/aiken/aiken-lang-docs/src/pages/", current_user),
                "--prefix-replace", "https://aiken-lang.org/"])
                .assert().success();

}

#[test]
fn runs_with_strip_current_dir() {
    let mut cmd = Command::cargo_bin("text-splitter").unwrap();

    cmd.args(["--input-files", "tests/inputs/files2process.txt", 
                "--dir", "tests/inputs/", "-o", "tests/outputs/", 
                "--minchar", "200", "--maxchar", "500"
                , "--strip-prefix", "tests/inputs/"
                , "--prefix-replace", "https://aiken-lang.org/" ]).assert().success();        

}

//#[test]
fn start_webserver() {

    let mut cmd = Command::cargo_bin("text-splitter").unwrap();
    
    cmd.args(["--dir", "tests/inputs/", "-o", "tests/outputs/", 
              "--minchar", "200", "--maxchar", "500",
              "-w" ]).assert().success();
}

