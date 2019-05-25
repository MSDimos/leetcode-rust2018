use Q27_remove_element::Solution;


fn main() {
    let mut v = vec![0,1,2,2,3,0,4,2];

    Solution::remove_element(&mut v, 2);

    println!("{:#?}", v);
}