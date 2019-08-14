use crate::colored::Colorize;

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
  pub status: CollectorStatus,
  pub time: i32,
  pub uses: i32,
}

#[derive(Debug, PartialEq)]
pub enum CollectorStatus {
  Collecting,
  Waiting,
  Out,
}

impl WaterCollector {
  pub fn new() -> Self {
    WaterCollector {
      status: CollectorStatus::Out,
      time: 0,
      uses: 0,
    }
  }
  pub fn craft(&mut self) {
    self.status = CollectorStatus::Collecting;
    self.time = 0;
    self.uses = 0;
    println!("{}", "Collecting…".italic().dimmed());
  }
  pub fn destroy(&mut self) {
    self.status = CollectorStatus::Out;
    self.time = 0;
    self.uses = 0;
    println!("{}", "The water collector broke down!".red());
  }
  pub fn pass_time(&mut self) {
    if self.status == CollectorStatus::Collecting {
      self.time += 10;

      if self.time == 60 {
        self.status = CollectorStatus::Waiting;
        println!("{}", "The water collector is full!".green());
      }
    }
  }
  pub fn collect(&mut self) {
    self.time = 0;
    self.uses += 1;

    if self.uses == 3 {
      self.destroy();
    } else {
      self.status = CollectorStatus::Collecting;
      println!("{}", "Collecting…".italic().dimmed());
    }
  }
}
