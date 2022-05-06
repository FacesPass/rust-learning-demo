use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution {}

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    // 前序遍历递归实现
    pub fn preorder_traversal_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(node) = root {
                res.push(node.borrow().val);
                dfs(node.borrow_mut().left.take(), res);
                dfs(node.borrow_mut().right.take(), res);
            }
        }

        dfs(root, &mut res);

        res
    }

    // 前序遍历非递归实现
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];
        let mut cur = root;

        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                res.push(cur.clone().unwrap().borrow().val);
                stack.push(cur.clone().unwrap());
                cur = cur.clone().unwrap().borrow_mut().left.take();
            }

            cur = stack.pop();
            cur = cur.unwrap().borrow_mut().right.take();
        }

        res
    }

    // 中序遍历递归实现
    pub fn inorder_traversal_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(node) = root {
                dfs(node.borrow_mut().left.take(), res);
                res.push(node.borrow().val);
                dfs(node.borrow_mut().right.take(), res)
            }
        }

        dfs(root, &mut res);

        res
    }

    // 中序遍历非递归实现
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack = Vec::new();
        let mut cur = root;

        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                stack.push(cur.clone().unwrap());
                cur = cur.clone().unwrap().borrow_mut().left.take();
            }

            cur = stack.pop();
            res.push(cur.clone().unwrap().borrow().val);
            cur = cur.clone().unwrap().borrow_mut().right.take();
        }

        res
    }

    // 后序遍历递归实现
    pub fn postorder_traversal_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(node) = root {
                dfs(node.borrow_mut().left.take(), res);
                dfs(node.borrow_mut().right.take(), res);
                res.push(node.borrow().val);
            }
        }

        dfs(root, &mut res);

        res
    }

    // 后序遍历非递归实现
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: VecDeque<i32> = VecDeque::new();
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root.clone()];

        while !stack.is_empty() {
            if let Some(cur) = stack.pop().unwrap() {
                res.push_front(cur.clone().borrow().val);

                if cur.clone().borrow().left.is_some() {
                    stack.push(cur.clone().borrow_mut().left.take());
                }

                if cur.clone().borrow().right.is_some() {
                    stack.push(cur.clone().borrow_mut().right.take());
                }
            }
        }

        res.into_iter().collect()
    }
}

use std::collections::HashMap;
struct Solution2 {}

// 输入: nums = [2,7,11,15], target = 9
// 输出：[0,1]

impl Solution2 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let val = target - num;
            match map.get(&val) {
                Some(v) => {
                    return vec![*v as i32, i as i32];
                }
                None => {
                    map.insert(num, i);
                }
            }
        }

        vec![]
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut cur = head;

        while let Some(mut node) = cur {
            cur = node.next.take();
            node.next = pre;
            pre = Some(node);
        }

        pre
    }
}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut window: HashMap<char, i32> = HashMap::new();
        let (mut left, mut right, mut res) = (0, 0, 0);

        let chars: Vec<char> = s.chars().collect();

        while right < chars.len() {
            let right_char = chars[right];

            right += 1;

            match window.contains_key(&right_char) {
                true => {
                    let char_num = *window.get(&right_char).unwrap();
                    window.insert(right_char, char_num + 1)
                }
                false => window.insert(right_char, 1),
            };

            while *window.get(&right_char).unwrap() > 1 {
                let left_char = chars[left];
                left += 1;

                let char_num = *window.get(&left_char).unwrap();
                window.insert(left_char, char_num - 1);
            }

            let len: i32 = (right - left) as i32;
            if len > res {
                res = len;
            }
        }

        res
    }

    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut max: i32 = -1;
        let mut map = HashMap::new();
        let chars: Vec<(usize, char)> = s.chars().enumerate().collect();

        for (i, v) in chars {
            if !map.contains_key(&v) {
                map.insert(v, i);
            } else {
                max = max.max((i - map.get(&v).unwrap() - 1) as i32);
            }
        }

        max
    }
}

fn main() {
    let solution = Solution::length_of_longest_substring(String::from("tmmzuxt"));
    println!("len: {}", solution);
}
