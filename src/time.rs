use std::convert::From;
use std::fmt::{self, Display, Formatter};

pub const M_AS_S: u64 =              60;
pub const H_AS_M: u64 =              60;
pub const H_AS_S: u64 = M_AS_S * H_AS_M;
pub const D_AS_H: u64 =              24;

#[derive(Default, PartialEq, Debug)]
pub struct Time {
  pub  h: u8,
  pub  m: u8,
  pub  s: u8,
  pub xs: u64
}

impl From<u64> for Time {

  fn from(init_s: u64) -> Self {
    let  s = (init_s                  )             % M_AS_S;
    let  m = (init_s -  s             ) /    M_AS_S % H_AS_M;
    let  h = (init_s -  s - m * M_AS_S) /    H_AS_S % D_AS_H;
    let xs =  init_s - (s + m * M_AS_S + h * H_AS_S);
    Self {
       h: h as u8,
       m: m as u8,
       s: s as u8,
      xs
    }
  }
}

impl Time {

  pub fn for_header(&self) -> String {
    ImfFixdateTime(self).to_string()
  }
}

// ImfFixdateTime

pub struct ImfFixdateTime<'a>(&'a Time);

impl Display for ImfFixdateTime<'_> {

  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let ImfFixdateTime(t) = self;
    write!(f, "{:02}:{:02}:{:02}", t.h, t.m, t.s)
  }
}

#[cfg(test)]
pub mod test {

  use super::{Time, M_AS_S, H_AS_S, H_AS_M, D_AS_H};
  use crate::date::D_AS_S;

  pub const JAN_01_1970_00_00_00: Time = Time { h:                  0, m:                  0, s:                  0, xs:      0 };
      const JAN_01_1970_00_00_59: Time = Time { h:                  0, m:                  0, s: (M_AS_S - 1) as u8, xs:      0 };
      const JAN_01_1970_00_01_00: Time = Time { h:                  0, m:                  1, s:                  0, xs:      0 };
      const JAN_01_1970_00_59_59: Time = Time { h:                  0, m: (H_AS_M - 1) as u8, s: (M_AS_S - 1) as u8, xs:      0 };
      const JAN_01_1970_01_00_00: Time = Time { h:                  1, m:                  0, s:                  0, xs:      0 };
      const JAN_01_1970_23_59_59: Time = Time { h: (D_AS_H - 1) as u8, m: (H_AS_M - 1) as u8, s: (M_AS_S - 1) as u8, xs:      0 };
      const JAN_02_1970_00_00_00: Time = Time { h:                  0, m:                  0, s:                  0, xs: D_AS_S };

  #[test]
  fn time_default() {

    assert_eq!(JAN_01_1970_00_00_00, Time::default());
  }

  #[test]
  fn time_from() {

    assert_eq!(JAN_01_1970_00_00_00, Time::from(         0));
    assert_eq!(JAN_01_1970_00_00_59, Time::from(M_AS_S - 1));
    assert_eq!(JAN_01_1970_00_01_00, Time::from(M_AS_S    ));
    assert_eq!(JAN_01_1970_00_59_59, Time::from(H_AS_S - 1));
    assert_eq!(JAN_01_1970_01_00_00, Time::from(H_AS_S    ));
    assert_eq!(JAN_01_1970_23_59_59, Time::from(D_AS_S - 1));
    assert_eq!(JAN_02_1970_00_00_00, Time::from(D_AS_S    ));
  }

  #[test]
  fn time_for_header() {

    assert_eq!(String::from("00:00:00"), JAN_01_1970_00_00_00.for_header());
    assert_eq!(String::from("00:00:59"), JAN_01_1970_00_00_59.for_header());
    assert_eq!(String::from("00:01:00"), JAN_01_1970_00_01_00.for_header());
    assert_eq!(String::from("00:59:59"), JAN_01_1970_00_59_59.for_header());
    assert_eq!(String::from("01:00:00"), JAN_01_1970_01_00_00.for_header());
    assert_eq!(String::from("23:59:59"), JAN_01_1970_23_59_59.for_header());
    assert_eq!(String::from("00:00:00"), JAN_02_1970_00_00_00.for_header());
  }
}
