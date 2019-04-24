pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            s
        } else {
            let chars: Vec<char> = s.chars().collect();
            let mut rows: Vec<String> = vec![];
            let mut current_row = 0;
            let mut direction = false;

            for chr in chars {
                if rows.get(current_row).is_some() {
                    rows[current_row].push(chr);
                } else {
                    rows.push(String::new());
                    rows[current_row].push(chr);
                }

                if current_row == 0 || current_row == ((num_rows - 1) as usize) {
                    direction = !direction;
                }

                if direction {
                    current_row += 1;
                } else {
                    current_row -= 1;
                }
            }

            rows.join("")
        }

    }
}