// Helper function to check if a number is prime
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    let mut i = 5;
    // We only need to check up to the square root of n
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    
    true
}

pub fn prev_prime(nbr: u64) -> u64 {
    // If the number is 2 or less, there are no smaller primes.
    if nbr <= 2 {
        return 0;
    }
    
    // Start checking from the number right below `nbr`
    let mut current = nbr - 1;
    
    while current >= 2 {
        if is_prime(current) {
            return current;
        }
        current -= 1;
    }
    
    0
}

/*
fn is_prime(n: u64) -> bool {
  if n <= 1 {
    return false;
  }
  if n <=3 {
      return true;
  }
  if n % 2 == 0 || n % 3 == 0 {
      return false;
  }

  let mut i = 5;
  while i * i <= n {
      if n % i == 0 || n % ( i + 2 ) == 0{
          return false;
      }
      i = i + 6;
    }
    true
}

pub fn prev_prime(nbr: u64) -> u64 {
    if nbr <= 2 {
        return 0;
    }
    let mut current = nbr - 1;
    while current >= 2 {
        if is_prime(current) {
        return current;
    }
    current = current - 1;
  }
  0
}


 */