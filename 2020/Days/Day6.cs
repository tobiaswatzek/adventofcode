using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public class Day6 : IDay
    {
        public int Number { get; } = 6;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var firstSolution = await SolveFirst();
            var secondSolution = await SolveSecond();
            
            return (firstSolution, secondSolution);
        }

        private static async Task<string> SolveFirst()
        {
            var answerGroups = await ParseAnswers(ParseGroupFirst);

            return answerGroups.Sum(answerGroup => answerGroup.Count).ToString();
        }

        private static async Task<string> SolveSecond()
        {
            var answerGroups = await ParseAnswers(ParseGroupSecond);

            return answerGroups.Sum(answerGroup => answerGroup.Count).ToString();
        }

        private static async Task<IEnumerable<ISet<char>>> ParseAnswers(Func<TextReader, Task<ISet<char>>> parseGroup)
        {
            var answerGroups = new List<ISet<char>>();
            using var reader = File.OpenText("./input/day6.txt");
            while (!reader.EndOfStream)
            {
                var answerGroup = await parseGroup(reader);
                answerGroups.Add(answerGroup);
            }

            return answerGroups;
        }

        private static async Task<ISet<char>> ParseGroupFirst(TextReader reader)
        {
            var answers = new HashSet<char>();
            while (true)
            {
                var line = await reader.ReadLineAsync();
                if (line is (null or ""))
                {
                    break;
                }

                answers.UnionWith(line);
            }

            return answers;
        }

        private static async Task<ISet<char>> ParseGroupSecond(TextReader reader)
        {
            var firstRun = true;
            var answers = new HashSet<char>();
            while (true)
            {
                var line = await reader.ReadLineAsync();
                if (line is (null or ""))
                {
                    break;
                }

                if (firstRun)
                {
                    answers.UnionWith(line);
                    firstRun = false;
                }
                else
                {
                    answers.IntersectWith(line);
                }
            }

            return answers;
        }
    }
}
