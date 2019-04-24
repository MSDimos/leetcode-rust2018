pub struct Solution;

const ROMAN_NUMBER: [[&'static str; 9]; 4] = [
    ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
    ["X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
    ["C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
    ["M", "MM", "MMM", "", "", "", "", "", ""],
];

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut roman = String::new();
        let mut low = 0;
        let mut level = 0;

        while num > 0 {
            low = (num % 10) as usize;
            if low != 0 {
                roman.insert_str(0, &ROMAN_NUMBER[level][low - 1]);
            }
            num /= 10;
            level += 1;
        }

        roman
    }
}