pub mod statics;

use crate::{
    functions::label_num,
    structs::duration::statics::{SECS_PER_DAY, SECS_PER_HOUR, SECS_PER_MIN, SECS_PER_MONTH, SECS_PER_WEEK, SECS_PER_YEAR},
};
use anyhow::Result;
use parse_duration::parse;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Duration {
    pub total_secs: u64,
    pub years: u64,
    pub months: u64,
    pub weeks: u64,
    pub days: u64,
    pub hours: u64,
    pub mins: u64,
    pub secs: u64,
}

impl Duration {
    pub fn new() -> Self {
        Self { total_secs: 0, years: 0, months: 0, weeks: 0, days: 0, hours: 0, mins: 0, secs: 0 }
    }

    pub fn parse<T: ToString>(mut self, duration: T) -> Result<Self> {
        self.total_secs = parse(&duration.to_string())?.as_secs();
        self.years = self.total_secs / SECS_PER_YEAR;

        let mut secs = self.total_secs - SECS_PER_YEAR * self.years;
        self.months = secs / SECS_PER_MONTH;

        secs -= SECS_PER_MONTH * self.months;
        self.weeks = secs / SECS_PER_WEEK;

        secs -= SECS_PER_WEEK * self.weeks;
        self.days = secs / SECS_PER_DAY;

        secs -= SECS_PER_DAY * self.days;
        self.hours = secs / SECS_PER_HOUR;

        secs -= SECS_PER_HOUR * self.hours;
        self.mins = secs / SECS_PER_MIN;

        self.secs = secs - SECS_PER_MIN * self.mins;

        Ok(self)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut units = vec![];

        // Years
        if self.years > 0 {
            units.push(label_num(self.years, "year"));
        }

        // Months
        if self.months > 0 {
            units.push(label_num(self.months, "month", "months"));
        }

        // Minutes
        if self.weeks > 0 {
            units.push(label_num(self.weeks, "week", "weeks"));
        }

        // Days
        if self.days > 0 {
            units.push(label_num(self.days, "day", "days"));
        }

        // Hours
        if self.hours > 0 {
            units.push(label_num(self.hours, "hour", "hours"));
        }

        // Minutes
        if self.mins > 0 {
            units.push(label_num(self.mins, "min", "mins"));
        }

        // Secs
        if self.secs > 0 {
            units.push(label_num(self.secs, "sec", "secs"));
        }

        write!(f, "{}", units.join(", "))
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self::new()
    }
}