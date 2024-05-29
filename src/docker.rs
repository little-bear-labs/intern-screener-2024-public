use log::{error, info};
use std::{env, process::Command, str::from_utf8};

// As the image is stopped after every run we have to get the lastest container id Ex., 93ba9f52e9ea
fn get_container_id() -> Option<String> {
    let image = "ghcr.io/little-bear-labs/lbl-test-proxy:latest";
    let output = Command::new("docker")
        .args(["ps", "-aq", "--filter", &format!("ancestor={}", image)])
        .output()
        .ok()?;
    let output_str = from_utf8(&output.stdout).ok()?;
    let container_id = output_str.lines().next()?.trim().to_string();
    Some(container_id)
}

/// Extracts the result file from the docker container
pub fn extract_result_file() -> std::io::Result<()> {
    let local_path = env::current_dir()
        .unwrap_or_else(|e| panic!("Failed to get current directory: {}", e))
        .join("test1.result.json");

    let container_id = match get_container_id() {
        Some(id) => id,
        None => {
            eprintln!("Failed to retrieve container ID");
            return Ok(());
        }
    };

    // Copy file from the container to the local current directory
    let output = Command::new("docker")
        .args([
            "cp",
            &format!("{}:/artifact/test1.result.json", container_id),
            local_path.to_str().unwrap(),
        ])
        .output()?;

    if output.status.success() {
        info!("Extracted file to {}", local_path.display());
    } else {
        error!("Failed to copy result file: {:?}", output);
    }

    Ok(())
}

// test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_container_id() {
        let container_id = get_container_id();
        assert!(container_id.is_some());
    }

    #[test]
    fn test_extract_result_file() {
        let result = extract_result_file();
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_result_file_creates_file() {
        extract_result_file().unwrap();
        let local_path = env::current_dir()
            .unwrap_or_else(|e| panic!("Failed to get current directory: {}", e))
            .join("test1.result.json");
        assert!(local_path.exists());
        // fs::remove_file(local_path).unwrap(); // Clean up the test artifact
    }
}
