#[derive(Debug, Clone, Copy, PartialEq)]
struct Kaprekar {
    value: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Errors {
    InvalidNumber,
    SameConsecutiveDigits,
    InvalidNumberDoingKaprekar
}

impl Kaprekar {
    fn new(value: u32) -> Result<Self, Errors> {
        if value > 10000 || value < 1000 {
            return Err(Errors::InvalidNumber);
        }
        let digits = Kaprekar::get_each_digits(value);
        if digits.iter().all(|&d| d == digits[0]) {
            return Err(Errors::SameConsecutiveDigits);
        }
        Ok(Self { value })
    }

    fn get_each_digits(value: u32) -> [Option<u32>; 4] {
        let mut digits = [None; 4];
        
        for (i, ch) in value.to_string().chars().enumerate() {
            digits[i] = Some(ch.to_digit(10).unwrap());
        }

        digits
        // value.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect()
    }

    fn kaprekar(&mut self) -> Result<u32, Errors> {
        let mut iterations = 0;
        loop {
            let digits = Kaprekar::get_each_digits(self.value);
            let mut asc = digits.clone();
            asc.sort();
            let mut desc = asc.clone();
            desc.reverse();
            let asc_num = Self::merge_num_vec(asc);
            let desc_num = Self::merge_num_vec(desc);
            dbg!(asc_num, desc_num);
            let abs_diff = dbg!((asc_num as i32 - desc_num as i32).abs() as u32);
            if Self::get_each_digits(abs_diff).iter().filter(|i| i.is_some()).count() != 4 {
                return Err(Errors::InvalidNumberDoingKaprekar);
            }
            self.value = abs_diff;
            iterations += 1;
            if abs_diff == 6174 {
                break;
            }
            dbg!(iterations);
        }
        Ok(iterations)
    }

    fn merge_num_vec(digits: [Option<u32>; 4]) -> u32 {
        digits.iter().fold(0, |acc, &d| match d {
            Some(d) => acc * 10 + d,
            None => acc,
        })
    }
}

fn main() {
    let mut number= Kaprekar::new(std::env::args().nth(1).unwrap_or("3872".to_string()).parse::<u32>().unwrap()).unwrap();
    let iterations = number.kaprekar();
    println!("Number of iterations: {}", iterations.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_each_digits() {
        let number = Kaprekar::get_each_digits(1234);
        assert_eq!(number.map(|i| i.unwrap()), [1, 2, 3, 4]);
    }

    #[test]
    fn test_multiple_zeros() {
        let number = Kaprekar::get_each_digits(1000);

        assert_eq!(number.map(|i| i.unwrap()), [0, 0, 0, 0]);
    }

    #[test]
    fn test_merge_num_vec() {
        let digits = [1, 2, 3, 4];
        let num = Kaprekar::merge_num_vec(digits.map(|d| Some(d)));

        assert_eq!(num, 1234);
    }

    #[test]
    fn test_kaprekar() {
        let mut number = Kaprekar::new(3872).unwrap();
        let iterations = number.kaprekar();

        assert_eq!(iterations, Ok(4));
    }

    #[test]
    fn test_error_consecutive_digits() {
        let number = Kaprekar::new(9999);

        assert_eq!(number, Err(Errors::SameConsecutiveDigits));
    }
}