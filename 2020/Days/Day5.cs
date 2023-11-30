using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public class Day5 : IDay
    {
        public int Number { get; } = 5;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var lines = await File.ReadAllLinesAsync("./input/day5.txt");
            var seatIds = ParseSeatIds(lines).ToArray();
            var firstSolution = seatIds.Max().ToString();
            var secondSolution = SolveSecond(seatIds).ToString();
            return (firstSolution, secondSolution);
        }

        private int SolveSecond(IEnumerable<int> seatIds)
        {
            var orderedSeatIds = seatIds.OrderBy(id => id).ToArray();
            
            for (int i = 0; i < orderedSeatIds.Length - 1; i++)
            {
                var seatId = orderedSeatIds[i];
                if (orderedSeatIds[i + 1] != seatId + 1)
                {
                    return seatId + 1;
                }
            }

            throw new InvalidOperationException("No seat id found.");
        }

        private IEnumerable<int> ParseSeatIds(IEnumerable<string> lines)
        {
            return lines.Select(line =>
            {
                var row = ParseBinarySpacePartitioning(line[..7], 0, 127, 'F', 'B');

                var column = ParseBinarySpacePartitioning(line[7..], 0, 7, 'L', 'R');

                return row * 8 + column;
            });
        }

        private static int ParseBinarySpacePartitioning(IEnumerable<char> chars,
            int lower,
            int upper,
            char takeLower,
            char takeUpper)
        {
            var tuple = chars.Aggregate((lower, upper),
                (t, c) =>
                {
                    if (takeLower == c)
                    {
                        return (t.lower, (int) Math.Floor((t.lower + t.upper) / 2d));
                    }

                    if (takeUpper == c)
                    {
                        return ((int) Math.Ceiling((t.lower + t.upper) / 2d), t.upper);
                    }

                    throw new InvalidOperationException($"Char '{c}' not known.");
                });

            Debug.Assert(tuple.lower == tuple.upper);

            return tuple.lower;
        }
    }
}
