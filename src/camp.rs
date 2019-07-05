// trait CampUpgrade {
//   fn craft() -> Self;

//   fn destroy(&mut self);
// }

#[derive(Debug)]
pub struct Fire {
  pub status: FireStatus,
}

#[derive(Debug, PartialEq)]
pub enum FireStatus {
  Hot,
  Regular,
  Low,
  Out,
}

impl Fire {
  pub fn new() -> Self {
    Fire {
      status: FireStatus::Out,
    }
  }
  pub fn craft(&mut self) {
    self.status = FireStatus::Hot;
  }
  pub fn destroy(&mut self) {
    self.status = FireStatus::Out;
  }
}

pub struct WaterCollector {
  status: i32,
}
