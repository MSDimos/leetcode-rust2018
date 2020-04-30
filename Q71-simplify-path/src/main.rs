struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut segments: Vec<&str> = path.split_terminator('/').collect();
        let mut canonical_path = vec![];

        segments.retain(|&str| {!str.is_empty()});

        for segment in segments {
            if segment == "." {
                continue;
            } else if segment == ".." {
                if !canonical_path.is_empty() {
                    canonical_path.pop();
                }
            } else {
                canonical_path.push(segment);
            }
        }

        let mut result = String::new();

        for segment in canonical_path {
            result.push_str("/");
            result.push_str(segment);
        }

        if result.is_empty() {String::from("/")} else {result}
    }
}

fn main() {
    assert_eq!("/home".to_string(), Solution::simplify_path("/home".to_string()));
    assert_eq!("/".to_string(), Solution::simplify_path("/../".to_string()));
    assert_eq!("/home/foo".to_string(), Solution::simplify_path("/home//foo/".to_string()));
    assert_eq!("/c".to_string(), Solution::simplify_path("/a/./b/../../c/".to_string()));
    assert_eq!("/c".to_string(), Solution::simplify_path("/a/../../b/../c//.//".to_string()));
    assert_eq!("/a/b/c".to_string(), Solution::simplify_path("/a//b////c/d//././/..".to_string()));
}
