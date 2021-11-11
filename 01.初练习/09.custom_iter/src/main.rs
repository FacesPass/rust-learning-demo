// 素数结构体
struct Primes {
    limit: usize,
}

struct PrimesIter {
    index: usize,
    computed: Vec<bool>,
}

impl Primes {
    fn new(limit: usize) -> Primes {
        Primes { limit }
    }

    fn iter(&self) -> PrimesIter {
        PrimesIter {
            index: 2,
            computed: compute_primes(self.limit),
        }
    }
}

impl Iterator for PrimesIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.index += 1;
            if self.index > self.computed.len() - 1 {
                return None;
            } else if self.computed[self.index] {
                return Some(self.index);
            } else {
                continue;
            }
        }
    }
}

// 素数筛选算法
fn compute_primes(limit: usize) -> Vec<bool> {
    let mut sieve = vec![true; limit];
    let mut m = 2;
    while m * m < limit {
        if sieve[m] {
            for i in (m * 2..limit).step_by(m) {
                sieve[i] = false
            }
        }
        m += 1;
    }
    sieve
}

fn main() {
    let primes = Primes::new(1000);
    for i in primes.iter() {
        println!("{}", i);
    }
}
