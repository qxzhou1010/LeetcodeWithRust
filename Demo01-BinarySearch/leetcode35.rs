use std::vec::Vec;

struct Solution {}


impl Solution {
      pub fn search_insert_2022_0905(nums: Vec<i32>, target: i32) -> i32 {
        //
        if nums.is_empty() {
            return -1;
        }
        if target < nums[0] {
            return 0;
        }
        if target > nums[nums.len() - 1] {
            return nums.len() as i32;
        }
        // start to binary search
        let mut start = 0;
        let mut end = nums.len() - 1;
        while start + 1 < end {

            let mut mid = (end - start)/2 + start;
            if *nums.get(mid).unwrap() == target {
                return mid as i32;
            }else if *nums.get(mid).unwrap() < target {
                start = mid;
            }else {
                end = mid;
            }
        }
        // double check 
        if *nums.get(start).unwrap() == target {
            return start as i32;
        }
        return end as i32;
    }

    // pub fn search_insert_2022_0905_02(nums: Vec<i32>, target: i32) -> i32 {


    // }
}

fn main() {
    // just for complier
}