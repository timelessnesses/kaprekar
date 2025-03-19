#[derive(Debug, Clone, Copy)]
struct Kaprekar {
    value: u32,
}

#[derive(Debug, Clone, Copy)]
enum Errors {
    InvalidNumber,
}

impl Kaprekar {
    fn new(value: u32) -> Result<Self, Errors> {
        if value > 10000 || value < 1000 {
            return Err(Errors::InvalidNumber);
        }
        // range is like 1000 to 9999
        Ok(Self { value })
    }

    fn get_each_digits(&self) -> Vec<u32> {
        self.value.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect()
    }

    fn kaprekar(&mut self) -> u32 {
        let mut iterations = 0;
        loop {
            let digits = self.get_each_digits();
            let mut asc = digits.clone();
            asc.sort();
            let mut desc = asc.clone();
            desc.reverse();
            let asc_num = Self::merge_num_vec(asc);
            let desc_num = Self::merge_num_vec(desc);
            let abs_diff = (asc_num as i32 - desc_num as i32).abs() as u32;
            self.value = abs_diff;
            iterations += 1;
            if abs_diff == 6174 {
                break;
            }
        }
        iterations
    }

    fn merge_num_vec(digits: Vec<u32>) -> u32{
        digits.iter().fold(0, |acc, &d| acc * 10 + d)
    }
}

fn main() {
    let mut number= Kaprekar::new(3872).unwrap();
    let iterations = number.kaprekar();
    println!("Number of iterations: {}", iterations);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_each_digits() {
        let number = Kaprekar::new(1234).unwrap();
        let digits = number.get_each_digits();

        assert_eq!(digits, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_multiple_zeros() {
        let number = Kaprekar::new(1000).unwrap();
        let digits = number.get_each_digits();

        assert_eq!(digits, vec![1, 0, 0, 0]);
    }

    #[test]
    fn test_merge_num_vec() {
        let digits = vec![1, 2, 3, 4];
        let num = Kaprekar::merge_num_vec(digits);

        assert_eq!(num, 1234);
    }
}