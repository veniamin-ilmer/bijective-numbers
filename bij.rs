extern crate rand; // 0.5.5
use rand::Rng;

use std::time::Instant;

pub fn main() {
  let start = Instant::now();
  
  let mut rng = rand::thread_rng();
/*
  let mut num1 = Bij::from(1_u64);
  let num2 = Bij::from(1_u64);
  let num3 = &num1 + &num2 + &num2;
  println!("{} + {} = {}", u64::from(num1), u64::from(num2), u64::from(num3));
*/
  let mut num1 = Bij::from(0_u64);
  
  for _ in 0..20000000 {
    let num2 = Bij::from(rng.gen_range(1, 10));
    num1 = &num1 + num2;
  }

  let temp = u64::from(num1);
  println!("{}", temp);

  let elapsed = start.elapsed();
  println!("{:?}", elapsed);

}

struct Bij {
  mem: Vec<bool>,
}

impl Bij {
  fn new() -> Bij {
    Bij { mem: Vec::new() }
  }
}
  
//Assumption: `bigger` memory length must be >= `smaller` memory length.
//All locations that call this macro *must* take this into account before calling.
//Per Experiment 1:
//We should only reuse the memory if the reusable variable is bigger.
//If the only reusable variable is smaller, clone the bigger variable instead.
macro_rules! add {
  //Overwrites $bigger. Read only $smaller
  ($bigger:expr, $smaller:expr) => (
    let mut carry = None;
    for i in 0..$smaller.mem.len() {
      match ($bigger.mem[i], $smaller.mem[i], carry) {
        (false, false, None) => $bigger.mem[i] = true, //, carry = None;                 1+1+0= 2
        (false, false, Some(false)) => {}, //$bigger[i] = false, carry = Some(false)     1+1+1=11
        (false, false, Some(true)) => {$bigger.mem[i] = true; carry = Some(false);},   //1+1+2=12
        (true, false, None) => {$bigger.mem[i] = false; carry = Some(false);},         //2+1+0=11
        (true, false, Some(false)) => {}, //$bigger[i] = true, carry = Some(false)       2+1+1=12
        (true, false, Some(true)) => $bigger.mem[i] = false, //carry = Some(true)        2+1+2=21
        (false, true, None) => carry = Some(false),    //bigger[i] = false               1+2+0=11
        (false, true, Some(false)) => $bigger.mem[i] = true, //carry = Some(false)       1+2+1=12
        (false, true, Some(true)) => {}, //num[i] = false, carry = Some(true)            1+2+2=21
        (true, true, None) => carry = Some(false), // bigger[i] = true                   2+2+0=12
        (true, true, Some(false)) => {$bigger.mem[i] = false; carry = Some(true);},    //2+2+1=21
        (true, true, Some(true)) => {}, //bigger[i] = true; carry = Some(true)           2+2+2=22
      }
    }
    
    if carry.is_some() && $smaller.mem.len() < $bigger.mem.len() {  //$smaller.mem.len() < $bigger.mem.len() must be there, because we are assuming this below.
      let mut i_start = $smaller.mem.len();
      if carry == Some(true) {  //bigger.mem[i] == false && carry=Some(true) => bigger.mem[i] = false; carry = Some(false);
        carry = Some(false);    //bigger.mem[i] == false && carry=Some(true) => bigger.mem[i] = true; carry = Some(false);
        i_start += 1;
      }
      for i in i_start..$bigger.mem.len() {   //Carry is assumed to be 1 this whole time, unless break.
        match $bigger.mem[i] {
          false => {$bigger.mem[i] = true; carry = None; break;},     //1+1=2    No carry, so break
          true => $bigger.mem[i] = false,                              //2+1=11
        }
      }
    }
    
    //Ran out of numbers. At the end, still have a carry
    if carry.is_some() {
      $bigger.mem.push(carry.unwrap());
    }
  );
}


impl Clone for Bij {
    fn clone(&self) -> Bij {
      Bij { mem: self.mem.clone() }
    }
}

impl From<u64> for Bij {
  fn from(small: u64) -> Bij {
    let mut int = small;
    let mut bij = Bij::new();
    while int >= 1 {
      match int % 2 {
        1 => {bij.mem.push(false); int = (int - 1) / 2; },
        0 => {bij.mem.push(true); int = (int - 2) / 2; },
        _ => panic!("Unknown mod result: {}", int % 2),
      }
    }
    bij
  }
}

impl From<Bij> for u64 {
  fn from(bij: Bij) -> u64 {
    let mut out = 0;
    let mut multiplier = 1;
    for i in 0..bij.mem.len() {
    /*
      let num = match bij.mem[i] {
        true => 2,
        false => 1,
      };
      out += num * multiplier;
      multiplier *= 2;
      */
      out += multiplier;
      if bij.mem[i] {
        out += multiplier;
      }
      multiplier *= 2;
    }
    out
  }
}

use std::ops::AddAssign;

// a += &b
impl<'a> AddAssign<&'a Bij> for Bij {
  fn add_assign(&mut self, other: &'a Bij) {
    if self.mem.len() >= other.mem.len() {
      add!(self, other);
    } else {
      let mut cloned_other = (*other).clone();
      add!(cloned_other, self);
      *self = cloned_other;
    }
  }
}

// a += b
impl AddAssign for Bij {
  fn add_assign(&mut self, mut other: Bij) {
    if self.mem.len() >= other.mem.len() {
      add!(self, other);
    } else {
      add!(other, self);
      *self = other;
    }
  }
}

use std::ops::Add;

//c = a + &b
//Consume a, replace with c.
impl<'a> Add<&'a Bij> for Bij {
  type Output = Bij;
  fn add(mut self, other: &'a Bij) -> Bij {
    if self.mem.len() >= other.mem.len() {
      add!(self, other);
      self
    } else {
      let mut other = (*other).clone();
      add!(other, self);
      other
    }
  }
}

//c = a + b
//Consume the bigger one.
impl Add<Bij> for Bij {
  type Output = Bij;
  fn add(mut self, mut other: Bij) -> Bij {
    if self.mem.len() >= other.mem.len() {
      add!(self, other);
      self
    } else {
      add!(other, self);
      other
    }
  }
}

//c = &a + b
impl<'a> Add<Bij> for &'a Bij {
  type Output = Bij;
  fn add(self, mut other: Bij) -> Bij {
    if self.mem.len() >= other.mem.len() {
      let mut cloned_self = (*self).clone();
      add!(cloned_self, other);
      cloned_self
    } else {
      add!(other, self);
      other
    }
  }
}

//c = &a + &b
//Clone the biggest number.
impl<'a> Add<&'a Bij> for &'a Bij {
  type Output = Bij;
  fn add(self, other: &'a Bij) -> Bij {
    if self.mem.len() >= other.mem.len() {
      let mut cloned_self = (*self).clone();
      add!(cloned_self, other);
      cloned_self
    } else {
      let mut cloned_other = (*other).clone();
      add!(cloned_other, self);
      cloned_other
    }
  }
}
