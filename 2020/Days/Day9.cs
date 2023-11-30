using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.IO;
using System.Linq;
using System.Numerics;
using System.Threading.Tasks;
using adventofcode2020.Extensions;

namespace adventofcode2020.Days
{
    public class Day9 : IDay
    {
        public int Number { get; } = 9;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var input = await ParseInput();
            var firstSolution = SolveFirst(input);
            var secondSolution = SolveSecond(input);

            return (firstSolution, secondSolution);
        }

        private static string SolveFirst(IReadOnlyList<long> numbers)
        {
            return FindFirstWeakness(numbers).ToString();
        }

        private static string SolveSecond(IReadOnlyList<long> numbers)
        {
            var firstWeakness = FindFirstWeakness(numbers);
            var numberArray = numbers.ToArray();
            
            for (int offset = 0; offset < numberArray.Length - 1; offset++)
            {
                for (int subsetSize = 2; subsetSize < (numberArray.Length - offset); subsetSize++)
                {
                    var subset = new ReadOnlySpan<long>(numberArray, offset, subsetSize);
                    var sum = subset.Sum();

                    if (sum == firstWeakness)
                    {
                        return (subset.Min() + subset.Max()).ToString();
                    }
                }
            }

            throw new InvalidOperationException("No solution found.");
        }

        private static long FindFirstWeakness(IReadOnlyList<long> numbers)
        {
            var queue = new Queue<long>(25);
            var i = 0;
            for (; i < 25; i++)
            {
                queue.Enqueue(numbers[i]);
            }

            for (; i < numbers.Count; i++)
            {
                var currentNumber = numbers[i];
                if (!IsSumOfAnyTwo(currentNumber, queue.ToArray()))
                {
                    return currentNumber;
                }

                queue.Dequeue();
                queue.Enqueue(currentNumber);
            }

            throw new InvalidOperationException("No solution found.");
        }

        private static bool IsSumOfAnyTwo(long number, IReadOnlyList<long> numbers)
        {
            for (var i = 0; i < numbers.Count; i++)
            {
                var a = numbers[i];
                for (var j = 0; j < numbers.Count; j++)
                {
                    if (i == j)
                    {
                        continue;
                    }

                    var b = numbers[j];
                    if (a + b == number)
                    {
                        return true;
                    }
                }
            }

            return false;
        }

        private static async Task<IReadOnlyList<long>> ParseInput()
        {
            var lines = await File.ReadAllLinesAsync("./input/day9.txt");
            return lines.Select(long.Parse).ToList();
        }
    }
}
