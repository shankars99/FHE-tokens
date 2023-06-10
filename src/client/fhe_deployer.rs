use crate::client::account_handler::{get_keys, KeyPair};
use std::process::Command;
use std::str;

fn run_deployers_script(owner: &str) -> Option<String> {
    let output = Command::new("/home/shankar/.foundry/bin/forge")
        .arg("create")
        .arg("src/FHEToken.sol:FHEToken")
        .arg("--constructor-args")
        .arg("8")
        .arg("--unlocked")
        .arg("--from")
        .arg(owner)
        .output()
        .expect("Failed to execute script");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap().trim().to_string();
        let deployed_to_line = stdout.lines().find(|line| line.starts_with("Deployed to:"));
        if let Some(deployed_to_line) = deployed_to_line {
            let deployed_to = deployed_to_line
                .split(":")
                .nth(1)
                .map(|address| address.trim().to_string())
                .unwrap();
            return Some(deployed_to);
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Script execution failed:\n{}", stderr);
    }

    None
}

#[test]
fn test_deployer() {
    let owner = get_keys("owner").unwrap();
    let deployed_to = run_deployers_script(owner.public_key);
    if let Some(deployed_address) = deployed_to {
        println!("Contract deployed to: {}", deployed_address);
        assert!(!deployed_address.is_empty());
    } else {
        println!("Failed to deploy contract");
        assert!(false);
    }
}
