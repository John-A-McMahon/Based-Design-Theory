use itertools::Itertools::{self};
use std::collections::HashMap;
use std::collections::HashSet;

pub fn create(n: i32, nth: u32) -> Vec<(i32, i32)> {
    let mut nth = nth;
    let t = (n - 1) / 2;
    let pairs = (1..n).combinations(2).collect_vec();

    let mut s: Vec<(i32, i32)> = vec![];
    let mut memory: Vec<usize> = vec![];

    for _i in 0..(t) {
        memory.push(0);
    }

    //let mut unique_memory: HashMap<Vec<usize>, Vec<(i32, i32)>> = HashMap::new();

    while s.len() < t as usize || nth > 0 {
        s = vec![];
        let mut differences: HashSet<i32> = HashSet::new();
        let mut used: HashSet<i32> = HashSet::new();
        let mut sums: HashSet<i32> = HashSet::new();

        for i in 0..memory.len() {
            let mut index = i;
            let mut ran = false;
            while memory[index] >= pairs.len() {
                memory[index] = 0;
                index -= 1;
                ran = true;
            }
            if ran {
                memory[index] += 1;
            }
            let data = memory[i];
            let pair = (pairs[data][0], pairs[data][1]);
            if can_add(n, pair, &differences, &sums, &used) {
                s.push(pair);
                insert(n, pair, &mut differences, &mut sums, &mut used);
            } else {
                let mut counter = 0;
                let mut pair = (pairs[data + counter][0], pairs[data + counter][1]);
                while data + counter < pairs.len() && !can_add(n, pair, &differences, &sums, &used)
                {
                    pair = (pairs[data + counter][0], pairs[data + counter][1]);
                    counter += 1;
                }
                if data + counter >= pairs.len() {
                    memory[i] = 0;
                    let mut index = i - 1;
                    while memory[index] + 1 >= pairs.len() {
                        memory[index] = 0;
                        if index > 0 {
                            index -= 1
                        } else {
                            break;
                        }
                    }
                    memory[index] += 1;
                    break;
                } else if can_add(n, pair, &differences, &sums, &used) {
                    memory[i] = data + counter;

                    s.push(pair);
                    insert(n, pair, &mut differences, &mut sums, &mut used);
                }
            }
        }
        if s.len() == t as usize && nth > 0 {
            nth -= 1;
            memory[(t - 1) as usize] += 1;
        } else if s.len() == t as usize && nth == 0 {
            return s;
        }
    }
    s
}

fn can_add(
    n: i32,
    pair: (i32, i32),
    differences: &HashSet<i32>,
    sums: &HashSet<i32>,
    used: &HashSet<i32>,
) -> bool {
    let a = pair.0;
    let b = pair.1;
    if a == 0 || b == 0 {
        return false;
    }
    let sum = (a + b) % n;
    let diff = {
        let mut x = a - b;
        let mut y = b - a;
        if x < 0 {
            x *= -1;
        }
        if y < 0 {
            y *= -1;
        }
        if x < y {
            x
        } else {
            y
        }
    };

    let diff = if diff > n / 2 { n - diff } else { diff };

    if sums.contains(&sum) || sum % n == 0 {
        return false;
    }

    if differences.contains(&diff) {
        return false;
    }

    if used.contains(&a) || used.contains(&b) {
        return false;
    }

    true
}

fn insert(
    n: i32,
    pair: (i32, i32),
    differences: &mut HashSet<i32>,
    sums: &mut HashSet<i32>,
    used: &mut HashSet<i32>,
) {
    let a = pair.0;
    let b = pair.1;
    let sum = (a + b) % n;
    let diff = {
        let mut x = a - b;
        let mut y = b - a;
        if x < 0 {
            x *= -1;
        }
        if y < 0 {
            y *= -1;
        }
        if x < y {
            x
        } else {
            y
        }
    };

    let diff = if diff > n / 2 { n - diff } else { diff };
    if !can_add(n, pair, differences, sums, used) {
        panic!("cannot insert");
    }
    sums.insert(sum);
    differences.insert(diff);

    used.insert(a);
    used.insert(b);
}

fn remove(
    n: i32,
    pair: (i32, i32),
    differences: &mut HashSet<i32>,
    sums: &mut HashSet<i32>,
    used: &mut HashSet<i32>,
) {
    let a = pair.0;
    let b = pair.1;
    let sum = a + b;
    let diff = {
        let mut x = a - b;
        let mut y = b - a;
        if x < 0 {
            x *= -1;
        }
        if y < 0 {
            y *= -1;
        }
        if x < y {
            x
        } else {
            y
        }
    };

    let diff = if diff > n / 2 { n - diff } else { diff };
    if can_add(n, pair, differences, sums, used) {
        panic!("cannot remove");
    }
    sums.remove(&sum);
    differences.remove(&diff);

    used.remove(&a);
    used.remove(&b);
}
