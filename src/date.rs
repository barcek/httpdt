use crate::time::{H_AS_S, D_AS_H};

use std::fmt::{self, Display, Formatter};

pub const D_AS_S: u64 = H_AS_S * D_AS_H;

#[derive(PartialEq, Debug)]
pub struct Date {
  pub  d: u8,
  pub wd: Weekday,
  pub  m: Month,
  pub  y: Year,
  pub xs: u64
}

impl Default for Date {

  fn default() -> Self {
    Self {
       d: 1,
      wd: Weekday::default(),
       m: Month::default(),
       y: Year::default(),
      xs: 0
    }
  }
}

impl Date {

  pub fn skip(&self, diff_s: u64) -> Self {

    let Date { mut d, mut wd, mut m, mut y, xs: today_s } = self;
    let mut xs = diff_s + today_s;

    if xs >= D_AS_S {
      'months: loop {
        let this_m_as_d = m.len(y.is_leap());
          'days: loop {
            if      xs  < D_AS_S { break 'months }
                    xs -= D_AS_S;
                    wd  = wd.skip(1);
            if  d != this_m_as_d {
                     d += 1
            } else { d  = 1;       break 'days   }
          }
        if m.is_last() { y = y.skip(1) };
                         m = m.skip(1);
      }
    };
    Self { d, wd, m, y, xs }
  }

  pub fn for_header(&self) -> String {
    ImfFixdateDate(self).to_string()
  }
}

// ImfFixdateDate

pub struct ImfFixdateDate<'a>(&'a Date);

impl Display for ImfFixdateDate<'_> {

  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let ImfFixdateDate(d) = self;
    write!(f, "{:?}, {:02} {:?} {}", d.wd, d.d, d.m, d.y)
  }
}

// Weekday

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Weekday {
  Mon,
  Tue,
  Wed,
  Thu,
  Fri,
  Sat,
  Sun
}

impl Default for Weekday {

  fn default() -> Self {
    Self::Thu
  }
}

impl Weekday {

  pub fn skip(&self, diff_d: u64) -> Self {
    let mut current = self;
    for _ in 0..diff_d {
      current = match current {
        Self::Mon => &Self::Tue,
        Self::Tue => &Self::Wed,
        Self::Wed => &Self::Thu,
        Self::Thu => &Self::Fri,
        Self::Fri => &Self::Sat,
        Self::Sat => &Self::Sun,
        Self::Sun => &Self::Mon
      };
    }
    *current
  }
}

// Month

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Month {
  Jan,
  Feb,
  Mar,
  Apr,
  May,
  Jun,
  Jul,
  Aug,
  Sep,
  Oct,
  Nov,
  Dec
}

impl Default for Month {

  fn default() -> Self {
    Self::Jan
  }
}

impl Month {

  pub fn len(&self, is_leap_year: bool) -> u8 {
    match self {
      Self::Jan | Self::Mar | Self::May | Self::Jul |
                  Self::Aug | Self::Oct | Self::Dec => 31,
      Self::Apr | Self::Jun | Self::Sep | Self::Nov => 30,
      Self::Feb if !is_leap_year                    => 28,
      Self::Feb                                     => 29
    }
  }

  pub fn skip(&self, diff_m: u64) -> Self {
    let mut current = self;
    for _ in 0..diff_m {
      current = match current {
        Self::Jan => &Self::Feb,
        Self::Feb => &Self::Mar,
        Self::Mar => &Self::Apr,
        Self::Apr => &Self::May,
        Self::May => &Self::Jun,
        Self::Jun => &Self::Jul,
        Self::Jul => &Self::Aug,
        Self::Aug => &Self::Sep,
        Self::Sep => &Self::Oct,
        Self::Oct => &Self::Nov,
        Self::Nov => &Self::Dec,
        Self::Dec => &Self::Jan
      };
    }
    *current
  }

  pub fn is_last(&self) -> bool {
    *self == Month::Dec
  }
}

// Year

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Year(pub u64);

impl Default for Year {

  fn default() -> Self {
    Self(1970)
  }
}

impl Year {

  pub fn skip(&self, diff_y: u64) -> Self {
    let Year(y) = self;
    Self(y + diff_y)
  }

  pub fn is_leap(&self) -> bool {
    let Year(y) = self;
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
  }
}

impl Display for Year {

  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let Year(y) = self;
    write!(f, "{y}")
  }
}

#[cfg(test)]
pub mod test {

  use super::{Date, Weekday, Month, Year, D_AS_S};

  pub const M_28_AS_S: u64  = D_AS_S *  28;
  pub const M_29_AS_S: u64  = D_AS_S *  29;
  pub const M_30_AS_S: u64  = D_AS_S *  30;
  pub const M_31_AS_S: u64  = D_AS_S *  31;

  pub const Y_365_AS_S: u64 = D_AS_S * 365;
  pub const Y_366_AS_S: u64 = D_AS_S * 366;

