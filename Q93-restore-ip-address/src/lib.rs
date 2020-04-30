struct Solution {}

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let bytes = s.into_bytes();
        let mut ips = vec![];
        let mut ip = vec![];

        if bytes.len() < 4 {
            return vec![];
        }

        Solution::helper(&bytes[..], &mut ip, &mut ips);
        ips
    }

    fn helper(bytes: &[u8], ip: &mut Vec<String>, ips: &mut Vec<String>) {
        if ip.len() == 4 {
            if bytes.is_empty() {
                if let Some(ip_str) = Solution::validate_ip(ip) {
                    ips.push(ip_str);
                }
            } else {
                return;
            }
        } else if (4 - ip.len()) * 3 < bytes.len() {
            // max length of segment is 3(100 - 255),
            // if length of reset bytes is longer than (4 - ip.len()) * 3
            // it means that can't convert segments anymore
            return;
        } else {
            let mut segment = String::new();

            for i in 0..bytes.len() {
                // one segment consists of most three chars
                if i <= 3 {
                    segment.push(bytes[i] as char);
                    ip.push(segment.clone());
                    Solution::helper(&bytes[(i + 1)..], ip, ips);
                    ip.pop();
                } else {
                    break;
                }
            }
        }
    }

    fn validate_ip(ip: &Vec<String>) -> Option<String> {
        for segment in ip {
            if segment != "0" && segment.starts_with("0") {
                return None;
            } else if segment.len() > 3 {
                return None;
            } else if segment.parse::<u8>().is_err() {
                return None;
            }
        }

        Some(format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]))
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use crate::Solution;

    #[test]
    fn it_works() {
        let ips = Solution::restore_ip_addresses("25525511135".to_string());
        assert_eq!(ips, ["255.255.11.135".to_string(), "255.255.111.35".to_string()]);


        let ips = Solution::restore_ip_addresses("010010".to_string());
        assert_eq!(ips, ["0.10.0.10".to_string(),"0.100.1.0".to_string()]);
    }
}
