use Q26_remove_duplicates_from_sorted_array::Solution;


fn main() {
    let mut v = vec![0,0,1,1,1,2,2,3,3,4];

    Solution::remove_duplicates(&mut v);

    println!("{:#?}", v);
}