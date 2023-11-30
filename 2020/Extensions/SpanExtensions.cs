using System;

namespace adventofcode2020.Extensions
{
    public static class SpanExtensions
    {
        public static long Sum(this ReadOnlySpan<long> source)
        {
            long sum = 0;
            checked
            {
                foreach (var num in source)
                {
                    sum += num;
                }
            }

            return sum;
        }

        public static long Max(this ReadOnlySpan<long> source)
        {
            var max = long.MinValue;
            foreach (var num in source)
            {
                if (num > max)
                {
                    max = num;
                }
            }

            return max;
        }

        public static long Min(this ReadOnlySpan<long> source)
        {
            var min = long.MaxValue;
            foreach (var num in source)
            {
                if (num < min)
                {
                    min = num;
                }
            }

            return min;
        }
    }
}
