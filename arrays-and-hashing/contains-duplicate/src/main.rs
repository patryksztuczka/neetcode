// https://leetcode.com/problems/contains-duplicate/description/

// Problem:
// Given an integer array nums, return true if any value 
// appears at least twice in the array, and return false 
// if every element is distinct.

use std::collections::HashSet;

fn main() {}

// Time complexity: O(n^2)
// Memory complexity: O(1)
fn contains_duplicate(nums: Vec<i8>) -> bool {
    let mut contains_duplicate = false;

    for x in 0..nums.len() {
        for y in 0..nums.len() {
            if x != y && nums[x] == nums[y] {
                contains_duplicate = true;
            }
        }
    }

    contains_duplicate
}

// Time complexity: O(nlogn)
// Memory complexity: O(1)
fn better_contains_duplicate(mut nums: Vec<i8>) -> bool {
    let mut contains_duplicate = false;

    nums.sort();

    for x in 0..nums.len() {
        if x + 1 < nums.len() && nums[x] == nums[x + 1] {
            contains_duplicate = true;
        }
    }

    contains_duplicate
}

// Time complexity: O(n)
// Memory complexity: O(n)
fn the_best_contains_duplicate(nums: Vec<i8>) -> bool {
    let mut hashset: HashSet<i8> = HashSet::new();

    for num in nums {
        if hashset.contains(&num) {
            return true;
        }
        hashset.insert(num);
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::contains_duplicate;
    use crate::better_contains_duplicate;
    use crate::the_best_contains_duplicate;

    #[test]
    fn test_contains_duplicate() {
        let nums1 = vec![1,2,3,1];
        let nums2 = vec![1,2,3,4];
        let nums3 = vec![1,1,1,3,3,4,3,2,4,2];

         assert_eq!(contains_duplicate(nums1), true);
         assert_eq!(contains_duplicate(nums2), false);
         assert_eq!(contains_duplicate(nums3), true);
    }

    #[test]
    fn test_better_contains_duplicate() {
        let nums1 = vec![1,2,3,1];
        let nums2 = vec![1,2,3,4];
        let nums3 = vec![1,1,1,3,3,4,3,2,4,2];

        assert_eq!(better_contains_duplicate(nums1), true);
        assert_eq!(better_contains_duplicate(nums2), false);
        assert_eq!(better_contains_duplicate(nums3), true);
    }

    #[test]
    fn test_the_best_contains_duplicate() {
        let nums1 = vec![1,2,3,1];
        let nums2 = vec![1,2,3,4];
        let nums3 = vec![1,1,1,3,3,4,3,2,4,2];

        assert_eq!(the_best_contains_duplicate(nums1), true);
        assert_eq!(the_best_contains_duplicate(nums2), false);
        assert_eq!(the_best_contains_duplicate(nums3), true);
    }
}
