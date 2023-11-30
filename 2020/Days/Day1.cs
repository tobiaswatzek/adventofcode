using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public class Day1 : IDay
    {
        public int Number { get; } = 1;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var lines = await File.ReadAllLinesAsync("./input/day1.txt");

            var numbers = lines.Select(int.Parse).ToList();

            var first = SolveFirst(numbers);
            var second = SolveSecond(numbers);

            return (first, second);
        }

        private static string SolveFirst(IReadOnlyCollection<int> numbers)
        {
            var tuple = numbers.SelectMany(x => numbers.Select(y => (x: x, y: y)).Where(t => (t.x + t.y) == 2020))
                .FirstOrDefault();
            var result = tuple.x * tuple.y;

            return result.ToString();
        }

        private static string SolveSecond(IReadOnlyCollection<int> numbers)
        {
            var tuple = numbers.SelectMany(x => numbers
                    .SelectMany(y => numbers.Select(z => (x: x, y: y, z: z)))
                    .Where(t => (t.x + t.y + t.z) == 2020))
                .FirstOrDefault();
            var result = tuple.x * tuple.y * tuple.z;

            return result.ToString();
        }
    }
}
