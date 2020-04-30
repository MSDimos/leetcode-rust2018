struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (mut a_bytes, mut b_bytes) = (a.into_bytes(), b.into_bytes());
        let mut result = vec![];
        let diff = (a_bytes.len() as isize - b_bytes.len() as isize).abs();
        let (mut short_bytes, mut long_bytes) = if a_bytes.len() < b_bytes.len() {
            (a_bytes, b_bytes)
        } else {
            (b_bytes, a_bytes)
        };
        let mut carry = 0;

        short_bytes.reverse();
        long_bytes.reverse();
        short_bytes.append(&mut vec![b'0'; diff as usize]);

        for i in 0..long_bytes.len() {
            let sum = short_bytes[i] + long_bytes[i] - b'0' + carry;

            result.push((sum - b'0') % 2 + b'0');
            carry = sum / b'2';
        }

        if carry == 1 {
            result.push(b'1');
        }

        result.reverse();
        String::from_utf8(result).unwrap()
    }
}

fn main() {
    assert_eq!("100".to_string(), Solution::add_binary("11".to_string(), "1".to_string()));
    assert_eq!("0".to_string(), Solution::add_binary("0".to_string(), "0".to_string()));
    assert_eq!("0".to_string(), Solution::add_binary("0".to_string(), "".to_string()));
    assert_eq!("10101".to_string(), Solution::add_binary("1010".to_string(), "1011".to_string()));
    assert_eq!("11110".to_string(), Solution::add_binary("1111".to_string(), "1111".to_string()));
}
