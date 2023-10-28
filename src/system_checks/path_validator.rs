use std::path::{Path};

pub fn is_valid_path(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }

    if !path.is_dir() {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_valid_path;
    use std::path::Path;

    #[test]
    fn test_valid_path() {
        let valid_path = Path::new(".");
        assert_eq!(is_valid_path(valid_path), true);
    }

    #[test]
    fn test_invalid_path() {
        let invalid_path = Path::new("/some/nonexistent/path");
        assert_eq!(is_valid_path(invalid_path), false);
    }
}