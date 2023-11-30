using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public class Day15 : IDay
    {
        public int Number { get; } = 15;

        public Task<(string firstSolution, string secondSolution)> Solve()
        {
            var firstSolution = SolveFirst().ToString();
            var secondSolution = SolveSecond().ToString();

            return Task.FromResult((firstSolution, secondSolution));
        }

        private static int SolveFirst()
        {
            var startingNumbers = new[] {5, 1, 9, 18, 13, 8, 0};

            var number = SolveMemoryGame(startingNumbers, 2020);

            return number;
        }

        private static int SolveSecond()
        {
            var startingNumbers = new[] {5, 1, 9, 18, 13, 8, 0};

            var number = SolveMemoryGame(startingNumbers, 300_000_00);

            return number;
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        private static int SolveMemoryGame(IReadOnlyList<int> startingNumbers, int untilNumber)
        {
            var indices = new Dictionary<int, (int last, int beforeLast)>();
            for (var i = 0; i < startingNumbers.Count; i++)
            {
                var num = startingNumbers[i];
                AddIndex(indices, num, i);
            }

            var previousNumber = startingNumbers[^1];

            for (var i = startingNumbers.Count; i < untilNumber; i++)
            {
                var number = 0;
                if (indices.TryGetValue(previousNumber, out var indicesOfPrev) &&
                    indicesOfPrev.beforeLast != -1)
                {
                    number = indicesOfPrev.last - indicesOfPrev.beforeLast;
                }

                previousNumber = number;
                AddIndex(indices, number, i);
            }

            return previousNumber;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private static void AddIndex(IDictionary<int, (int last, int beforeLast)> indices, int num, int index)
        {
            if (indices.ContainsKey(num))
            {
                var current = indices[num];
                indices[num] = (index, current.last);
            }
            else
            {
                indices[num] = (index, -1);
            }
        }
    }
}
