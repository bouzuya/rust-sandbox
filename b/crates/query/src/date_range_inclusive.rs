use std::ops::{Bound, RangeBounds, RangeInclusive};

use limited_date_time::{Date, Year, YearMonth};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateRangeInclusive(RangeInclusive<Date>);

impl DateRangeInclusive {
    pub fn from_date(date: Date) -> Self {
        Self::new(date, date)
    }

    pub fn from_year(year: Year) -> Self {
        let start = Date::first_date_of_year(year);
        let end = Date::last_date_of_year(year);
        Self::new(start, end)
    }

    pub fn from_year_month(year_month: YearMonth) -> Self {
        let start = Date::first_date_of_month(year_month);
        let end = Date::last_date_of_month(year_month);
        Self::new(start, end)
    }

    pub fn new(start: Date, end: Date) -> Self {
        Self(RangeInclusive::new(start, end))
    }

    pub fn contains(&self, item: &Date) -> bool {
        self.0.contains(item)
    }

    pub fn into_inner(self) -> (Date, Date) {
        self.0.into_inner()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn start(&self) -> &Date {
        self.0.start()
    }

    pub fn end(&self) -> &Date {
        self.0.end()
    }
}

impl RangeBounds<Date> for DateRangeInclusive {
    fn start_bound(&self) -> Bound<&Date> {
        self.0.start_bound()
    }

    fn end_bound(&self) -> Bound<&Date> {
        self.0.end_bound()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn from_date_test() -> anyhow::Result<()> {
        let date = Date::from_str("2021-02-03")?;
        assert_eq!(
            DateRangeInclusive::from_date(date),
            DateRangeInclusive::new(date, date)
        );
        Ok(())
    }

    #[test]
    fn from_year_test() -> anyhow::Result<()> {
        let year = Year::from_str("2021")?;
        let first_date_of_year = Date::first_date_of_year(year);
        let last_date_of_year = Date::last_date_of_year(year);
        assert_eq!(
            DateRangeInclusive::from_year(year),
            DateRangeInclusive::new(first_date_of_year, last_date_of_year)
        );
        Ok(())
    }

    #[test]
    fn from_year_month_test() -> anyhow::Result<()> {
        let year_month = YearMonth::from_str("2021-02")?;
        let first_date_of_month = Date::first_date_of_month(year_month);
        let last_date_of_month = Date::last_date_of_month(year_month);
        assert_eq!(
            DateRangeInclusive::from_year_month(year_month),
            DateRangeInclusive::new(first_date_of_month, last_date_of_month)
        );
        Ok(())
    }

    #[test]
    fn new_test() -> anyhow::Result<()> {
        let start = Date::from_str("2021-02-03")?;
        let end = Date::from_str("2021-02-05")?;
        let date_range_inclusive = DateRangeInclusive::new(start, end);
        assert_eq!(date_range_inclusive.start(), &start);
        assert_eq!(date_range_inclusive.end(), &end);
        Ok(())
    }

    #[test]
    fn contains_test() -> anyhow::Result<()> {
        let start = Date::from_str("2021-02-03")?;
        let end = Date::from_str("2021-02-05")?;
        let date_range_inclusive = DateRangeInclusive::new(start, end);
        let d1 = Date::from_str("2021-02-02")?;
        let d2 = Date::from_str("2021-02-04")?;
        let d3 = Date::from_str("2021-02-06")?;
        assert!(!date_range_inclusive.contains(&d1));
        assert!(date_range_inclusive.contains(&start));
        assert!(date_range_inclusive.contains(&d2));
        assert!(date_range_inclusive.contains(&end));
        assert!(!date_range_inclusive.contains(&d3));
        Ok(())
    }

    #[test]
    fn into_inner_test() -> anyhow::Result<()> {
        let start = Date::from_str("2021-02-03")?;
        let end = Date::from_str("2021-02-05")?;
        let date_range_inclusive = DateRangeInclusive::new(start, end);
        assert_eq!(date_range_inclusive.into_inner(), (start, end));
        Ok(())
    }

    #[test]
    fn is_empty_test() -> anyhow::Result<()> {
        let d1 = Date::from_str("2021-02-03")?;
        let d2 = Date::from_str("2021-02-05")?;
        assert!(!DateRangeInclusive::new(d1, d2).is_empty());
        assert!(DateRangeInclusive::new(d2, d1).is_empty());
        Ok(())
    }

    #[test]
    fn start_test() -> anyhow::Result<()> {
        let start = Date::from_str("2021-02-03")?;
        let end = Date::from_str("2021-02-05")?;
        assert_eq!(DateRangeInclusive::new(start, end).start(), &start);
        Ok(())
    }

    #[test]
    fn end_test() -> anyhow::Result<()> {
        let start = Date::from_str("2021-02-03")?;
        let end = Date::from_str("2021-02-05")?;
        assert_eq!(DateRangeInclusive::new(start, end).end(), &end);
        Ok(())
    }

    #[test]
    fn range_bounds_start_bound_test() -> anyhow::Result<()> {
        let start = Date::from_str("2021-02-03")?;
        let end = Date::from_str("2021-02-05")?;
        assert_eq!(
            DateRangeInclusive::new(start, end).start_bound(),
            Bound::Included(&start)
        );
        Ok(())
    }

    #[test]
    fn range_bounds_end_bound_test() -> anyhow::Result<()> {
        let start = Date::from_str("2021-02-03")?;
        let end = Date::from_str("2021-02-05")?;
        assert_eq!(
            DateRangeInclusive::new(start, end).end_bound(),
            Bound::Included(&end)
        );
        Ok(())
    }
}
