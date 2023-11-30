using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public class Day13 : IDay
    {
        public int Number { get; } = 13;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var firstSolution = await SolveFirst();
            var secondSolution = await SolveSecond();
            
            return (firstSolution, secondSolution);
        }

        private static async Task<string> SolveFirst()
        {
            var (timestamp, ids) = await ParseInputForFirst();

            var (id, nextDeparture) = ids
                .Select(id => (id, nextDeparture: (int) (Math.Ceiling(timestamp / (double) id) * id)))
                .OrderBy(t => t.nextDeparture)
                .First();

            var minutesToWait = nextDeparture - timestamp;

            return (id * minutesToWait).ToString();
        }

        private static async Task<string> SolveSecond()
        {
            var idsWithOffset = await ParseInputForSecond();

            return "";
        }


        private static async Task<IEnumerable<(int id, int offset)>> ParseInputForSecond()
        {
            var lines = await File.ReadAllLinesAsync("./input/day13.txt");
            var idsWithOffset = lines[1]
                .Split(",", StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries)
                .Select((c, i) => c == "x" ? (id: -1, offset: i) : (id: int.Parse(c), offset: i))
                .Where(t => t.id != -1)
                .ToArray();

            return idsWithOffset;
        }

        private static async Task<(int timestamp, int[] ids)> ParseInputForFirst()
        {
            var lines = await File.ReadAllLinesAsync("./input/day13.txt");
            var timestamp = int.Parse(lines[0]);
            var ids = lines[1]
                .Replace("x", "")
                .Split(",", StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries)
                .Select(int.Parse)
                .ToArray();

            return (timestamp, ids);
        }
    }
}
