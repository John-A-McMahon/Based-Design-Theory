use std::collections::HashMap;
pub struct LatinSquare {
    square: Vec<Vec<i32>>,
    transpose: Vec<Vec<i32>>,
}

impl LatinSquare {
    pub fn new(square: Vec<Vec<i32>>) -> Result<Self, String> {
        let rows = square.clone();

        let mut transpose = vec![];
        for i in 0..square.len() {
            transpose.push(vec![]);
            for j in 0..square.len() {
                transpose[i].push(square[i][j]);
            }
        }

        let latin = Self { square, transpose };

        let x = latin.is_valid();
        if x.is_err() {
            println!("{:?}", x);
            return Err("Failed to create square".to_string());
        }

        Ok(latin)
    }

    fn is_valid(&self) -> Result<bool, String> {
        let mut symbols = HashMap::new();
        for row in &self.square {
            for col in row {
                symbols.insert(col, 1);
            }
        }
        if symbols.len() > self.square.len() {
            println!("{:?}", symbols.len());
            return Err("Too many unique symbols".to_string());
        }
        for row in &self.square {
            let mut nums: HashMap<i32, i32> = HashMap::new();
            for col in row {
                if nums.contains_key(col) {
                    return Err(format!("There is a duplicate number in {:?} ", row));
                } else {
                    nums.insert(*col, 0);
                }
            }
            if nums.len() < self.square.len() {
                return Err("There is an issue with the row not having enough ".to_string());
            }
        }

        for row in &self.transpose {
            let mut nums: HashMap<i32, i32> = HashMap::new();
            for col in row {
                if nums.contains_key(col) {
                    return Err(format!("There is a duplicate number in {:?} ", row));
                } else {
                    nums.insert(*col, 0);
                }
            }
            if nums.len() < self.square.len() {
                return Err("There is an issue with the row not having enough ".to_string());
            } else if nums.len() > self.square.len() {
                return Err("Too many unique symbols".to_string());
            }
        }

        Ok(true)
    }
}
