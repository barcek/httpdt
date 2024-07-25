//! # Datetime
//!
//! A datetime struct for HTTP clients and servers.

use crate::date::Date;
use crate::time::Time;

use std::time::SystemTime;
use std::fmt::{self, Display, Formatter};
use std::error::Error;

/// Stores the date, time and raw seconds since the epoch,
/// with constructor, core methods for update (`now`) and
/// output as a HTTP Date header timestamp (`for_header`),
/// utility methods for construction via diff (`set`) and
/// current number of seconds since the epoch (`raw`) and
/// a `Default` implementation for the Unix epoch values.
///
/// # Example
///
/// ```
/// use httpdt::Datetime;
///
/// let dt = Datetime::new()
///   .unwrap();
///
/// let ts_initial = dt
///   .for_header();
///
/// // ...
///
/// let ts_updated = dt
///   .now()
///   .unwrap()
///   .for_header();
/// ```
#[derive(Default, PartialEq, Debug)]
pub struct Datetime {
  pub date: Date,
  pub time: Time,
  pub secs: u64
}

impl Datetime {

  pub fn new() -> Result<Self, Box<dyn Error>> {
    let new = Self::default().now()?;
    Ok (new)
  }

  pub fn raw() -> Result<u64, Box<dyn Error>> {
    let raw = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_secs();
    Ok (raw)
  }

  pub fn now(&self) -> Result<Self, Box<dyn Error>> {
    let raw = Self::raw()?;
    let now = self.set(raw);
    Ok (now)
  }

  pub fn set(&self, secs: u64) -> Self {
    let date = self.date.skip(secs - self.secs);
    let time = Time::from(secs);
    Self { date, time, secs }
  }

  pub fn for_header(&self) -> String {
    ImfFixdate(self).to_string()
  }
}

// ImfFixdate

struct ImfFixdate<'a>(&'a Datetime);

impl Display for ImfFixdate<'_> {

  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let ImfFixdate(dt) = self;
    write!(f, "{} {} GMT",
      dt.date.for_header(),
      dt.time.for_header()
    )
  }
}

#[cfg(test)]
mod test {

  use super::Datetime;
  use crate::date::{self, D_AS_S, test::{M_28_AS_S, M_29_AS_S, M_30_AS_S, M_31_AS_S, Y_365_AS_S, Y_366_AS_S}};
  use crate::time::{self, Time, M_AS_S, H_AS_M, D_AS_H};

  use std::time::{SystemTime, Duration};
  use std::thread::sleep;

  // 1970
  const JAN_01_1970_00_00_00: Datetime = Datetime {
    date: date::test::JAN_01_1970_00_00_00,
    time: time::test::JAN_01_1970_00_00_00,
    secs: 0
  };
  const FEB_28_1970_23_59_59: Datetime = Datetime {
    date: date::test::FEB_28_1970_23_59_59,
    time: Time { h: D_AS_H - 1, m: H_AS_M - 1, s: M_AS_S - 1, xs:                    M_31_AS_S                     + M_28_AS_S - D_AS_S },
    secs: M_31_AS_S + M_28_AS_S - 1
  };
  const MAR_01_1970_00_00_00: Datetime = Datetime {
    date: date::test::MAR_01_1970_00_00_00,
    time: Time { h:          0, m:          0, s:          0, xs:                    M_31_AS_S                     + M_28_AS_S          },
    secs: M_31_AS_S + M_28_AS_S
  };
  const APR_30_1970_23_59_59: Datetime = Datetime {
    date: date::test::APR_30_1970_23_59_59,
    time: Time { h: D_AS_H - 1, m: H_AS_M - 1, s: M_AS_S - 1, xs:                    M_31_AS_S * 2 + M_30_AS_S     + M_28_AS_S - D_AS_S },
    secs: M_31_AS_S * 2 + M_30_AS_S + M_28_AS_S - 1
  };
  const MAY_01_1970_00_00_00: Datetime = Datetime {
    date: date::test::MAY_01_1970_00_00_00,
    time: Time { h:          0, m:          0, s:          0, xs:                    M_31_AS_S * 2 + M_30_AS_S     + M_28_AS_S          },
    secs: M_31_AS_S * 2 + M_30_AS_S + M_28_AS_S
  };
  const JUL_31_1970_23_59_59: Datetime = Datetime {
    date: date::test::JUL_31_1970_23_59_59,
    time: Time { h: D_AS_H - 1, m: H_AS_M - 1, s: M_AS_S - 1, xs:                    M_31_AS_S * 4 + M_30_AS_S * 2 + M_28_AS_S - D_AS_S },
    secs: M_31_AS_S * 4 + M_30_AS_S * 2 + M_28_AS_S - 1
  };
  const SEP_01_1970_00_00_00: Datetime = Datetime {
    date: date::test::SEP_01_1970_00_00_00,
    time: Time { h:          0, m:          0, s:          0, xs:                    M_31_AS_S * 5 + M_30_AS_S * 2 + M_28_AS_S          },
    secs: M_31_AS_S * 5 + M_30_AS_S * 2 + M_28_AS_S
  };
  const DEC_31_1970_23_59_59: Datetime = Datetime {
    date: date::test::DEC_31_1970_23_59_59,
    time: Time { h: D_AS_H - 1, m: H_AS_M - 1, s: M_AS_S - 1, xs: Y_365_AS_S                                                   - D_AS_S },
    secs: Y_365_AS_S - 1
  };

