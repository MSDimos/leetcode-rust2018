use Q14_longest_common_prefix::Solution;

fn main() {
    let s = Solution::longest_common_prefix(
        (["aaa","aa","aaa"])
            .iter()
            .map(|&x| String::from(x))
            .collect::<Vec<String>>()
    );

    println!("{}", s);
}