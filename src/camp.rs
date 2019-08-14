#[derive(Debug)]
pub struct Fire {
  pub status: FireStatus,
  pub time: i32,
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
      time: 0,
    }
  }
  pub fn craft(&mut self) {
    self.status = FireStatus::Hot;
    self.time = 0;
  }
  pub fn destroy(&mut self) {
    self.status = FireStatus::Out;
    self.time = 0;
  }
  pub fn pass_time(&mut self) {
    if self.status != FireStatus::Out {
      self.time += 10;

      if self.time % 60 == 0 {
        self.lower_status();
      }
    }
  }
  pub fn lower_status(&mut self) {
    match self.status {
      FireStatus::Hot => {
        println!("Fire is burning");
        self.status = FireStatus::Regular;
      }
      FireStatus::Regular => {
        println!("Fire is burning low");
        self.status = FireStatus::Low;
      }
      _ => {
        println!("Fire has burnt out");
        self.destroy();
      }
    }
  }
  pub fn increase_status(&mut self) {
    match self.status {
      FireStatus::Low => {
        println!("Fire is burning");
        self.time = 60;
        self.status = FireStatus::Regular;
      }
      FireStatus::Regular | FireStatus::Hot => {
        println!("Fire is burning hot");
        self.time = 0;
        self.status = FireStatus::Hot;
      }
      _ => {}
    }
  }
}

pub struct WaterCollector {
  status: i32,
}
