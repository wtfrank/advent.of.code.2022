extern crate test;

use bitvec::prelude::*;

pub trait Min {
  fn minval() -> Self;
}

pub trait Max {
  fn maxval() -> Self;
}

impl Max for usize {
  fn maxval() -> usize {
    usize::MAX
  }
}

impl Max for isize {
  fn maxval() -> isize {
    isize::MAX
  }
}

impl Max for u8 {
  fn maxval() -> u8 {
    u8::MAX
  }
}

impl Max for u16 {
  fn maxval() -> u16 {
    u16::MAX
  }
}

impl Max for u32 {
  fn maxval() -> u32 {
    u32::MAX
  }
}

impl Max for u64 {
  fn maxval() -> u64 {
    u64::MAX
  }
}

impl Max for u128 {
  fn maxval() -> u128 {
    u128::MAX
  }
}

impl Max for i8 {
  fn maxval() -> i8 {
    i8::MAX
  }
}

impl Max for i16 {
  fn maxval() -> i16 {
    i16::MAX
  }
}

impl Max for i32 {
  fn maxval() -> i32 {
    i32::MAX
  }
}

impl Max for i64 {
  fn maxval() -> i64 {
    i64::MAX
  }
}

impl Max for i128 {
  fn maxval() -> i128 {
    i128::MAX
  }
}

impl Min for usize {
  fn minval() -> usize {
    usize::MIN
  }
}

impl Min for isize {
  fn minval() -> isize {
    isize::MIN
  }
}

impl Min for u8 {
  fn minval() -> u8 {
    u8::MIN
  }
}

impl Min for u16 {
  fn minval() -> u16 {
    u16::MIN
  }
}

impl Min for u32 {
  fn minval() -> u32 {
    u32::MIN
  }
}

impl Min for u64 {
  fn minval() -> u64 {
    u64::MIN
  }
}

impl Min for u128 {
  fn minval() -> u128 {
    u128::MIN
  }
}

impl Min for i8 {
  fn minval() -> i8 {
    i8::MIN
  }
}

impl Min for i16 {
  fn minval() -> i16 {
    i16::MIN
  }
}

impl Min for i32 {
  fn minval() -> i32 {
    i32::MIN
  }
}

impl Min for i64 {
  fn minval() -> i64 {
    i64::MIN
  }
}

impl Min for i128 {
  fn minval() -> i128 {
    i128::MIN
  }
}

pub trait IntegerMaths:
  num::PrimInt
  + num::FromPrimitive
  + Max
  + Min
  + std::ops::MulAssign
  + std::cmp::PartialEq
  + std::cmp::PartialOrd
  + std::ops::Rem
  + std::ops::DivAssign
  + std::iter::Step
  + integer_sqrt::IntegerSquareRoot
{
}
impl<
    T: num::PrimInt
      + num::FromPrimitive
      + Max
      + Min
      + std::ops::MulAssign
      + std::cmp::PartialEq
      + std::cmp::PartialOrd
      + std::ops::Rem
      + std::ops::DivAssign
      + std::iter::Step
      + integer_sqrt::IntegerSquareRoot,
  > IntegerMaths for T
{
}

/* returns a vector of prime factors in ascending order
 * If a prime factorises a number of times, there will be
 * repeats
 */
pub fn prime_factors(n: usize) -> Vec<usize> {
  let mut factors = Vec::<usize>::new();

  let mut n = n;
  while n.is_multiple_of(2) {
    factors.push(2);
    n /= 2;
  }

  for i in (3..f64::sqrt(n as f64) as usize + 1).step_by(2) {
    while n.is_multiple_of(i) {
      factors.push(i);
      n /= i;
    }
  }

  if n > 2 {
    factors.push(n);
  }

  factors
}

