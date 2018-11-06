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

  let mut num1 = Bij::from(1_u64);
  
  //Regular add = 1.2 seconds
//  for _ in 0..50000000 {   //16 secs
//    let num2 = Bij::from(rng.gen_range(1, 10));
    let num2 = Bij::from(3_u64);
    num1 = num1 + &num2;
//  }
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
      let num = match bij.mem[i] {
        true => 2,
        false => 1,
      };
      out += num * multiplier;
      multiplier *= 2;
    }
    out
  }
}

use std::ops::Add;

//a + b = c
//Consume the bigger one.
impl Add<Bij> for Bij {
  type Output = Bij;
  fn add(self, other: Bij) -> Bij {
    let bigger;
    let smaller;
    if self.mem.len() >= other.mem.len() {
      bigger = self;
      smaller = other;
    } else {
      bigger = other;
      smaller = self;
    }
    bigger + &smaller  //Calls bigger + &smaller
  }
}

//&a + b = c
//Consume b, replace with c.
impl<'a> Add<Bij> for &'a Bij {
  type Output = Bij;
  fn add(self, other: Bij) -> Bij {
    other + self   //Calls other + &self
  }
}

//a + &b = c
//Consume a, replace with c.
//This is the most efficient add. It is consuming the only consumable and replacing it.
//All other adds use this one.
impl<'a> Add<&'a Bij> for Bij {
  type Output = Bij;
  fn add(self, other: &'a Bij) -> Bij {
    
    let bigger;
    let smaller;
    if self.mem.len() >= other.mem.len() {
      bigger = &self;
      smaller = other;
    } else {
      bigger = other;
      smaller = &self;
    }
    
    let mut out = self;
    println!("{:?} {:?}", out.mem, other.mem);
    let mut carry = 0u8;
    for i in 0..smaller.mem.len() {
      match (bigger.mem[i], smaller.mem[i], carry) {
        (false, false, 0) => out.mem[i] = true, //, carry = 0;     1+1+0= 2
        (false, false, 1) => {}, //out[i] = false, carry = 1   1+1+1=11
        (false, false, 2) => {out.mem[i] = true; carry = 1;},    //1+1+2=12
        (true, false, 0) => {out.mem[i] = false; carry = 1;},    //2+1+0=11
        (true, false, 1) => {}, //out[i] = true, carry = 1     2+1+1=12
        (true, false, 2) => out.mem[i] = false, //carry = 2        2+1+2=21
        (false, true, 0) => carry = 1,    //out[i] = false     1+2+0=11
        (false, true, 1) => out.mem[i] = true, //carry = 1         1+2+1=12
        (false, true, 2) => {}, //num[i] = false, carry = 2     1+2+2=21
        (true, true, 0) => carry = 1, // out[i] = true         2+2+0=12
        (true, true, 1) => {out.mem[i] = false; carry = 2;},     //2+2+1=21
        (true, true, 2) => {}, //out[i] = true; carry = 2      2+2+2=22
        _ => panic!("Unknown add combination {} {} {}", out.mem[i], other.mem[i], carry),
      }
    }
  
    for i in smaller.mem.len()..bigger.mem.len() {
      if carry == 0 {
        break;
      }
      match (bigger.mem[i], carry) {
        (false, 1) => {out.mem[i] = true; carry = 0;},  //1+1=2
        (false, 2) => {out.mem[i] = false; carry = 1;}, //1+2=11
        (true, 1) => out.mem[i] = false, //carry = 1      2+1=11
        (true, 2) => {out.mem[i] = true; carry = 1;},   //2+2=12
        _ => panic!("Unknown add combination {} {}", bigger.mem[i], carry),
      }
    }
    //Ran out of numbers, and there is still a carry
    if carry == 1 {
      out.mem.push(false);
    }
  
    out
  }
}

//&a + &b = c
//Clone the biggest number.
impl<'a> Add<&'a Bij> for &'a Bij {
  type Output = Bij;
  fn add(self, other: &'a Bij) -> Bij {
    let bigger;
    let smaller;
    if self.mem.len() >= other.mem.len() {
      bigger = self;
      smaller = other;
    } else {
      bigger = other;
      smaller = self;
    }

    (*bigger).clone() + smaller  //Calls bigger + &smaller
  }
}
