impl Solution {
    pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let set: std::collections::HashSet<_> = nums.into_iter().collect();

        while matches!(head.as_ref(), Some(node) if set.contains(&node.val)) {
            head = head.take().unwrap().next;
        }

        let mut current = head.as_mut();
        while let Some(node) = current {
            while matches!(node.next.as_ref(), Some(next) if set.contains(&next.val)) {
                node.next = node.next.take().unwrap().next;
            }
            current = node.next.as_mut();
        }

        head
    }
}