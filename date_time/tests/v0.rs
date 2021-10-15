use std::{convert::TryFrom, str::FromStr};

use date_time::{Instant, OffsetDateTime};

#[test]
fn use_case_plus_days() -> anyhow::Result<()> {
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
