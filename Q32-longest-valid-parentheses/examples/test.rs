use Q32_longest_valid_parentheses::Solution;

fn main() {
    let len = Solution::longest_valid_parentheses(String::from("()()()(())"));

    println!("{}", len);
}