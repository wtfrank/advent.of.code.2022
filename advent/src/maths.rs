use bitvec::prelude::*;

/* returns a vector of prime factors in ascending order
 * If a prime factorises a number of times, there will be
 * repeats
 */
pub fn prime_factors(n: usize) -> Vec<usize> {
  let mut factors = Vec::<usize>::new();

  let mut n = n;
  while n % 2 == 0 {
    factors.push(2);
    n /= 2;
  }

  for i in (3..f64::sqrt(n as f64) as usize + 1).step_by(2) {
    while n % i == 0 {
      factors.push(i);
      n /= i;
    }
  }

  if n > 2 {
    factors.push(n);
  }

  factors
}

pub fn lcm(numbers: &[usize]) -> usize {
  let mut factors = Vec::<Vec<usize>>::new();
  for n in numbers {
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

  *(composite.get_mut(0).unwrap()) = true;
  *(composite.get_mut(1).unwrap()) = true;
  if n >= 2 {
    *(composite.get_mut(2).unwrap()) = false;
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
}
