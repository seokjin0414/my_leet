class Solution {
    public List<Integer> postorder(Node root) {
        if (root == null) {
            return new ArrayList<>();
        }

        List<Integer> list = new ArrayList<>();
        dfs(root, list);
        return list;
    }

    private void dfs(Node root, List<Integer> l) {
        for (Node c : root.children) {
            dfs(c, l);
        }
        l.add(root.val);
    }
}