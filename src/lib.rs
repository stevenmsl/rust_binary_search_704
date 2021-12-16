use std::cmp::Ordering;
pub struct Solution {}

impl Solution {
    /*
      - move start and end until you either found the target
        or there is nothing further can be searched (start <= end)
    */
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let count = nums.len();
        let mut start = 0;
        let mut end = count - 1;
        let mut pos;
        while start <= end {
            pos = start + (end - start) / 2;
            println!("start: {}, end: {}, pos:{}", start, end, pos);
            match target.cmp(&nums[pos]) {
                Ordering::Less => {
                    // search further left
                    end = pos - 1;
                }
                Ordering::Greater => {
                    // search further right
                    start = pos + 1;
                }
                Ordering::Equal => {
                    return pos.try_into().unwrap();
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        assert_eq!(4, Solution::search(nums, target));
    }
    #[test]
    fn found_last() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 12;
        assert_eq!(5, Solution::search(nums, target));
    }

    #[test]
    fn not_found() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        assert_eq!(-1, Solution::search(nums, target));
    }
}