  // 1970
  pub const JAN_01_1970_00_00_00: Date = Date { wd: Weekday::Thu, d:  1, m: Month::Jan, y: Year(1970), xs:          0 };
  pub const FEB_28_1970_23_59_59: Date = Date { wd: Weekday::Sat, d: 28, m: Month::Feb, y: Year(1970), xs: D_AS_S - 1 };
  pub const MAR_01_1970_00_00_00: Date = Date { wd: Weekday::Sun, d:  1, m: Month::Mar, y: Year(1970), xs:          0 };
  pub const APR_30_1970_23_59_59: Date = Date { wd: Weekday::Thu, d: 30, m: Month::Apr, y: Year(1970), xs: D_AS_S - 1 };
  pub const MAY_01_1970_00_00_00: Date = Date { wd: Weekday::Fri, d:  1, m: Month::May, y: Year(1970), xs:          0 };
  pub const JUL_31_1970_23_59_59: Date = Date { wd: Weekday::Fri, d: 31, m: Month::Jul, y: Year(1970), xs: D_AS_S - 1 };
  pub const SEP_01_1970_00_00_00: Date = Date { wd: Weekday::Tue, d:  1, m: Month::Sep, y: Year(1970), xs:          0 };
  pub const DEC_31_1970_23_59_59: Date = Date { wd: Weekday::Thu, d: 31, m: Month::Dec, y: Year(1970), xs: D_AS_S - 1 };

  // 1972
  pub const JAN_01_1972_00_00_00: Date = Date { wd: Weekday::Sat, d:  1, m: Month::Jan, y: Year(1972), xs:          0 };
  pub const FEB_29_1972_23_59_59: Date = Date { wd: Weekday::Tue, d: 29, m: Month::Feb, y: Year(1972), xs: D_AS_S - 1 };
  pub const MAR_01_1972_00_00_00: Date = Date { wd: Weekday::Wed, d:  1, m: Month::Mar, y: Year(1972), xs:          0 };
  pub const DEC_31_1972_23_59_59: Date = Date { wd: Weekday::Sun, d: 31, m: Month::Dec, y: Year(1972), xs: D_AS_S - 1 };

  // 2000
  pub const JAN_01_2000_00_00_00: Date = Date { wd: Weekday::Sat, d:  1, m: Month::Jan, y: Year(2000), xs:          0 };
  pub const DEC_31_2000_23_59_59: Date = Date { wd: Weekday::Sun, d: 31, m: Month::Dec, y: Year(2000), xs: D_AS_S - 1 };

  // 2024
  pub const DEC_31_2024_23_59_59: Date = Date { wd: Weekday::Tue, d: 31, m: Month::Dec, y: Year(2024), xs: D_AS_S - 1 };

  #[test]
  fn date_default() {

    assert_eq!(JAN_01_1970_00_00_00, Date::default());
  }