  // 1972
  const JAN_01_1972_00_00_00: Datetime = Datetime {
    date: date::test::JAN_01_1972_00_00_00,
    time: Time { h:          0, m:          0, s:          0, xs: Y_365_AS_S *  2                                                       },
    secs: Y_365_AS_S * 2
  };
  const FEB_29_1972_23_59_59: Datetime = Datetime {
    date: date::test::FEB_29_1972_23_59_59,
    time: Time { h: D_AS_H - 1, m: H_AS_M - 1, s: M_AS_S - 1, xs: Y_365_AS_S *  2 + M_31_AS_S                      + M_29_AS_S - D_AS_S },
    secs: Y_365_AS_S * 2 + M_31_AS_S + M_29_AS_S - 1
  };
  const MAR_01_1972_00_00_00: Datetime = Datetime {
    date: date::test::MAR_01_1972_00_00_00,
    time: Time { h:          0, m:          0, s:          0, xs: Y_365_AS_S *  2 + M_31_AS_S                      + M_29_AS_S          },
    secs: Y_365_AS_S * 2 + M_31_AS_S + M_29_AS_S
  };
  const DEC_31_1972_23_59_59: Datetime = Datetime {
    date: date::test::DEC_31_1972_23_59_59,
    time: Time { h: D_AS_H - 1, m: H_AS_M - 1, s: M_AS_S - 1, xs: Y_365_AS_S *  2 + Y_366_AS_S                                 - D_AS_S },
    secs: Y_365_AS_S * 2 + Y_366_AS_S - 1
  };

  // 2000
  const JAN_01_2000_00_00_00: Datetime = Datetime {
    date: date::test::JAN_01_2000_00_00_00,
    time: Time { h:          0, m:          0, s:          0, xs: Y_365_AS_S * 23 + Y_366_AS_S *  7                                     },
    secs: Y_365_AS_S * 23 + Y_366_AS_S * 7
  };
  const DEC_31_2000_23_59_59: Datetime = Datetime {
    date: date::test::DEC_31_2000_23_59_59,
    time: Time { h: D_AS_H - 1, m: H_AS_M - 1, s: M_AS_S - 1, xs: Y_365_AS_S * 23 + Y_366_AS_S *  8                            - D_AS_S },
    secs: Y_365_AS_S * 23 + Y_366_AS_S * 8 - 1
  };

  // 2024
  const DEC_31_2024_23_59_59: Datetime = Datetime {
    date: date::test::DEC_31_2024_23_59_59,
    time: Time { h: D_AS_H - 1, m: H_AS_M - 1, s: M_AS_S - 1, xs: Y_365_AS_S * 41 + Y_366_AS_S * 14                            - D_AS_S },
    secs: Y_365_AS_S * 41 + Y_366_AS_S * 14 - 1
  };

  #[test]
  fn datetime_default() {

    assert_eq!(JAN_01_1970_00_00_00, Datetime::default());
  }

  #[test]
  fn datetime_raw() {

    let st_raw = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH).unwrap()
      .as_secs();

