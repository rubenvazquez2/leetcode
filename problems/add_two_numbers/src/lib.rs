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

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head1 = &l1;
        let mut head2 = &l2;

        let mut ints: Vec<i32> = vec![];

        let mut quotient = 0;
        loop {
            match (head1, head2) {
                (Some(current1), Some(current2)) => {
                    let sum = current1.val + current2.val + quotient;
                    quotient = sum / 10;
                    let remainder = sum % 10;
                    ints.push(remainder);
                    head1 = &current1.next;
                    head2 = &current2.next;
                }
                (Some(current), None) | (None, Some(current)) => {
                    let sum = current.val + quotient;
                    quotient = sum / 10;
                    let remainder = sum % 10;
                    ints.push(remainder);
                    if head1.is_none() {
                        head2 = &current.next;
                    } else {
                        head1 = &current.next;
                    }
                }
                (None, None) => {
                    if quotient > 0 {
                        ints.push(quotient);
                    }
                    break;
                }
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
        let list1 = Solution::construct_linked_list_from_ints(vec![2, 4, 3]);
        let list2 = Solution::construct_linked_list_from_ints(vec![5, 6, 4]);

        let actual = Solution::add_two_numbers(list1, list2);
        let expected = Solution::construct_linked_list_from_ints(vec![7, 0, 8]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn example_2() {
        let list1 = Solution::construct_linked_list_from_ints(vec![0]);
        let list2 = Solution::construct_linked_list_from_ints(vec![0]);

        let actual = Solution::add_two_numbers(list1, list2);
        let expected = Solution::construct_linked_list_from_ints(vec![0]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn example_3() {
        let list1 = Solution::construct_linked_list_from_ints(vec![9, 9, 9, 9, 9, 9, 9]);
        let list2 = Solution::construct_linked_list_from_ints(vec![9, 9, 9, 9]);

        let actual = Solution::add_two_numbers(list1, list2);
        let expected = Solution::construct_linked_list_from_ints(vec![8, 9, 9, 9, 0, 0, 0, 1]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn example_4() {
        let list1 = Solution::construct_linked_list_from_ints(vec![9]);
        let list2 = Solution::construct_linked_list_from_ints(vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9]);

        let actual = Solution::add_two_numbers(list1, list2);
        let expected =
            Solution::construct_linked_list_from_ints(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

        assert_eq!(actual, expected);
    }
}
