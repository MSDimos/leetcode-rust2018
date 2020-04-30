use std::collections::{ HashSet, HashMap };
use std::cell::Cell;
use std::borrow::Borrow;

struct Solution {}

impl Solution {

    // backtracking
    fn helper(
        result: &mut Vec<Vec<i32>>, sample: &mut Vec<i32>,
        pool: &mut HashSet<usize>, sample_pool: &mut HashSet<Vec<i32>>, nums: &Vec<i32>
    ) {
        for (idx, num) in nums.iter().enumerate() {
            if !pool.contains(&idx) {
                sample.push(*num);
                pool.insert(idx);

                if sample.len() == nums.len() && !sample_pool.contains(sample) {
                    // IDEA bug, add type converting syntax to solve it
                    result.push(sample.clone() as Vec<i32>);
                    sample_pool.insert(sample.clone() as Vec<i32>);
                } else {
                    Solution::helper(result, sample, pool, sample_pool, nums);
                }

                pool.remove(&idx);
                sample.pop();
            }
        }
    }
    fn push_item(stack: &mut Vec<i32>, map: &mut HashMap<i32, usize>, result: &mut Vec<Vec<i32>>, len: usize) {
        if stack.len() == len {
            result.push(stack.clone() as Vec<i32>);
            return;
        }

        for (item, times) in map.clone().iter() {
            stack.push(*item);
            if *times >= 1 {
                map.insert(*item, *times - 1);
                Solution::push_item(stack, map, result, len);
                map.insert(*item, *times);
            }
            stack.pop();
        }
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // let mut result = vec![];
        // let mut sample = vec![];
        // let mut pool = HashSet::new();
        // let mut sample_pool = HashSet::new();
        //
        // Solution::helper(&mut result, &mut sample, &mut pool, &mut sample_pool, &nums);
        // result


        // solution 2
        let mut stack = vec![];
        let mut result = vec![];
        let mut map = {
            let mut m = HashMap::new();
            for i in nums.iter() {
                if m.contains_key(i) {
                    if let Some(times) = m.get_mut(i) {
                        *times += 1;
                    }
                } else {
                    m.insert(i.clone(), 1);
                }
            }
            m
        };

        Solution::push_item(&mut stack, &mut map, &mut result, nums.len());
        result
    }
}

fn main() {
    // let mut a = vec![vec![]];
    // let mut b = vec![1, 2, 3, 4];
    // let ptr = &mut b;
    //
    // a.push(ptr.clone());
    // eprintln!("{:#?}", ptr);

    // assert_eq!(
    //     vec![
    //         vec![1,1,2],
    //         vec![1,2,1],
    //         vec![2,1,1],
    //     ],
    //     Solution::permute_unique(vec![1, 1, 2])
    // );
    fn next(mut nums: Vec<i32>) -> Vec<i32> {
        if let Some(prev) = (0..nums.len()-1).rposition(|x| nums[x] < nums[x+1]) {
            let j = nums.iter().rposition(|&x| x > nums[prev]).unwrap();
            nums.swap(prev, j);
            nums[prev+1..].reverse();
        } else {
            nums.reverse();
        }
        nums
    }

    println!("{:#?}", next(vec![1, 1, 2]));
}