  #[test]
  fn date_skip() {

    // individual, each from the Unix epoch

    // 1970
    assert_eq!(JAN_01_1970_00_00_00, JAN_01_1970_00_00_00.skip(                                                              0));
    assert_eq!(FEB_28_1970_23_59_59, JAN_01_1970_00_00_00.skip(                  M_31_AS_S                     + M_28_AS_S - 1));
    assert_eq!(MAR_01_1970_00_00_00, JAN_01_1970_00_00_00.skip(                  M_31_AS_S                     + M_28_AS_S    ));
    assert_eq!(APR_30_1970_23_59_59, JAN_01_1970_00_00_00.skip(                  M_31_AS_S * 2 + M_30_AS_S     + M_28_AS_S - 1));
    assert_eq!(MAY_01_1970_00_00_00, JAN_01_1970_00_00_00.skip(                  M_31_AS_S * 2 + M_30_AS_S     + M_28_AS_S    ));
    assert_eq!(JUL_31_1970_23_59_59, JAN_01_1970_00_00_00.skip(                  M_31_AS_S * 4 + M_30_AS_S * 2 + M_28_AS_S - 1));
    assert_eq!(SEP_01_1970_00_00_00, JAN_01_1970_00_00_00.skip(                  M_31_AS_S * 5 + M_30_AS_S * 2 + M_28_AS_S    ));
    assert_eq!(DEC_31_1970_23_59_59, JAN_01_1970_00_00_00.skip(Y_365_AS_S                                                  - 1));

    // 1972
    assert_eq!(JAN_01_1972_00_00_00, JAN_01_1970_00_00_00.skip(Y_365_AS_S *  2                                                ));
    assert_eq!(FEB_29_1972_23_59_59, JAN_01_1970_00_00_00.skip(Y_365_AS_S *  2                 + M_31_AS_S     + M_29_AS_S - 1));
    assert_eq!(MAR_01_1972_00_00_00, JAN_01_1970_00_00_00.skip(Y_365_AS_S *  2                 + M_31_AS_S     + M_29_AS_S    ));
    assert_eq!(DEC_31_1972_23_59_59, JAN_01_1970_00_00_00.skip(Y_365_AS_S *  2 + Y_366_AS_S                                - 1));

    // 2000
    assert_eq!(JAN_01_2000_00_00_00, JAN_01_1970_00_00_00.skip(Y_365_AS_S * 23 + Y_366_AS_S *  7                              ));
    assert_eq!(DEC_31_2000_23_59_59, JAN_01_1970_00_00_00.skip(Y_365_AS_S * 23 + Y_366_AS_S *  8                           - 1));

    // 2024
    assert_eq!(DEC_31_2024_23_59_59, JAN_01_1970_00_00_00.skip(Y_365_AS_S * 41 + Y_366_AS_S * 14                           - 1));

    // sequential, each from the preceding value

    // 1970
    assert_eq!(MAR_01_1970_00_00_00, FEB_28_1970_23_59_59.skip(                                                              1));
    assert_eq!(APR_30_1970_23_59_59, MAR_01_1970_00_00_00.skip(                  M_31_AS_S     + M_30_AS_S                 - 1));
    assert_eq!(MAY_01_1970_00_00_00, APR_30_1970_23_59_59.skip(                                                              1));
    assert_eq!(JUL_31_1970_23_59_59, MAY_01_1970_00_00_00.skip(                  M_31_AS_S * 2 + M_30_AS_S                 - 1));
    assert_eq!(SEP_01_1970_00_00_00, JUL_31_1970_23_59_59.skip(                  M_31_AS_S                                 + 1));
    assert_eq!(DEC_31_1970_23_59_59, SEP_01_1970_00_00_00.skip(                  M_31_AS_S * 2 + M_30_AS_S * 2             - 1));

    // 1972
    assert_eq!(JAN_01_1972_00_00_00, DEC_31_1970_23_59_59.skip(Y_365_AS_S                                                  + 1));
    assert_eq!(FEB_29_1972_23_59_59, JAN_01_1972_00_00_00.skip(                  M_31_AS_S                     + M_29_AS_S - 1));
    assert_eq!(MAR_01_1972_00_00_00, FEB_29_1972_23_59_59.skip(                                                              1));
    assert_eq!(DEC_31_1972_23_59_59, MAR_01_1972_00_00_00.skip(                  M_31_AS_S * 6 + M_30_AS_S * 4             - 1));

    // 2000
    assert_eq!(JAN_01_2000_00_00_00, DEC_31_1972_23_59_59.skip(Y_365_AS_S * 21 + Y_366_AS_S *  6                           + 1));
    assert_eq!(DEC_31_2000_23_59_59, JAN_01_2000_00_00_00.skip(                  M_31_AS_S * 7 + M_30_AS_S * 4 + M_29_AS_S - 1));

    // 2024
    assert_eq!(DEC_31_2024_23_59_59, DEC_31_2000_23_59_59.skip(Y_365_AS_S * 18 + Y_366_AS_S *  6                              ));
  }

  #[test]
  fn date_for_header() {

    // 1970
    assert_eq!(String::from("Thu, 01 Jan 1970"), JAN_01_1970_00_00_00.for_header());
    assert_eq!(String::from("Sat, 28 Feb 1970"), FEB_28_1970_23_59_59.for_header());
    assert_eq!(String::from("Sun, 01 Mar 1970"), MAR_01_1970_00_00_00.for_header());
    assert_eq!(String::from("Thu, 30 Apr 1970"), APR_30_1970_23_59_59.for_header());
    assert_eq!(String::from("Fri, 01 May 1970"), MAY_01_1970_00_00_00.for_header());
    assert_eq!(String::from("Fri, 31 Jul 1970"), JUL_31_1970_23_59_59.for_header());
    assert_eq!(String::from("Tue, 01 Sep 1970"), SEP_01_1970_00_00_00.for_header());
    assert_eq!(String::from("Thu, 31 Dec 1970"), DEC_31_1970_23_59_59.for_header());

    // 1972
    assert_eq!(String::from("Sat, 01 Jan 1972"), JAN_01_1972_00_00_00.for_header());
    assert_eq!(String::from("Tue, 29 Feb 1972"), FEB_29_1972_23_59_59.for_header());
    assert_eq!(String::from("Wed, 01 Mar 1972"), MAR_01_1972_00_00_00.for_header());
    assert_eq!(String::from("Sun, 31 Dec 1972"), DEC_31_1972_23_59_59.for_header());

    // 2000
    assert_eq!(String::from("Sat, 01 Jan 2000"), JAN_01_2000_00_00_00.for_header());
    assert_eq!(String::from("Sun, 31 Dec 2000"), DEC_31_2000_23_59_59.for_header());

    // 2024
    assert_eq!(String::from("Tue, 31 Dec 2024"), DEC_31_2024_23_59_59.for_header());
  }
}
