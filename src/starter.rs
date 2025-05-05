use itertools::Itertools::{self};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Starter {
    n: i32,
    pairs: Vec<(i32, i32)>,
}

impl Starter {
    pub fn new(n: i32, pairs: Vec<(i32, i32)>) -> Result<Self, String> {
        if n % 2 != 1 {
            return Err(
                "N must be an odd number because a starter only applies to an odd group"
                    .to_string(),
            );
        }

        let mut dists = vec![];
        let mut unique = HashMap::new();
        for i in 1..(n / 2 + 1 + 1) {
            dists.push(i);
        }

        for pair in &pairs {
            let pair = *pair;
            if unique.contains_key(&pair.0) || unique.contains_key(&pair.1) {
                return Err("Oh no there is a duplicate dude!".to_string());
            }
            unique.insert(pair.0, true);
            unique.insert(pair.1, true);
            let mut a = (pair.0 - pair.1) % n;
            if a < 0 {
                //       a += n;

                a *= -1;
            }
            let mut b = (pair.1 - pair.0) % n;
            if b < 0 {
                //b += n;
                b *= -1;
            }

            let mut x = {
                if a < b {
                    a
                } else {
                    b
                }
            };
            if x > n / 2 {
                x = n - x;
            }
            if dists.contains(&x) {
                let index = dists.iter().position(|e| *e == x).unwrap();
                dists.remove(index);
            } else {
                return Err("not a valid starter, {x}".to_string());
            }
        }
        if dists.len() > 1 {
            return Err(format!(
                "not a valid starter, not all dists used {:?}",
                dists
            ));
        }

        Ok(Self { n, pairs })
    }
    pub fn new_patterned(n: i32, k: i32) -> Self {
        if n % 2 != 1 {
            panic!("N must be an odd number because a starter only applies to an odd group")
        }
        let mut pairs = vec![];

        for i in 1..(n / 2 + 1) {
            //let mut a = ((i + 1) - (n + 1) / 2 + k / 2) % n;
            //let mut b = ((n - i) - (n + 1) / 2 + k / 2) % n;

            let mut a = ((k / 2) + i) % n;
            let mut b = ((k / 2) - i) % n;
            if a < 0 {
                a += n;
            }
            if b < 0 {
                b += n;
            }
            pairs.push((a, b));
        }

        let mut dists = vec![];
        let mut unique = HashMap::new();
        for i in 1..((n + 1) / 2) {
            dists.push(i);
        }

        for pair in &pairs {
            let pair = *pair;
            if unique.contains_key(&pair.0) || unique.contains_key(&pair.1) {
                panic!("Oh no there is a duplicate dude!");
            }
            unique.insert(pair.0, true);
            unique.insert(pair.1, true);
            let mut a = (pair.0 - pair.1) % n;
            if a < 0 {
                //       a += n;

                a *= -1;
            }
            let mut b = (pair.1 - pair.0) % n;
            if b < 0 {
                //b += n;
                b *= -1;
            }

            let mut x = {
                if a < b {
                    a
                } else {
                    b
                }
            };
            if x > n / 2 {
                x = n - x;
            }
            if dists.contains(&x) {
                let index = dists.iter().position(|e| *e == x).unwrap();
                dists.remove(index);
            } else {
                panic!("not a valid starter, {x}");
            }
        }
        if dists.len() > 1 {
            panic!("not a valid starter, not all dists used {:?}", dists);
        }

        Self { n, pairs }
    }

    pub fn is_strong(&self) -> bool {
        let mut sums = HashSet::new();
        let mut dists = HashSet::new();
        let mut distinct = HashSet::new();

        for pair in &self.pairs {
            let sum = pair.0 + pair.1;
            let dist = {
                let mut a = (pair.0 - pair.1) % self.n;
                let mut b = (pair.1 - pair.0) % self.n;
                if a < 0 {
                    a *= -1;
                }
                if b < 0 {
                    b *= -1;
                }
                if a < b {
                    a
                } else {
                    b
                }
            };

            if sums.contains(&sum) || sum == 0 {
                println!("sum issue");
                return false;
            }
            if dists.contains(&dist) {
                println!("dist issue");
                return false;
            }
            if distinct.contains(&pair.0) || distinct.contains(&pair.1) {
                println!("duplicate issue");
                return false;
            }
            if pair.0 == 0 || pair.1 == 0 {
                return false;
            }
            sums.insert(sum);
            dists.insert(dist);
            distinct.insert(pair.0);
            distinct.insert(pair.1);
        }
        distinct.len() == self.n as usize - 1
    }

    pub fn new_strong(n: i32, mut seed: i32) -> Self {
        let pairs = (0..n).combinations(2);
        let potentials = pairs.combinations((n as usize - 1) / 2);

        for potential in potentials {
            let mut temp = vec![];
            for v in potential {
                temp.push((v[0], v[1]));
            }
            let maybe = Self::new(n, temp);
            if let Ok(x) = maybe {
                if x.is_strong() && seed == 0 {
                    return x;
                } else if x.is_strong() {
                    seed -= 1;
                }
            }
        }
        panic!("failed to build a new strong starter");
    }

    pub fn hamilton(n: i32, k: i32, strong: &Self, should_print: bool) -> bool {
        //bijection = 1(x)+1
        let patterned = Self::new_patterned(n, k);
        let f_p = patterned;

        let start = 0;
        let mut cur = start;
        let mut count = n - 1;
        //println!("{:?}", f_p);
        //println!("{:?}", strong);

        if should_print {
            print!("{}", start);
        }

        let mut order = HashSet::new();
        while count > 0 {
            for pair in &f_p.pairs {
                if pair.0 == cur {
                    if should_print {
                        print!("->{}", pair.1);
                    }

                    cur = pair.1;
                    break;
                } else if pair.1 == cur {
                    if should_print {
                        print!("->{}", pair.0);
                    }
                    cur = pair.0;
                    break;
                }
            }
            if order.contains(&cur) {
                return false;
            }
            order.insert(cur);
            count -= 1;
            for pair in &strong.pairs {
                if pair.0 == cur {
                    if should_print {
                        print!("=>{}", pair.1);
                    }
                    cur = pair.1;
                    break;
                } else if pair.1 == cur {
                    if should_print {
                        print!("=>{}", pair.0);
                    }
                    cur = pair.0;
                    break;
                }
            }
            count -= 1;

            if order.contains(&cur) {
                return false;
            }
            order.insert(cur);
        }
        order.len() == (n - 1) as usize
    }
    pub fn get_pairs(&self) -> Vec<(i32, i32)> {
        let mut copy = self.pairs.clone();
        copy.sort();
        return copy;
    }
}
