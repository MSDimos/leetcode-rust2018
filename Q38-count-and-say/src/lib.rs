pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut prev = String::from("1");
        let mut cur = String::new();
        let mut i = 0;
        let mut stack = vec![];

        while i < n - 1 {
            i += 1;

            for num in prev.chars() {
                if let Some(t) = stack.last() {
                    if t != &num {
                        cur.push_str(&Solution::pop_top(&mut stack));
                    }
                    stack.push(num);
                } else {
                    stack.push(num);
                }
            }

            cur.push_str(&Solution::pop_top(&mut stack));
            prev = cur;
            cur = String::new();
            stack.clear();
        }

        prev
    }

    fn pop_top(stack: &mut Vec<char>) -> String {
        if stack.is_empty() {
            String::new()
        } else {
            let top = stack.pop();
            let mut times = 1;

            while !stack.is_empty() {
                if stack.last() == top.as_ref() {
                    stack.pop();
                    times += 1;
                } else {
                    break;
                }
            }

            format!("{}{}", times, top.as_ref().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let c = Solution::count_and_say(5);
        assert_eq!(&c, "11");
    }
}
