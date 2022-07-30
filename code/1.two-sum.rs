// Category: algorithms
// Level: Easy
// Percent: 48.887383%


/*
 * Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 * 
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 * 
 * You can return the answer in any order.
 * 
 *  
 * Example 1:
 * 
 * Input: nums = [2,7,11,15], target = 9
 * Output: [0,1]
 * Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
 * 
 * 
 * Example 2:
 * 
 * Input: nums = [3,2,4], target = 6
 * Output: [1,2]
 * 
 * 
 * Example 3:
 * 
 * Input: nums = [3,3], target = 6
 * Output: [0,1]
 * 
 * 
 *  
 * Constraints:
 * 
 * 
 * 	2 <= nums.length <= 10^4
 * 	-10^9 <= nums[i] <= 10^9
 * 	-10^9 <= target <= 10^9
 * 	Only one valid answer exists.
 * 
 * 
 *  
 * Follow-up: Can you come up with an algorithm that is less than O(n^2) time complexity?
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let size = nums.len();
        for i in 0..size {
            let j = target - nums[i];
            let index = match nums.iter().position(|&r| r == j) {
                Some(x) => x,
                _ => usize::MAX
            };
            if (index != i) && (index != usize::MAX) {
            ans.push(i as i32);
            ans.push(index as i32);
            break;
            } else {
            continue;
            }
        }
        ans 
    }
}
// @lc code=end
