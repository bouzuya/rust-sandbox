use crate::bbn_date_range;

pub fn date_range(month: String, week_date: bool) -> anyhow::Result<()> {
    Ok(bbn_date_range(month, week_date)?)
}
