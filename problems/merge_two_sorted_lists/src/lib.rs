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

pub struct Solution {}

impl Solution {
    pub fn construct_linked_list_from_ints(ints: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut last = &mut head;

        for int in ints {
            match last {
                None => *last = Some(Box::new(ListNode::new(int))),
                Some(current) => {
                    current.next = Some(Box::new(ListNode::new(int)));
                    last = &mut current.next;
                }
            }
        }
        head
    }

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ints: Vec<i32> = vec![];

        let mut head1 = &list1;
        let mut head2 = &list2;

        loop {
            match (head1, head2) {
                (Some(next1), Some(next2)) => {
                    if next1.val < next2.val {
                        ints.push(next1.val);
                        head1 = &next1.next;
                    } else {
                        ints.push(next2.val);
                        head2 = &next2.next;
                    }
                }
                (Some(next1), None) => {
                    ints.push(next1.val);
                    head1 = &next1.next;
                }
                (None, Some(next2)) => {
                    ints.push(next2.val);
                    head2 = &next2.next;
                }
                (None, None) => break,
            }
        }

        Solution::construct_linked_list_from_ints(ints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let list1 = Solution::construct_linked_list_from_ints(vec![1, 2, 4]);
        let list2 = Solution::construct_linked_list_from_ints(vec![1, 3, 4]);

        let actual = Solution::merge_two_lists(list1, list2);
        let expected = Solution::construct_linked_list_from_ints(vec![1, 1, 2, 3, 4, 4]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn example_2() {
        let list1 = Solution::construct_linked_list_from_ints(vec![]);
        let list2 = Solution::construct_linked_list_from_ints(vec![]);

        let actual = Solution::merge_two_lists(list1, list2);
        let expected = Solution::construct_linked_list_from_ints(vec![]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn example_3() {
        let list1 = Solution::construct_linked_list_from_ints(vec![]);
        let list2 = Solution::construct_linked_list_from_ints(vec![0]);

        let actual = Solution::merge_two_lists(list1, list2);
        let expected = Solution::construct_linked_list_from_ints(vec![0]);

        assert_eq!(actual, expected);
    }
}
