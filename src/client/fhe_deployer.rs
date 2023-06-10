use crate::client::account_handler::{get_keys, KeyPair};
use std::process::Command;
use std::str;

// store the deployed address so that we can use it in other tests
static mut DEPLOYED_ADDRESS: Option<String> = None;

fn run_deployers_script(owner: &str) -> Option<String> {
    let output = Command::new("/home/shankar/.foundry/bin/forge")
        .arg("create")
        .arg("src/contracts/FHEToken.sol:FHEToken")
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

            unsafe {
                DEPLOYED_ADDRESS = Some(deployed_to.clone());
            }
            return Some(deployed_to);
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Script execution failed:\n{}", stderr);
    }

    None
}

pub fn get_deployed_address() -> &'static str {
    unsafe {
        if DEPLOYED_ADDRESS.is_none() {
            // The contract isn't deployed yet, so deploy it
            println!("Deploying contract...");
            let owner = get_keys("owner").unwrap();
            run_deployers_script(owner.public_key);
        }
        DEPLOYED_ADDRESS.as_ref().map(|s| s.as_str()).unwrap()
    }
}

#[test]
fn test_deployer() {
    unsafe {
        assert!(DEPLOYED_ADDRESS.is_none());
        println!("DEPLOYED_ADDRESS = {:?}", get_deployed_address());
        assert!(DEPLOYED_ADDRESS.is_some());
    }
}
