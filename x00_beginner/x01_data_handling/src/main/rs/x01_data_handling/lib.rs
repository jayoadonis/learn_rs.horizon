#![allow(dead_code, unused_variables, unused_imports, unused_assignments)]

pub fn scalar_data_type() {
  let boolean: bool = false | true; //REM: 1 byte, 8 bits

  let signed_byte: i8 = -128 | 127; //REM: 1 byte
  let unsigned_byte: u8 = 0 | 255; //REM: 1 byte

  let signed_short: i16 = -32_768 | 32_767; //REM: 2 bytes, 0x8000  | 0x7FFF
  let unsigned_short: u16 = 0 | 65_535; //REM: 2 bytes, 0 | 0xFFFF

  let character: char = '\0'; //REM: 4 bytes, 32 bits

  let signed_int: i32 = -2_147_483_648 | 2_147_483_647; //REM: 4 bytes, 0x8000_0000  | 0x7FFF_FFFF
  let unsigned_int: u32 = 0 | 4_294_967_295; //REM: 4 bytes, 0x0 | 0xFFFF_FFFF

  let signed_long: i64 = -9_223_372_036_854_775_808 | 9_223_372_036_854_775_807; //REM: 8 bytes
  let unsigned_long: u64 = 0 | 18_446_744_073_709_551_615; //REM: 8 bytes

  let float: f32 = 3.4028235e38 as f32; //REM: (-,+) 3.4028235e38 
  let float: f64 = 1.7976931348623157e308 as f64; //REM: (-,+) 1.7976931348623157e308 

  // let float: f32 =
  //   ((-3.4028235e38 as f32).to_bits() | (3.4028235e38 as f32).to_bits()) as f32; //REM: Max 7 precission
  // let double: f64 = ((-1.7976931348623157e308 as f64).to_bits()
  //   | (1.7976931348623157e308 as f64).to_bits()) as f64; //REM: Max 16 precission

  let signed_128: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728
    | 170_141_183_460_469_231_731_687_303_715_884_105_727; //REM: 16 bytes, 170x10^36
  let unsigned_123: u128 =
    0 | 340_282_366_920_938_463_463_374_607_431_768_211_455; //REM: 16 bytes, 340x10^36

  //REM: Architecture dependent, usually 32, 64 or other bits Architecture
  let signed_size: isize = std::isize::MIN | std::isize::MAX;
  let unsigned_size: usize = std::usize::MIN | std::usize::MAX;
}

pub fn compound_data_type() {
  let tuple: (i32, &str, String, f32) = (1, "2", "3".to_string(), (4.0 as f32));
  let int: i32 = tuple.0; //REM: trivial copy (rhs ---> lhs)
  let str: &str = tuple.1; //REM: immutable barrow (rhs ---> lhs)
  let string: String = tuple.2; //REM: owned | move (rhs ---> lhs)
  let float: f32 = tuple.3; //REM: trivial copy (rhs ---> lhs)

  let array: [i32; 4] = [1, 2, 3, 4];
  let array_i: [f32; 4] = [0.0; 4]; //REM: [0.0, 0.0, 0.0, 0.0]
  let f32_slice: &[f32] = &array_i[0..3]; //REM: [0.0, 0.0, 0.0], indices (0 ---> 2), 3 not-included
  let f32_slice_i: &[f32] = &array_i[0..=3]; //REM: [0.0, 0.0, 0.0, 0.0], indices (0 ---> 3)

  // let float_error: f32 = array_i[4]; //REM: Caution: capture at runtime; error index out of bounds
  let float_ii: f32 = f32_slice[1];
  let float_i: f32 = array_i[3];
  let int_i: i32 = array[0];
}

pub fn immutable() {
  let x: i8 = -128 | 127;
  // x = 0; //REM: ERROR, cannot mutate immutable variable 'x'.

  let y: &str = "Hi";
  // y = "New There"; //REM: ERROR, cannot mutate immutable variable 'y'.

  let z: String = "Hi".to_string();
  // z.push( ' ' ); //REM: ERROR, cannot mutate immutable variable 'z'.
  // z.push_str( "There" ); //REM: ERROR, cannot mutate immutable variable 'z'.

  let k: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
  // k[0] = 1.5; //REM: ERROR, cannot mutate

  let v = &k[..=3]; //REM: slice, indices (0 ---> 3)
  // v[0] = 1.0; //REM: ERROR, cannot mutate 'address-of' (rhs).
  // v = &k[3..1] //REM: ERROR, cannot mutate 'identifier' (lhs).
}

pub fn mutable() {
  let mut x: i8 = -128 | 127;
  x = 0;

  let mut y: &str = "Hi";
  y = "New There";

  let mut z: String = "Hi".to_string();
  z.push(' ');
  z.push_str("There");

  let mut k: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
  k[3] = 4.5;
  k = [5.0, 6.0, 7.0, 8.0];

  let mut v: &mut [f32] = &mut k[2..=3]; //REM: slice, indices (2 --> 3)
  v[0] = 2.5;
  v[1] = 3.5;
  v = &mut k[..]; //REM: all slices
  v = &mut k[..1]; //REM: only index 0 sliced

  let i: &mut [f32] = &mut k[..];
  i[3] += 0.1;
  // i = &mut k[..=2]; //REM: ERROR, only the 'address-of' can be mutated, not the 'identifier'

  let mut t: &[f32] = &i[..];
  // t[3] -= 0.1; //REM: ERROR, only the 'identifier can be reassigned, can not mutate the 'address-of'
  t = &k[..];
}