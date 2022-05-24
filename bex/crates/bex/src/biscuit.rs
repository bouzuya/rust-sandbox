use std::{fmt::Display, str::FromStr};

use anyhow::Context;
use pocket::RetrieveItemResponse;
use serde::Serialize;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

#[derive(Debug, Serialize)]
pub struct Biscuit {
    id: String,
    title: String,
    url: String,
    added_at: BiscuitTimestamp,
}

impl TryFrom<RetrieveItemResponse> for Biscuit {
    type Error = anyhow::Error;

    fn try_from(value: RetrieveItemResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.item_id,
            title: value.given_title,
            url: value.given_url,
            added_at: BiscuitTimestamp::from_str(
                value.time_added.context("time_added is None")?.as_str(),
            )?,
        })
    }
}

#[derive(Debug)]
pub struct BiscuitTimestamp(OffsetDateTime);

impl Display for BiscuitTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.format(&Rfc3339).map_err(|_| std::fmt::Error)?
        )
    }
}

impl FromStr for BiscuitTimestamp {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unix_timestamp = s.parse::<i64>()?;
        let offset_date_time = OffsetDateTime::from_unix_timestamp(unix_timestamp)?;
        Ok(Self(offset_date_time))
    }
}

impl Serialize for BiscuitTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self.to_string().as_str())
    }
}
