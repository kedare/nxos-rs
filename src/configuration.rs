use super::command::run;
use anyhow::{anyhow, Error};
use chrono::prelude::*;
use std::fs;
use std::path::Path;

const CONFIG_RUNNING_CONFIG_PATH: &str = "/mnt/cfg/0/ascii/ascii_cfg.tar.gz";

/// Return the date of the last change on the startup configuration
pub fn get_startup_configuration_date() -> Result<DateTime<Utc>, Error> {
    let config_file = fs::metadata(CONFIG_RUNNING_CONFIG_PATH);

    match config_file {
        Ok(x) => {
            let date = DateTime::from(x.modified()?);
            return Ok(date);
        }
        Err(e) => {
            return Err(anyhow!(
                "Failed to get the startup configuration metadata: {}",
                e
            ))
        }
    }
}

/// Return true if the device has a startup configuration, or false
pub fn has_startup_configuration() -> bool {
    Path::new(CONFIG_RUNNING_CONFIG_PATH).exists()
}

/// Save the running configuration into the startup configuration
pub fn save_configuration() -> Result<(), Error> {
    let result = run("copy running-config startup-config".to_string())?;
    Ok(())
}
