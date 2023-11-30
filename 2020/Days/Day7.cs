using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public class Day7 : IDay
    {
        public int Number { get; } = 7;
        private const string MyBag = "shiny gold";

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var regulations = await ParseBagRegulations();

            var firstSolution = SolveFirst(regulations);
            var secondSolution = SolveSecond(regulations);

            return (firstSolution, secondSolution);
        }


        private static string SolveFirst(IDictionary<string, Node> regulations)
        {
            return regulations.Count(r => HasDescendent(regulations, r.Value)).ToString();

            static bool HasDescendent(IDictionary<string, Node> allNodes, Node node)
                => node.Children.Any(n => n.Name == MyBag || HasDescendent(allNodes, allNodes[n.Name]));
        }


        private static string SolveSecond(IDictionary<string, Node> regulations)
        {
            return (CountInnerBags(regulations, regulations[MyBag]) - 1).ToString();
            
            static long CountInnerBags(IDictionary<string, Node> allNodes, Node node) =>
                node.Children.Aggregate(1L, (acc, cur) => acc += cur.Value.Count * CountInnerBags(allNodes, allNodes[cur.Name]));
        }

        private static async Task<IDictionary<string, Node>> ParseBagRegulations()
        {
            var regulations = new Dictionary<Bag, IEnumerable<Bag>>();
            using (var reader = File.OpenText("./input/day7.txt"))
            {
                while (!reader.EndOfStream)
                {
                    var line = await reader.ReadLineAsync();
                    if (line is (null or ""))
                    {
                        continue;
                    }

                    var regulation = ParseBagRegulation(line);
                    regulations.Add(regulation.from, regulation.children);
                }
            }

            var nodes = regulations.Keys.Select(s => new Node(s.Color, s)).ToList();
            foreach (var node in nodes)
            {
                var children = regulations[node.Value];
                foreach (var childValue in children)
                {
                    var childNode = new Node(childValue.Color, childValue);
                    node.AddChild(childNode);
                }
            }

            return nodes.ToDictionary(n => n.Name, n => n);
        }

        private static (Bag from, IEnumerable<Bag> children) ParseBagRegulation(string line)
        {
            var lineRegex = new Regex("^(?<container>[a-z ]+) bags contain (?<children>[a-z0-9, ]+)\\.$",
                RegexOptions.IgnoreCase);

            var match = lineRegex.Match(line);
            if (!match.Success)
            {
                throw new ArgumentException($"Line '{line}' not matching regex", nameof(line));
            }

            var containerColor = match.Groups["container"].Value;
            var containerBag = new Bag(containerColor, 1);
            var rawChildren = match.Groups["children"].Value;

            if (rawChildren == "no other bags")
            {
                return (containerBag, Enumerable.Empty<Bag>());
            }


            var childRegex = new Regex("(?<count>\\d+) (?<color>[a-z ]+) bags?",
                RegexOptions.IgnoreCase | RegexOptions.Compiled);
            var children = rawChildren
                .Split(",", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
                .Select(rawChild =>
                {
                    var childMatch = childRegex.Match(rawChild);
                    if (!childMatch.Success)
                    {
                        throw new InvalidOperationException($"Child match with '{rawChild}' not possible.");
                    }

                    var count = int.Parse(childMatch.Groups["count"].Value);
                    var color = childMatch.Groups["color"].Value;

                    return new Bag(color, count);
                })
                .ToList();

            return (containerBag, children);
        }
    }

    public record Bag(string Color, int Count);

    public record Node(string Name, Bag Value)
    {
        private readonly List<Node> children = new();

        public IEnumerable<Node> Children => children;

        public void AddChild(Node child)
        {
            children.Add(child);
        }
    }
}
