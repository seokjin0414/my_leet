class Solution {
    public boolean isPalindrome(int x) {
        String n = Integer.toString(x);
        StringBuilder sb = new StringBuilder(n);
        sb.reverse();
        
        if(n.equals(sb.toString())){
            return true;
        }
        
        return false;
    }
}