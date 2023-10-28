use std::env;

pub fn detect_os() -> String {
    return if cfg!(target_os = "windows") {
        "Windows".to_string()
    } else if cfg!(target_os = "macos") {
        "macOS".to_string()
    } else if cfg!(target_os = "linux") {
        "Linux".to_string()
    } else {
        "Unknown".to_string()
    }

}

pub fn detect_os_version() -> String {
    env::consts::OS.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_os() {
        let os = detect_os();
        assert!(os == "Windows" || os == "macOS" || os == "Linux" || os == "Unknown");
    }

    #[test]
    fn test_detect_os_version() {
        let os_version = detect_os_version();
        println!("{}", os_version);
        assert!(!os_version.is_empty());
    }
}