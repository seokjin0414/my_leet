class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        numMap = {}
        n = len(nums)
    
        for i in range(n):
            numMap[nums[i]] = i

        for i in range(n):
            x = target - nums[i]
            if x in numMap and numMap[x] != i:
                return [i, numMap[x]]

        return []