// https://leetcode.com/problems/two-sum/description/

// Problem:
// Given an array of integers nums and an integer target, return indices of the 
// two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, 
// and you may not use the same element twice.

// You can return the answer in any order.

use std::collections::HashMap;

fn main() {

    println!("{:?}", another_two_sum(vec![2,7,11,15], 9));
    println!("{:?}", another_two_sum(vec![3,2,4], 6));
    println!("{:?}", another_two_sum(vec![3,3], 6));
}

fn two_sum(nums: Vec<i8>, target: i8) -> Vec<i8> {
    for x in 0..nums.len() {
        for y in 0..nums.len() {
            if x != y && nums[x] + nums[y] == target {
                return vec![x.try_into().unwrap(), y.try_into().unwrap()];
            }
        }
    }

    vec![]
}

fn another_two_sum(nums: Vec<i8>, target: i8) -> Vec<i8> {
    let mut result_vec: Vec<i8> = vec![];
    let mut map: HashMap<i8, i8> = HashMap::new();

    for x in 0..nums.len() {
        let tmp: i8 = target - nums[x];

        if map.contains_key(&tmp) {
            result_vec.push(*map.get(&tmp).unwrap());
            result_vec.push(x.try_into().unwrap());
        } else {
            map.insert(nums[x], x.try_into().unwrap());
        }
    } 

    result_vec
}

#[cfg(test)]
mod tests {
    use crate::two_sum;
    use crate::another_two_sum;

    #[test]
    fn test_two_sum() {
         assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0,1]);
         assert_eq!(two_sum(vec![3,2,4], 6), vec![1,2]);
         assert_eq!(two_sum(vec![3,3], 6), vec![0,1]);
    }

    #[test]
    fn test_another_two_sum() {
         assert_eq!(another_two_sum(vec![2,7,11,15], 9), vec![0,1]);
         assert_eq!(another_two_sum(vec![3,2,4], 6), vec![1,2]);
         assert_eq!(another_two_sum(vec![3,3], 6), vec![0,1]);
    }
}