pub fn prime_factors2<T>(n: T) -> Vec<T>
where
  T: IntegerMaths,
  <T as std::ops::Rem>::Output: PartialEq<T>,
{
  let mut factors = Vec::<T>::new();

  let mut n: T = n;
  let two = T::from_u8(2).unwrap();
  while n % two == T::zero() {
    factors.push(two);
    n /= two;
  }

  // let range_max: T = Into::<T>::into(f64::sqrt(n.into()) + 1_f64);
  let range_max: T = n.integer_sqrt() + T::one();
  let three = T::from_u8(3).unwrap();
  for i in (three..range_max).step_by(2) {
    while n % i == T::zero() {
      factors.push(i);
      n /= i;
    }
  }

  if n > two {
    factors.push(n);
  }

  factors
}

pub fn lcm(numbers: &[usize]) -> usize {
  let mut factors = Vec::<Vec<usize>>::new();
  for n in numbers {
    if *n == 0 {
      return 0;
    }
    factors.push(prime_factors(*n).into_iter().rev().collect());
  }

  let mut result = 1;

  loop {
    let mut all_empty = true;
    let mut lowest = usize::MAX;
    for fs in &factors {
      if fs.is_empty() {
        continue;
      }

      all_empty = false;
      if *fs.last().unwrap() < lowest {
        lowest = *fs.last().unwrap()
      }
    }

    if all_empty {
      break;
    }

    for fs in &mut factors {
      if fs.is_empty() {
        continue;
      }
      if *fs.last().unwrap() == lowest {
        fs.pop();
      }
    }
    result *= lowest;
  }

  result
}

pub fn lcm2<T>(numbers: &[T]) -> T
where
  T: IntegerMaths,
  <T as std::ops::Rem>::Output: PartialEq<T>,
{
  let mut factors = Vec::<Vec<T>>::new();
  for n in numbers {
    if *n == T::zero() {
      return T::zero();
    }
    factors.push(prime_factors2(*n).into_iter().rev().collect());
  }

  let mut result: T = T::one();

  loop {
    let mut all_empty = true;
    let mut lowest = T::maxval();
    for fs in &factors {
      if fs.is_empty() {
        continue;
      }

      all_empty = false;
      if *fs.last().unwrap() < lowest {
        lowest = *fs.last().unwrap()
      }
    }

    if all_empty {
      break;
    }

    for fs in &mut factors {
      if fs.is_empty() {
        continue;
      }
      if *fs.last().unwrap() == lowest {
        fs.pop();
      }
    }
    result *= lowest;
  }

  result
}

// euclidian algorithm
pub fn gcd(a: usize, b: usize) -> usize {
  let mut a = a;
  let mut b = b;
  loop {
    if b == 0 {
      return a;
    }

    (a, b) = (b, a.wrapping_rem(b));
  }
}

pub fn gcds(a: isize, b: isize) -> isize {
  let mut a = a;
  let mut b = b;
  loop {
    if b == 0 {
      return a.abs();
    }

    (a, b) = (b, a.wrapping_rem_euclid(b));
  }
}

/* a version of modulus that maps negative
 * remainders into the positive range
 */
pub fn pos_mod<T>(a: T, b: T) -> T
where
  T: num::Integer + num::Signed + Copy,
{
  let c = a % b;
  if c < num::zero() {
    c + b
  } else {
    c
  }
}

/* returns a vector of primes up to and including n.
 * Calculates prime numbers using a sieve.
 * If I needed to do this for large numbers, then I could
 * precalculate a data file up to usize_t and read from that */
