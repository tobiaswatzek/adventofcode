using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using adventofcode2020.Extensions;

namespace adventofcode2020.Days
{
    public class Day2 : IDay
    {
        private static readonly Regex PolicyRegex = new(
            "(?<first>\\d+)-(?<second>\\d+) (?<letter>[a-z]): (?<password>[a-z]+)",
            RegexOptions.Compiled);

        public int Number { get; } = 2;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var lines = await File.ReadAllLinesAsync("./input/day2.txt");

            var data = lines.ToList();

            var first = SolveFirst(data).ToString();
            var second = SolveSecond(data).ToString();

            return (first, second);
        }

        private static (IPasswordPolicy, string) ParseLine(string line,
            Func<char, int, int, IPasswordPolicy> policyBuilder)
        {
            var match = PolicyRegex.Match(line);
            if (!match.Success)
            {
                throw new Exception($"Line could not be parsed {line}.");
            }

            var letter = match.Groups["letter"].Value[0];
            var first = int.Parse(match.Groups["first"].Value);
            var second = int.Parse(match.Groups["second"].Value);
            var password = match.Groups["password"].Value;

            return (policyBuilder(letter, first, second), password);
        }

        private static int SolveFirst(IEnumerable<string> data)
        {
            return data.Select(l => ParseLine(l, (letter, min, max) => new MinMaxPasswordPolicy(letter, min, max)))
                .Count(t => t.Item1.IsValid(t.Item2));
        }

        private static int SolveSecond(IEnumerable<string> data)
        {
            return data.Select(l =>
                    ParseLine(l, (letter, min, max) => new ExactlyOnePositionPasswordPolicy(letter, min, max)))
                .Count(t => t.Item1.IsValid(t.Item2));
        }
    }
    
    public interface IPasswordPolicy
    {
        public bool IsValid(string password);
    }

    public record MinMaxPasswordPolicy(char Letter, int Min, int Max) : IPasswordPolicy
    {
        public bool IsValid(string password) => password.Count(c => c == Letter).IsBetween(Min, Max);
    }

    public record ExactlyOnePositionPasswordPolicy(char Letter, int First, int Second) : IPasswordPolicy
    {
        public bool IsValid(string password) => password.ElementAtOrDefault(First - 1) == Letter ^
                                                password.ElementAtOrDefault(Second - 1) == Letter;
    }

}
