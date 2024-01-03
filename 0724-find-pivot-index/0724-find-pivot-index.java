class Solution {
    public int pivotIndex(int[] nums) {
        int n = 0;
        int l = 0;
        int r = 0;

        if (nums.length == 1) {
            return n;
        } else {
            for (int i = 1; i < nums.length; i++) {
                r += nums[i];
            }
            if (l == r) {
                return n;
            }
            n++;
        }

        while (n < nums.length) {
            l += nums[n - 1];
            r -= nums[n];

            if (l == r) {
                return n;
            }
            n++;
        }
        return -1;
    }
}