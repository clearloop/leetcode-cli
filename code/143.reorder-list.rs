// Category: algorithms
// Level: Medium
// Percent: 49.535244%


/*
 * You are given the head of a singly linked-list. The list can be represented as:
 * 
 * L_0 → L_1 → … → L_n - 1 → L_n
 * 
 * 
 * Reorder the list to be on the following form:
 * 
 * L_0 → L_n → L_1 → L_n - 1 → L_2 → L_n - 2 → …
 * 
 * 
 * You may not modify the values in the list's nodes. Only nodes themselves may be changed.
 * 
 *  
 * Example 1:
 * 
 * Input: head = [1,2,3,4]
 * Output: [1,4,2,3]
 * 
 * 
 * Example 2:
 * 
 * Input: head = [1,2,3,4,5]
 * Output: [1,5,2,4,3]
 * 
 * 
 *  
 * Constraints:
 * 
 * 
 * 	The number of nodes in the list is in the range [1, 5 * 10^4].
 * 	1 <= Node.val <= 1000
 * 
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        
        
    }
}
// @lc code=end
