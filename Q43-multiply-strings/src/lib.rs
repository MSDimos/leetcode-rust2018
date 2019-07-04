use std::mem::swap;

pub struct Solution;

impl Solution {
    // solution 2
    // it's easy to understand but it's complicated
    pub fn multiply2(num1: String, num2: String) -> String {

        if num1 == "0".to_string() || num2 == "0".to_string() {
            return "0".to_string();
        }

        let num1: Vec<u8> = num1.into_bytes().iter().map(|x| x - 48).collect();
        let num2: Vec<u8> = num2.into_bytes().iter().map(|x| x - 48).collect();
        let mut i = num2.len();
        let mut ans = vec![];

        while i > 0 {
            i -= 1;
            let mut a = Solution::multiply_signal_number(&num1, num2[i]);

            a.append(&mut vec![0; (num2.len() - i - 1)]);
            ans = Solution::add(a, ans);
        }

        ans.iter_mut().for_each(|x| *x = *x + 48);
        String::from_utf8(ans).unwrap_or_default()
    }

    pub fn multiply_signal_number(num: &Vec<u8>, digit: u8) -> Vec<u8> {

        if digit == 0 {
            return vec![0];
        }

        let mut i = num.len();
        let mut  product= 0;
        let mut carry = 0;
        let mut res = vec![];

        while i > 0 {
            i -= 1;
            product = num[i] * digit + carry;
            carry = product / 10;
            product %= 10;
            res.push(product);
        }

        if carry > 0 {
            res.push(carry);
        }

        res.reverse();
        res
    }

    pub fn add(mut num1: Vec<u8>, mut num2: Vec<u8>) -> Vec<u8> {
        let diff = (num1.len() as i32 - num2.len() as i32).abs() as usize;
        let mut i = num1.len().max(num2.len());
        let mut sum = 0;
        let mut carry = 0;

        if num2.len() > num1.len() {
            swap(&mut num1, &mut num2);
        }

        while i > 0 {
            i -= 1;
            if i >= diff {
                sum = num1[i] + num2[i - diff] + carry;
                carry = sum / 10;
                sum %= 10;
                num1[i] = sum;
            } else {
                sum = num1[i] + carry;
                carry = sum / 10;
                sum %= 10;
                num1[i] = sum;
            }
        }

        if carry > 0 {
            num1.insert(0, carry);
        }

        num1
    }

    // solution 1
    // harder to understand than solution 1
    // but it's more clever
    pub fn multiply(mut num1: String, mut num2: String) -> String {

        if &num1 == "0" || &num2 == "0" {
            return "0".to_string();
        }

        let mut num1: Vec<usize> = num1.into_bytes().into_iter().map(|mut x| { x -= 48; x as usize }).collect();
        let mut num2: Vec<usize> = num2.into_bytes().into_iter().map(|mut x| { x -= 48; x as usize}).collect();
        let mut store = vec![0; (num1.len() + num2.len() - 1)];

        for _i in 0..num2.len() {
            for _j in 0..num1.len() {
                let i = num2.len() - _i - 1;
                let j = num1.len() - _j - 1;
                store[i + j] += num1[j] * num2[i];
            }
        }

        let mut k = store.len();
        let mut carry = 0;

        while k > 0 {
            k -= 1;
            store[k] += carry;
            carry = store[k] / 10;
            store[k] %= 10;
        }

        if carry > 0 {
            store.insert(0, carry);
        }

        String::from_utf8(store.into_iter().map(|mut x| { x += 48; x as u8 }).collect()).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn it_works() {
        let r = Solution::multiply_signal_number(&vec![1, 9, 9], 2);
        assert_eq!(r, vec![3, 9, 8]);

        let r = Solution::add(vec![1, 2, 3], vec![4, 5, 6]);
        assert_eq!(r, vec![5, 7, 9]);

        let r = Solution::multiply(String::from("123"), String::from("456"));
        assert_eq!(r, String::from("56088"));

        let r = Solution::multiply(String::from("99999"), String::from("321"));
        assert_eq!(r, String::from("32099679"));

        let r = Solution::multiply(String::from("0"), String::from("123"));
        assert_eq!(r, String::from("0"));

        let r = Solution::multiply(String::from("99999"), String::from("0"));
        assert_eq!(r, String::from("0"));

        let r = Solution::multiply2(String::from("456"), String::from("321"));
        assert_eq!(r, String::from("146376"));
    }
}
