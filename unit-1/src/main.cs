public class Solution {
    public int SearchInsert(int[] nums, int target) {
        
        var left = 0;
        var right = nums.Length - 1;

        while (left <= right)
        {
            var mid = left + (right - left)/2;

            if (nums[mid] == target)
            {
                return mid;
            }

            else if (nums[mid] < target)
            {
                left = mid +1;
            }

            else
            {
               right = mid - 1;
            }
        }

        // Target not found, return the correct insertion position
        return left;
    }
}