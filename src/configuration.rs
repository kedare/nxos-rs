use super::command::run;
use anyhow::{Error, anyhow};
use chrono::prelude::*;

const CONFIG_TIME_FORMAT: &str = "%a %b %e %T %Y";

/// Extract the date from the Time line of the configuration
fn extract_date_from_time_line(line: String) -> Result<DateTime<Utc>, Error> {
    let date_vector: Vec<&str> = line.split(": ").collect();
    let date_string = match  date_vector.get(1) {
        Some(x) => Ok(x.trim()),
        None => Err(anyhow!("Could not extract date from line"))
    }?;
    let date = Utc.datetime_from_str(date_string, CONFIG_TIME_FORMAT)?;
    Ok(date)
}
/// Return the date of the last change on the startup configuration
pub fn get_startup_configuration_date() -> Result<DateTime<Utc>, Error> {
    let raw_date_string= run("show startup-config | begin '!Time:' | head -n 1".to_string())?.stdout;
    let date = extract_date_from_time_line(raw_date_string)?;
    Ok(date)
}

/// Return true if the device has a startup configuration, or false
pub fn has_startup_configuration() -> Result<bool, Error> {
    let command_result = run("show startup-config".to_string())?.stdout;
    Ok(!command_result.contains("No startup configuration"))
}

/// Save the running configuration into the startup configuration
pub fn save_configuration() -> Result<(), Error> {
    let result = run("copy running-config startup-config".to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::configuration::extract_date_from_time_line;
    use chrono::{Utc, TimeZone};

    #[test]
    fn extract_good_date() {
        let line = "!Time: Mon Jun 28 13:22:05 2021";
        let date = extract_date_from_time_line(line.to_string());
        assert_eq!(date.unwrap(), Utc.ymd(2021, 6, 28).and_hms(13, 22, 5));
    }

    #[test]
    fn extract_bad_dates() {
        let lines = vec![
            "Mon Jun 28 13:22:05 2021",
            "Sun Jan 99 12:34:78 2099"
        ];

        for line in lines {
            let date = extract_date_from_time_line(line.to_string());
            assert_eq!(date.is_err(), true)
        }

    }
}