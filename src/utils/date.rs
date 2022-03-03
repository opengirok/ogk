use chrono::prelude::*;
use std::convert::From;

pub struct KstDateTime {
  pub datetime: DateTime<FixedOffset>,
}

impl KstDateTime {
  pub fn format(&self, format: Option<&str>) -> String {
    let _format = match format {
      Some(f) => f,
      None => "%Y-%m-%d",
    };
    self.datetime.format(_format).to_string()
  }
}

impl From<DateTime<Utc>> for KstDateTime {
  fn from(datetime: DateTime<Utc>) -> KstDateTime {
    KstDateTime {
      datetime: datetime.with_timezone(&FixedOffset::east(9 * 3600)), // KST +09:00
    }
  }
}
