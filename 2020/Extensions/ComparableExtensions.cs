using System;
using System.Collections.Generic;

namespace adventofcode2020.Extensions
{
    public static class ComparableExtensions
    {
        public static bool IsBetween<T>(this T item, T start, T end) where T : IComparable, IComparable<T>
        {
            return Comparer<T>.Default.Compare(item, start) >= 0 && Comparer<T>.Default.Compare(item, end) <= 0;
        }
    }
}
