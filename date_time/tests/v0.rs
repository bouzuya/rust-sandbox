use std::{convert::TryFrom, str::FromStr};

use date_time::{
    DayOfMonth, Instant, LocalDate, LocalDateTime, Month, OffsetDateTime, Year, YearMonth,
};

#[test]
fn use_case_offset_date_time_plus_days() -> anyhow::Result<()> {
    let plus_days =
        |offset_date_time: OffsetDateTime, days: u64| -> anyhow::Result<OffsetDateTime> {
            // TODO: offset_date_time + duration
            let instant = offset_date_time.instant();
            let time_zone_offset = offset_date_time.time_zone_offset();

            // TODO: instant + duration
            let seconds = u64::from(instant);
            let updated_seconds = seconds + days * 86400;
            let updated_instant = Instant::try_from(updated_seconds)?;

            let updated_offset_date_time =
                OffsetDateTime::from_instant(updated_instant, time_zone_offset);
            Ok(updated_offset_date_time)
        };

    let offset_date_time = OffsetDateTime::from_str("2021-02-03T04:05:06+09:00")?;
    let days = 2;
    let updated_offset_date_time = plus_days(offset_date_time, days)?;
    assert_eq!(
        updated_offset_date_time.to_string(),
        "2021-02-05T04:05:06+09:00"
    );

    Ok(())
}

#[test]
fn use_case_offset_date_time_with_day_of_month() -> anyhow::Result<()> {
    let with_day_of_month =
        |offset_date_time: OffsetDateTime, day_of_month: u8| -> anyhow::Result<OffsetDateTime> {
            let local_date_time = offset_date_time.local_date_time();
            let time_zone_offset = offset_date_time.time_zone_offset();
            let local_date = local_date_time.date();
            let local_time = local_date_time.time();

            // TODO: day_of_month -> local_date -> local_date
            let day_of_month = DayOfMonth::try_from(day_of_month)?;
            let updated_local_date =
                LocalDate::from_ymd(local_date.year(), local_date.month(), day_of_month)?;

            let updated_local_date_time = LocalDateTime::from_dt(updated_local_date, local_time);
            let updated_offset_date_time =
                OffsetDateTime::new(updated_local_date_time, time_zone_offset);
            Ok(updated_offset_date_time)
        };

    let offset_date_time = OffsetDateTime::from_str("2021-02-03T04:05:06+09:00")?;
    let day_of_month = 14;
    let updated_offset_date_time = with_day_of_month(offset_date_time, day_of_month)?;
    assert_eq!(
        updated_offset_date_time.to_string(),
        "2021-02-14T04:05:06+09:00"
    );

    Ok(())
}

#[test]
fn use_case_offset_date_time_next_date() -> anyhow::Result<()> {
    let next_date = |offset_date_time: OffsetDateTime| -> anyhow::Result<OffsetDateTime> {
        let local_date_time = offset_date_time.local_date_time();
        let time_zone_offset = offset_date_time.time_zone_offset();
        let local_date = local_date_time.date();
        let local_time = local_date_time.time();

        // TODO: local_date -> local_date // next date
        let updated_local_date =
            if local_date.day_of_month() == local_date.year_month().last_day_of_month() {
                let day_of_month = DayOfMonth::try_from(1)?;
                // TODO: year_month next month
                match local_date.month().succ() {
                    Some(next_month) => {
                        LocalDate::from_ymd(local_date.year(), next_month, day_of_month)?
                    }
                    None => {
                        // TODO: year next year
                        let year_as_u16 = u16::from(local_date.year());
                        let next_year_as_u16 = year_as_u16 + 1;
                        let next_year = Year::try_from(next_year_as_u16)?;
                        LocalDate::from_ymd(next_year, Month::try_from(1)?, day_of_month)?
                    }
                }
            } else {
                // TODO: day of month next day
                let day_of_month_as_u8 = u8::from(local_date.day_of_month());
                let next_day_of_month_as_u8 = day_of_month_as_u8 + 1;
                let next_day_of_month = DayOfMonth::try_from(next_day_of_month_as_u8)?;
                LocalDate::from_ymd(local_date.year(), local_date.month(), next_day_of_month)?
            };

        let updated_local_date_time = LocalDateTime::from_dt(updated_local_date, local_time);
        let updated_offset_date_time =
            OffsetDateTime::new(updated_local_date_time, time_zone_offset);
        Ok(updated_offset_date_time)
    };

    let offset_date_time = OffsetDateTime::from_str("2021-02-03T04:05:06+09:00")?;
    let updated_offset_date_time = next_date(offset_date_time)?;
    assert_eq!(
        updated_offset_date_time.to_string(),
        "2021-02-04T04:05:06+09:00"
    );

    Ok(())
}

#[test]
fn use_case_offset_date_time_next_month() -> anyhow::Result<()> {
    let next_month = |offset_date_time: OffsetDateTime| -> anyhow::Result<OffsetDateTime> {
        let local_date_time = offset_date_time.local_date_time();
        let time_zone_offset = offset_date_time.time_zone_offset();
        let local_date = local_date_time.date();
        let local_time = local_date_time.time();
        let year_month = local_date.year_month();

        // TODO: local_date -> local_date // next date
        // TODO: year_month next month
        let updated_year_month = match local_date.month().succ() {
            Some(next_month) => YearMonth::new(year_month.year(), next_month),
            None => {
                // TODO: year next year
                let year_as_u16 = u16::from(year_month.year());
                let next_year_as_u16 = year_as_u16 + 1;
                let next_year = Year::try_from(next_year_as_u16)?;
                YearMonth::new(next_year, Month::try_from(1)?)
            }
        };
        let updated_local_date = LocalDate::from_ymd(
            updated_year_month.year(),
            updated_year_month.month(),
            local_date.day_of_month(),
        )?;
        let updated_local_date_time = LocalDateTime::from_dt(updated_local_date, local_time);
        let updated_offset_date_time =
            OffsetDateTime::new(updated_local_date_time, time_zone_offset);
        Ok(updated_offset_date_time)
    };

    let offset_date_time = OffsetDateTime::from_str("2021-02-03T04:05:06+09:00")?;
    let updated_offset_date_time = next_month(offset_date_time)?;
    assert_eq!(
        updated_offset_date_time.to_string(),
        "2021-03-03T04:05:06+09:00"
    );

    Ok(())
}
