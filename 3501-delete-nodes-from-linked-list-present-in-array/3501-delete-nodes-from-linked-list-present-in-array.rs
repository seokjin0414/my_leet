impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        std::iter::successors(head.as_deref(), |node| node.next.as_deref())
            .scan(
                nums.into_iter().collect::<std::collections::HashSet<_>>(),
                |f, n| Some((!f.contains(&n.val)).then_some(n.val)),
            )
            .flatten()
            .collect::<Vec<_>>()
            .into_iter()
            .rfold(None, |next, val| Some(Box::new(ListNode { val, next })))
    }
}