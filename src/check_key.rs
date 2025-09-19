use std::env;
use std::fs;
use std::path::PathBuf;
use std::time;

/// Time validation data container
struct TimeKey {
    /// Expiration timestamp for AlphaFold Toolkit license (Unix timestamp in seconds)
    alphafold_toolkit: usize,
}

/// Manages license validation for software tools
pub struct Key {
    /// Time-based validation fields container
    time_key: TimeKey,
    /// File content verification value
    file_key: usize,
}

impl Key {
    /// Creates a new license validation instance with default parameters
    ///
    /// # Returns
    ///
    /// `Key` struct containing predefined validation values
    pub fn new() -> Key {
        Key {
            time_key: TimeKey {
                alphafold_toolkit: 1789776000,
            },
            file_key: 178903,
        }
    }

    /// Validates software license against stored credentials
    ///
    /// # Arguments
    ///
    /// * `software` - Target software name for validation
    ///
    /// # Returns
    ///
    /// `true` if license is valid; `false` otherwise
    pub fn check(&self, software: &str) -> bool {
        let home_dir = env::var("HOME").unwrap_or(String::from("/home/icedragon"));
        let key_path = PathBuf::from(home_dir + ".icedragon_key");
        let file_key: usize = fs::read_to_string(key_path)
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        if file_key != self.file_key {
            return false;
        }
        let current = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if self.get_time(software) > current.try_into().unwrap() {
            true
        } else {
            false
        }
    }

    /// Retrieves time-based validation value for specified field
    ///
    /// # Arguments
    ///
    /// * `field_name` - Field identifier (e.g., "alphafold_toolkit")
    ///
    /// # Returns
    ///
    /// Validation timestamp in seconds if found, 0 otherwise
    fn get_time(&self, field_name: &str) -> usize {
        match field_name {
            "alphafold_toolkit" => self.time_key.alphafold_toolkit,
            _ => 0,
        }
    }
}
