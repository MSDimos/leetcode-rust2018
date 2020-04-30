struct Solution {}

impl Solution {
    pub fn full_justify(mut words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut lines = vec![];
        let first_word = words.remove(0);
        let mut line_buffer = vec![first_word.clone()];
        let mut line_size = first_word.len();

        for word in words {
            if word.len() + 1 + line_size > max_width as usize {
                if line_buffer.len() == 1 {
                    let padding = max_width as usize - line_size;
                    let space_padding = " ".repeat(padding);
                    let first_word = line_buffer.remove(0);
                    lines.push(format!("{}{}", first_word, space_padding));
                    line_buffer = vec![word.clone()];
                    line_size = word.len();
                    continue;
                }

                let rest = max_width as usize - line_size;
                let padding = rest / (line_buffer.len() - 1);
                let mut leftover_padding = rest % (line_buffer.len() - 1);
                let even_space = " ".repeat(padding);
                let first_word = line_buffer.remove(0);
                let line = line_buffer.into_iter().fold(first_word, |line, word| {
                    let uneven_space = if leftover_padding > 0 { " " } else { "" };
                    if leftover_padding > 0 {
                        leftover_padding -= 1;
                    }

                    format!("{} {}{}{}", line, even_space, uneven_space, word)
                });
                lines.push(line);
                line_buffer = vec![word.clone()];
                line_size = word.len();
            } else {
                line_size += 1 + word.len();
                line_buffer.push(word);
            }
        }

        if line_buffer.len() > 0 {
            let padding = max_width as usize - line_size;
            let space_padding = " ".repeat(padding);
            let mut line = line_buffer.join(" ");
            line.push_str(&space_padding);
            lines.push(line);
        }

        lines
    }
}


fn main() {
    let input = vec![
        "This".to_string(), "is".to_string(), "an".to_string(),
        "example".to_string(), "of".to_string(), "text".to_string(),
        "justification.".to_string()
    ];
    let output = vec![
        "This    is    an".to_string(),
        "example  of text".to_string(),
        "justification.  ".to_string()
    ];

    assert_eq!(output, Solution::full_justify(input, 16));
}
