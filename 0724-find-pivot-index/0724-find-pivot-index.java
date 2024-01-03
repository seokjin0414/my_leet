class Solution {
    public int pivotIndex(int[] nums) {
        int r = 0;
        int l = 0;
        for (int n : nums) {
            r += n;
        }
        for (int i = 0; i < nums.length; l += nums[i++]) {
            if (l * 2 == r - nums[i]) {
                return i;
            }
        }
        return -1;
    }
}