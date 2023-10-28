use sysinfo::{DiskExt, System, SystemExt};
use std::path::Path;

pub fn has_enough_space(path: &Path, required_space: u64) -> bool {
    let mut system = System::new_all();
    system.refresh_all();

    let disks = system.disks();
    println!("Disks: {:?}", disks);
    for disk in disks {
        if disk.mount_point() == path {
            let available_space = disk.available_space();
            return available_space >= required_space;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::has_enough_space;
    use std::path::Path;

    #[test]
    fn test_has_enough_space() {
        let path = Path::new("C:\\");
        let required_space = 1;
        assert_eq!(has_enough_space(path, required_space), true);
    }

    #[test]
    fn test_not_enough_space() {
        let path = Path::new("C:\\");
        let required_space = 1_000_000_000_000_000_000;
        assert_eq!(has_enough_space(path, required_space), false);
    }
}