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
