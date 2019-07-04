pub struct Solution;

impl Solution {
    pub fn add_strings(mut num1: String, mut num2: String) -> String {

        if num1.len() < num2.len() {
            num1 = "0".repeat(num2.len() - num1.len()) + &num1;
        } else {
            num2 = "0".repeat(num1.len() - num2.len()) + &num2;
        }

        let num_bytes1: Vec<u8> = num1.as_bytes().iter().map(|x| x - 48).collect();
        let mut num_bytes2: Vec<u8> = num2.as_bytes().iter().map(|x| x - 48).collect();
        let mut i = num_bytes1.len();
        let mut carry = 0;
        assert_eq!(num_bytes1.len(), num_bytes2.len());
        while i > 0 {
            i -= 1;
            let mut sum = num_bytes1[i] + num_bytes2[i] + carry;
            carry = if sum >= 10 { sum -= 10; 1 } else { 0 };
            num_bytes2[i] = sum;
        }

        if carry > 0 {
            // ascii code 49 is equal to char "1"
            // num_bytes2.insert(0, 49);
            let s = String::from_utf8(
                num_bytes2
                .iter()
                .map(|x| x + 48)
                .collect()
            ).unwrap_or_default();

            return "1".to_string() + &s;
        } else {

        }

        return String::from_utf8(
            num_bytes2
                .iter()
                .map(|x| x + 48).collect()
        ).unwrap_or_default();
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let r = Solution::add_strings(String::from("199999999"), String::from("1"));
        assert_eq!(r, String::from("200000000"));
        let r = Solution::add_strings(String::from("0"), String::from("0"));
        assert_eq!(r, String::from("0"));
        let r = Solution::add_strings(String::from("1"), String::from("1"));
        assert_eq!(r, String::from("2"));
        let r = Solution::add_strings(String::from("123"), String::from("456"));
        assert_eq!(r, String::from("579"));
        let r = Solution::add_strings(String::from("9999"), String::from("2"));
        assert_eq!(r, String::from("10001"));
    }
}
