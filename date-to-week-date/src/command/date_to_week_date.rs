use time::{macros::format_description, Date};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Command {
    date: String,
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid date format")]
    InvalidDateFormat,
    #[error("invalid week date format")]
    InvalidWeekDateFormat,
}

pub fn handle(command: Command) -> Result<String, Error> {
    let date_format = format_description!("[year]-[month]-[day]");
    let date = Date::parse(&command.date, &date_format).map_err(|_| Error::InvalidDateFormat)?;
    let week_date_format =
        format_description!("[year base:iso_week]-W[week_number repr:iso]-[weekday repr:monday]");
    date.format(&week_date_format)
        .map_err(|_| Error::InvalidWeekDateFormat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        assert_eq!(
            handle(Command {
                date: "2023-01-01".to_owned(),
            })?,
            "2022-W52-7".to_owned()
        );
        assert_eq!(
            handle(Command {
                date: "2023-01-02".to_owned(),
            })?,
            "2023-W01-1".to_owned()
        );
        assert_eq!(
            handle(Command {
                date: "2023-01-1".to_owned(),
            }),
            Err(Error::InvalidDateFormat)
        );
        Ok(())
    }
}
