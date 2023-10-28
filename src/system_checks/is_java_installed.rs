use std::process::Command;

pub fn is_specific_java_version_installed(required_version: &str) -> bool {
    let output = Command::new("java")
        .arg("-version")
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    stderr.contains(required_version)
}

pub fn is_java_installed() -> bool {
    let output = Command::new("java")
        .arg("-version")
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            stdout.contains("version") || stderr.contains("version")
        },
        Err(_) => false
    }
}

#[cfg(test)]
mod tests {
    use super::is_java_installed;
    use super::is_specific_java_version_installed;

    /*#[test]
    fn test_is_java_not_installed() {
        assert_eq!(is_java_installed(), false);
    }*/

    #[test]
    fn test_is_java_installed() {
        assert_eq!(is_java_installed(), true);
    }
    
    #[test]
    fn test_is_specific_java_version_installed() {
        assert_eq!(is_specific_java_version_installed("17.0.9"), true);
    }

    #[test]
    fn test_is_specific_java_version_not_installed() {
        assert_eq!(is_specific_java_version_installed("1.0.0"), false);
    }
}