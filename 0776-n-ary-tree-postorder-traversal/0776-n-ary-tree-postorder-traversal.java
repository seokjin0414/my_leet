class Solution {
    public List<Integer> postorder(Node root) {
        if (root == null) return new ArrayList<>();
        List<Integer> list = new ArrayList<>();
        Stack<Node> s = new Stack<>();
        s.push(root);

        while (!s.isEmpty()) {
            Node n = s.pop();
            list.add(0, n.val);
            for (Node child : n.children) {
                s.push(child);
            }
        }
        return list;
    }
}