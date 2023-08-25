namespace unit_1;

public class Solution1
{
    public int MySqrt(int x)
    {

        if (x <= 1)
        {
            return x;
        }

        var left = 1;
        var right = x;

        while (left <= right)
        {
            var mid = left + (right - left) / 2;
            var sqrt = x / mid;

            if (sqrt == mid)
            {
                return mid;
            }

            else if (sqrt < mid)
            {
                right = mid - 1;
            }

            else
            {
                left = mid + 1;
            }

        }

        // Target not found, return the bottom bound
        return right;

    }
}