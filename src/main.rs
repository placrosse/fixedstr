#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
//use std::mem;
mod lib;
use lib::*;

fn main()
{
   let s1 = str16::from("abc");
   let mut s2 = str32::from("and xyz");
   s2.push(" and 1234");
   println!("{} {}, {}", s1, &s2, s2.len());
   println!("{}", &s1=="abc");
   let s3 = s1;     // copied, not moved
   println!("{}", "abc"==&s1);
   println!("{}, {} ", s1==s3, s1==s2.resize());

   let mut s4:fstr<256> = s3.resize();
   s4.push("ccccccccccccccccccccccccccccccccccccccccccccccccccccccz");
   println!("{}, length {}",&s4, s4.len());
   let mut s5:fstr<32> = s4.resize();
   println!("{}, length {}",&s5, s5.len());
   println!("{:?}, length {}",&s5[0..10], s5.len());      
   println!("{}", s2.substr(2,6));
   println!("{}", s2.substr(2,6).len());   
let mut s4:fstr<64> = s1.resize();
   let owned_string:String = s4.to_string();
   let str_slice:&str = s4.to_str();
   println!("as &str: {}",&str_slice[0..2]);
   s4 = s1.resize();
   let s5 = str8::new();
   let ss5 = s5.as_str();
   othertests();
}//main

fn othertests()
{
  let s1:fstr<8> = fstr::from("abcdefg");
  let s2:fstr<16> = s1.resize();
  let s3:fstr<8> = fstr::from("abcdxr");
  println!("cmp test: {}", s3>s1);

//  let s = [65u8, 66,67];
//  let st = &s[..] as &str;

  let chrs = ['a','b','c','\0'];
  // try to coerce into str
  let rawp = (&chrs[..]) as *const [char];
  let raw2 = rawp as *const &str;
  println!("what is raw2? {:?}",raw2); // mem addr
  
  //let string1:&str = mem::transmute::<&[char], &str>(&chrs[0..3]);
  //println!("got str: {:?}",string1);
}//othertests
