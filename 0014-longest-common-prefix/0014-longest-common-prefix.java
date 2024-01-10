class Solution {
    public String longestCommonPrefix(String[] v) {
        StringBuilder sb = new StringBuilder();
        Arrays.sort(v);
        String a = v[0];
        String b = v[v.length-1];
        
        for (int i = 0; i < Math.min(a.length(), b.length()); i++) {
            if (a.charAt(i) != b.charAt(i)) {
                return sb.toString();
            }
            sb.append(a.charAt(i));
        }
        
        return sb.toString();
    }
}