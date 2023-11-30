using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public class Day14 : IDay
    {
        public int Number { get; } = 14;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var firstSolution = await SolveFirst();

            var secondSolution = await SolveSecond();

            return (firstSolution, secondSolution);
        }

        private static async Task<string> SolveFirst()
        {
            var memory = new Dictionary<int, ulong>();
            ulong? orMask = null;
            ulong? andMask = null;
            using var stream = File.OpenText("./input/day14.txt");
            while (!stream.EndOfStream)
            {
                var line = await stream.ReadLineAsync();
                if (line is (null or ""))
                {
                    continue;
                }

                if (line.StartsWith("mask"))
                {
                    var mask = line.Substring(line.LastIndexOf(' ') + 1);
                    orMask = Convert.ToUInt64(mask.Replace("X", "0"), 2);
                    andMask = Convert.ToUInt64(mask.Replace("X", "1"), 2);
                    continue;
                }

                if (!orMask.HasValue || !andMask.HasValue)
                {
                    throw new InvalidOperationException("Mask not set.");
                }

                var splits = line.Split(" = ");
                var address = Convert.ToInt32(splits[0][4..(splits[0].Length - 1)], 10);
                var val = Convert.ToUInt64(splits[1], 10);

                memory[address] = (val | orMask.Value) & andMask.Value;
            }

            var sum = memory.Values.Aggregate<ulong, ulong>(0, (current, value) => current + value);
            return sum.ToString();
        }

        private static async Task<string> SolveSecond()
        {
            var memory = new Dictionary<ulong, ulong>();
            ulong[]? floatingMasks = null;
            ulong? orMask = null;
            using var stream = File.OpenText("./input/day14.txt");
            while (!stream.EndOfStream)
            {
                var line = await stream.ReadLineAsync();
                if (line is (null or ""))
                {
                    continue;
                }

                if (line.StartsWith("mask"))
                {
                    var mask = line.Substring(line.LastIndexOf(' ') + 1);
                    orMask = Convert.ToUInt64(mask.Replace("X", "0"), 2);
                    var indices = AllIndicesOf(mask, 'X')
                        .Select(i => mask.Length - i - 1)
                        .ToArray();

                    floatingMasks = AllSubsets(indices)
                        .Where(s => s.Any())
                        .Select(subset =>
                            subset.Aggregate(0UL, (acc, cur) => acc | 1UL << cur))
                        .ToArray();

                    continue;
                }

                if (!orMask.HasValue  || floatingMasks is null)
                {
                    throw new InvalidOperationException("Mask not set.");
                }

                var splits = line.Split(" = ");
                var address = Convert.ToUInt64(splits[0][4..(splits[0].Length - 1)], 10);
                var val = Convert.ToUInt64(splits[1], 10);

                var maskedAddress = address | orMask.Value;
                memory[maskedAddress] = val;
                
                var floatingMaskedAddresses = floatingMasks.Select(mask => maskedAddress ^ mask);
                foreach (var floatingMaskedAddress in floatingMaskedAddresses)
                {
                    memory[floatingMaskedAddress] = val;
                }
            }

            var sum = memory.Values.Aggregate<ulong, ulong>(0, (current, value) => current + value);
            return sum.ToString();
        }

        private static IEnumerable<IEnumerable<int>> AllSubsets(IReadOnlyCollection<int> numbers)
        {
            for (int i = 0; i < (1 << numbers.Count); i++)
            {
                // (1<<j) is a number with jth bit 1 
                // so when we 'and' them with the 
                // subset number we get which numbers 
                // are present in the subset and which 
                // are not 
                yield return numbers.Where((n, j) => (i & (1 << j)) > 0).ToList();
            }
        }

        private static IEnumerable<int> AllIndicesOf(string s, char c)
        {
            for (int i = 0; i < s.Length; i++)
            {
                if (s[i] == c)
                {
                    yield return i;
                }
            }
        }
    }
}