    assert_eq!(st_raw, Datetime::raw().unwrap());
  }

  #[test]
  fn datetime_new() {

    let st_raw = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH).unwrap()
      .as_secs();

    let dt_new = Datetime::new().unwrap();

    assert_eq!(st_raw, dt_new.secs);
    assert_eq!(st_raw, dt_new.date.xs + dt_new.time.xs);
  }

  #[test]
  fn datetime_now() {

    let dt_new = Datetime::new().unwrap();

    sleep(Duration::from_secs(1));

    let dt_now = dt_new.now().unwrap();

    assert_eq!(dt_new.secs + 1, dt_now.secs);
    assert_eq!(dt_new.date.xs + dt_new.time.xs + 1, dt_now.date.xs + dt_now.time.xs);
  }

  #[test]
  fn datetime_set() {

    // 1970
    assert_eq!(FEB_28_1970_23_59_59, JAN_01_1970_00_00_00.set(                  M_31_AS_S                     + M_28_AS_S - 1));
    assert_eq!(MAR_01_1970_00_00_00, FEB_28_1970_23_59_59.set(                  M_31_AS_S                     + M_28_AS_S    ));
    assert_eq!(APR_30_1970_23_59_59, MAR_01_1970_00_00_00.set(                  M_31_AS_S * 2 + M_30_AS_S     + M_28_AS_S - 1));
    assert_eq!(MAY_01_1970_00_00_00, APR_30_1970_23_59_59.set(                  M_31_AS_S * 2 + M_30_AS_S     + M_28_AS_S    ));
    assert_eq!(JUL_31_1970_23_59_59, MAY_01_1970_00_00_00.set(                  M_31_AS_S * 4 + M_30_AS_S * 2 + M_28_AS_S - 1));
    assert_eq!(SEP_01_1970_00_00_00, JUL_31_1970_23_59_59.set(                  M_31_AS_S * 5 + M_30_AS_S * 2 + M_28_AS_S    ));
    assert_eq!(DEC_31_1970_23_59_59, SEP_01_1970_00_00_00.set(Y_365_AS_S                                                  - 1));

    // 1972
    assert_eq!(JAN_01_1972_00_00_00, DEC_31_1970_23_59_59.set(Y_365_AS_S *  2                                                ));
    assert_eq!(FEB_29_1972_23_59_59, JAN_01_1972_00_00_00.set(Y_365_AS_S *  2                 + M_31_AS_S     + M_29_AS_S - 1));
    assert_eq!(MAR_01_1972_00_00_00, FEB_29_1972_23_59_59.set(Y_365_AS_S *  2                 + M_31_AS_S     + M_29_AS_S    ));
    assert_eq!(DEC_31_1972_23_59_59, MAR_01_1972_00_00_00.set(Y_365_AS_S *  2 + Y_366_AS_S                                - 1));

    // 2000
    assert_eq!(JAN_01_2000_00_00_00, DEC_31_1972_23_59_59.set(Y_365_AS_S * 23 + Y_366_AS_S *  7                              ));
    assert_eq!(DEC_31_2000_23_59_59, JAN_01_2000_00_00_00.set(Y_365_AS_S * 23 + Y_366_AS_S *  8                           - 1));

    // 2024
    assert_eq!(DEC_31_2024_23_59_59, DEC_31_2000_23_59_59.set(Y_365_AS_S * 41 + Y_366_AS_S * 14                           - 1));
  }

  #[test]
  fn datetime_for_header() {

    // 1970
    assert_eq!(String::from("Thu, 01 Jan 1970 00:00:00 GMT"), JAN_01_1970_00_00_00.for_header());
    assert_eq!(String::from("Sat, 28 Feb 1970 23:59:59 GMT"), FEB_28_1970_23_59_59.for_header());
    assert_eq!(String::from("Sun, 01 Mar 1970 00:00:00 GMT"), MAR_01_1970_00_00_00.for_header());
    assert_eq!(String::from("Thu, 30 Apr 1970 23:59:59 GMT"), APR_30_1970_23_59_59.for_header());
    assert_eq!(String::from("Fri, 01 May 1970 00:00:00 GMT"), MAY_01_1970_00_00_00.for_header());
    assert_eq!(String::from("Fri, 31 Jul 1970 23:59:59 GMT"), JUL_31_1970_23_59_59.for_header());
    assert_eq!(String::from("Tue, 01 Sep 1970 00:00:00 GMT"), SEP_01_1970_00_00_00.for_header());
    assert_eq!(String::from("Thu, 31 Dec 1970 23:59:59 GMT"), DEC_31_1970_23_59_59.for_header());

    // 1972
    assert_eq!(String::from("Sat, 01 Jan 1972 00:00:00 GMT"), JAN_01_1972_00_00_00.for_header());
    assert_eq!(String::from("Tue, 29 Feb 1972 23:59:59 GMT"), FEB_29_1972_23_59_59.for_header());
    assert_eq!(String::from("Wed, 01 Mar 1972 00:00:00 GMT"), MAR_01_1972_00_00_00.for_header());
    assert_eq!(String::from("Sun, 31 Dec 1972 23:59:59 GMT"), DEC_31_1972_23_59_59.for_header());

    // 2000
    assert_eq!(String::from("Sat, 01 Jan 2000 00:00:00 GMT"), JAN_01_2000_00_00_00.for_header());
    assert_eq!(String::from("Sun, 31 Dec 2000 23:59:59 GMT"), DEC_31_2000_23_59_59.for_header());

    // 2024
    assert_eq!(String::from("Tue, 31 Dec 2024 23:59:59 GMT"), DEC_31_2024_23_59_59.for_header());
  }
}
