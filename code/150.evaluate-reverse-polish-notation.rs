// Category: algorithms
// Level: Medium
// Percent: 43.61331%


/*
 * Evaluate the value of an arithmetic expression in Reverse Polish Notation.
 * 
 * Valid operators are +, -, *, and /. Each operand may be an integer or another expression.
 * 
 * Note that division between two integers should truncate toward zero.
 * 
 * It is guaranteed that the given RPN expression is always valid. That means the expression would always evaluate to a result, and there will not be any division by zero operation.
 * 
 *  
 * Example 1:
 * 
 * Input: tokens = ["2","1","+","3","*"]
 * Output: 9
 * Explanation: ((2 + 1) * 3) = 9
 * 
 * 
 * Example 2:
 * 
 * Input: tokens = ["4","13","5","/","+"]
 * Output: 6
 * Explanation: (4 + (13 / 5)) = 6
 * 
 * 
 * Example 3:
 * 
 * Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
 * Output: 22
 * Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
 * = ((10 * (6 / (12 * -11))) + 17) + 5
 * = ((10 * (6 / -132)) + 17) + 5
 * = ((10 * 0) + 17) + 5
 * = (0 + 17) + 5
 * = 17 + 5
 * = 22
 * 
 * 
 *  
 * Constraints:
 * 
 * 
 * 	1 <= tokens.length <= 10^4
 * 	tokens[i] is either an operator: "+", "-", "*", or "/", or an integer in the range [-200, 200].
 * 
 */

// @lc code=start
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        use std::collections::VecDeque;
        let mut ss = VecDeque::new();
        for i in tokens {
            if i=="+" || i == "-" || i=="*" || i=="/" {
                let op: Vec<char> = i.chars().collect();
                let op1 = *ss.back().unwrap(); ss.pop_back();
                let op2 = *ss.back().unwrap(); ss.pop_back();
                let ans = match op[0] {
                    '+' => op1+op2,
                    '-' => op2-op1,
                    '/' => op2/op1,
                    '*' => op1*op2,
                    _ => 0,
                };
                ss.push_back(ans);
            }
            
            else {
                ss.push_back(i.parse().unwrap());
            }
        }
        *ss.back().unwrap() + 100
    }
}
// @lc code=end
