using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public class Day10 : IDay
    {
        public int Number { get; } = 10;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var adapters = await ParseInput();
            var firstSolution = SolveFirst(adapters);
            var secondSolution = SolveSecond(adapters);

            return (firstSolution, secondSolution);
        }

        private static string SolveFirst(IReadOnlyCollection<int> adapters)
        {
            var orderedAdapters = adapters.Concat(new[] {0, adapters.Max() + 3}).OrderBy(a => a).ToArray();
            var joltDifferences = new Dictionary<int, int>(3);
            for (int i = 1; i < orderedAdapters.Length; i++)
            {
                var previousAdapter = orderedAdapters[i - 1];
                var currentAdapter = orderedAdapters[i];
                var joltDifference = currentAdapter - previousAdapter;
                joltDifferences[joltDifference] =
                    joltDifferences.TryGetValue(joltDifference, out var count) ? count + 1 : 1;
            }

            joltDifferences.TryGetValue(1, out var oneJoltDifferences);
            joltDifferences.TryGetValue(3, out var threeJoltDifferences);

            return (oneJoltDifferences * threeJoltDifferences).ToString();
        }

        private static string SolveSecond(IReadOnlyCollection<int> adapters)
        {
            var maxJoltage = adapters.Max() + 3;
            var orderedAdapters = adapters.Concat(new[] {0, maxJoltage}).OrderBy(a => a).ToArray();
            var paths = new Dictionary<int, ulong>(orderedAdapters.Length) {[0] = 1};
            foreach (var adapter in orderedAdapters)
            {
                for (int diff = 1; diff < 4; diff++)
                {
                    var nextAdapter = adapter + diff;
                    if (!orderedAdapters.Contains(nextAdapter))
                    {
                        continue;
                    }

                    if (paths.ContainsKey(nextAdapter))
                    {
                        paths[nextAdapter] += paths[adapter];
                    }
                    else
                    {
                        paths[nextAdapter] = paths[adapter];
                    }
                }
            }

            return paths[maxJoltage].ToString();
        }

        private static async Task<IReadOnlyCollection<int>> ParseInput()
        {
            var lines = await File.ReadAllLinesAsync("./input/day10.txt");
            return lines.Select(int.Parse).ToArray();
        }
    }
}