pub fn primes_lte(n: usize) -> Vec<usize> {
  //test both  bitvec and bit-vec crates and Vec<bool> for sieve of erasothenes implementations
  //implement variant where we only store odd numbers in the vector and halve memory usage

  let mut composite = bitvec![0; n+1];

  composite.set(0, true);
  composite.set(1, true);
  if n >= 2 {
    composite.set(2, false);
  }

  let sqrtn = f64::sqrt(n as f64).floor() as usize;

  //for i in (3..f64::sqrt(n as f64) as usize + 1).step_by(2) {
  for i in (3..=sqrtn).step_by(2) {
    if !composite[i] {
      for j in ((i * i)..=n).step_by(i) {
        *(composite.get_mut(j).unwrap()) = true;
      }
    }
  }

  let mut primes = Vec::new();
  if n >= 2 {
    primes.push(2);
  }
  for i in (3..=n).step_by(2) {
    if !composite[i] {
      primes.push(i);
    }
  }
  primes
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pos_mod() {
    assert_eq!(pos_mod(-5, 3), 1);
    assert_eq!(pos_mod(-4, 3), 2);
    assert_eq!(pos_mod(-3, 3), 0);
    assert_eq!(pos_mod(-2, 3), 1);
    assert_eq!(pos_mod(-1, 3), 2);
    assert_eq!(pos_mod(1, 3), 1);
    assert_eq!(pos_mod(2, 3), 2);
    assert_eq!(pos_mod(3, 3), 0);
    assert_eq!(pos_mod(4, 3), 1);
  }
  #[test]
  fn test_primes_lte() {
    assert_eq!(primes_lte(1), vec![]);
    assert_eq!(primes_lte(2), vec![2]);
    assert_eq!(primes_lte(7), vec![2, 3, 5, 7]);
    assert_eq!(primes_lte(10), vec![2, 3, 5, 7]);
    assert_eq!(primes_lte(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
  }

  #[test]
  fn test_gcd() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(1, 1), 1);
    assert_eq!(gcd(0, 0), 0);
    assert_eq!(gcd(15, 10), 5);
    assert_eq!(gcd(17, 3), 1);
  }
  #[test]
  fn test_gcds() {
    assert_eq!(gcds(48, 18), 6);
    assert_eq!(gcds(1, 1), 1);
    assert_eq!(gcds(0, 0), 0);
    assert_eq!(gcds(15, 10), 5);
    assert_eq!(gcds(17, 3), 1);

    assert_eq!(gcds(-48, 18), 6);
    assert_eq!(gcds(-1, 1), 1);
    assert_eq!(gcds(-15, 10), 5);
    assert_eq!(gcds(-17, 3), 1);

    assert_eq!(gcds(48, -18), 6);
    assert_eq!(gcds(1, -1), 1);
    assert_eq!(gcds(15, -10), 5);
    assert_eq!(gcds(17, -3), 1);

    assert_eq!(gcds(-48, -18), 6);
    assert_eq!(gcds(-1, -1), 1);
    assert_eq!(gcds(-15, -10), 5);
    assert_eq!(gcds(-17, -3), 1);
  }

  #[test]
  fn test_lcm() {
    assert_eq!(lcm(&[13, 17]), 221);
    assert_eq!(lcm(&[2, 6]), 6);
    assert_eq!(lcm(&[10, 15]), 30);
    assert_eq!(lcm(&[0, 1]), 0); // possibly controversial
    assert_eq!(lcm(&[1, 1]), 1);
    assert_eq!(lcm(&[1, 3, 5, 7, 9]), 315);
    assert_eq!(lcm(&[2, 4, 6, 8]), 24);
  }

  #[test]
  fn test_lcm2() {
    assert_eq!(lcm2(&[13, 17]), 221);
    assert_eq!(lcm2(&[2, 6]), 6);
    assert_eq!(lcm2(&[10, 15]), 30);
    assert_eq!(lcm2(&[0, 1]), 0); // possibly controversial
    assert_eq!(lcm2(&[1, 1]), 1);
    assert_eq!(lcm2(&[1, 3, 5, 7, 9]), 315);
    assert_eq!(lcm2(&[2, 4, 6, 8]), 24);

    assert_eq!(lcm2(&[13_u8, 17]), 221);
    assert_eq!(lcm2(&[2_u8, 6]), 6);
    assert_eq!(lcm2(&[10_u8, 15]), 30);
    assert_eq!(lcm2(&[0_u8, 1]), 0); // possibly controversial
    assert_eq!(lcm2(&[1_u8, 1]), 1);
    assert_eq!(lcm2(&[1_u8, 3, 5, 7]), 105);
    assert_eq!(lcm2(&[2_u8, 4, 6, 8]), 24);

    assert_eq!(lcm2(&[2_i8, 6]), 6);
    assert_eq!(lcm2(&[10_i8, 15]), 30);
    assert_eq!(lcm2(&[0_i8, 1]), 0); // possibly controversial
    assert_eq!(lcm2(&[1_i8, 1]), 1);
    assert_eq!(lcm2(&[1_i8, 3, 5, 7]), 105);
    assert_eq!(lcm2(&[2_i8, 4, 6, 8]), 24);

    assert_eq!(lcm2(&[13_isize, 17]), 221);
    assert_eq!(lcm2(&[2_isize, 6]), 6);
    assert_eq!(lcm2(&[10_isize, 15]), 30);
    assert_eq!(lcm2(&[0_isize, 1]), 0); // possibly controversial
    assert_eq!(lcm2(&[1_isize, 1]), 1);
    assert_eq!(lcm2(&[1_isize, 3, 5, 7, 9]), 315);
    assert_eq!(lcm2(&[2_isize, 4, 6, 8]), 24);

    // assert_eq!(lcm2(&vec![-1_isize, 3]), -3);
    // assert_eq!(lcm2(&vec![-1_isize, -3]), 3);
  }

  #[test]
  fn test_prime_factors2() {
    assert_eq!(prime_factors2(1_u8), vec![]);
    assert_eq!(prime_factors2(2_u8), vec![2]);
    assert_eq!(prime_factors2(7_u8), vec![7]);
    assert_eq!(prime_factors2(10_u8), vec![2, 5]);
    assert_eq!(prime_factors2(20_u8), vec![2, 2, 5]);

    assert_eq!(prime_factors2(1_u16), vec![]);
    assert_eq!(prime_factors2(2_u16), vec![2]);
    assert_eq!(prime_factors2(7_u16), vec![7]);
    assert_eq!(prime_factors2(10_u16), vec![2, 5]);
    assert_eq!(prime_factors2(20_u16), vec![2, 2, 5]);

    assert_eq!(prime_factors2(1_u32), vec![]);
    assert_eq!(prime_factors2(2_u32), vec![2]);
    assert_eq!(prime_factors2(7_u32), vec![7]);
    assert_eq!(prime_factors2(10_u32), vec![2, 5]);
    assert_eq!(prime_factors2(20_u32), vec![2, 2, 5]);

    assert_eq!(prime_factors2(1_u64), vec![]);
    assert_eq!(prime_factors2(2_u64), vec![2]);
    assert_eq!(prime_factors2(7_u64), vec![7]);
    assert_eq!(prime_factors2(10_u64), vec![2, 5]);
    assert_eq!(prime_factors2(20_u64), vec![2, 2, 5]);

    assert_eq!(prime_factors2(1_u128), vec![]);
    assert_eq!(prime_factors2(2_u128), vec![2]);
    assert_eq!(prime_factors2(7_u128), vec![7]);
    assert_eq!(prime_factors2(10_u128), vec![2, 5]);
    assert_eq!(prime_factors2(20_u128), vec![2, 2, 5]);
  }

  // ---------------------------------------------
  use test::{black_box, Bencher};

  #[bench]
  fn bench_lcm(b: &mut Bencher) {
    let input = vec![1_usize, 3, 5, 7, 9];
    b.iter(|| black_box(lcm(&input)));
  }
  #[bench]
  fn bench_lcm2(b: &mut Bencher) {
    let input = vec![1_usize, 3, 5, 7, 9];
    b.iter(|| black_box(lcm2(&input)));
  }
}
