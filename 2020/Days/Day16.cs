using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public class Day16 : IDay
    {
        public int Number { get; } = 16;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var parsedInput = await ParseInput();

            var firstSolution = SolveFirst(parsedInput);
            var secondSolution = SolveSecond(parsedInput);

            return (firstSolution, secondSolution);
        }

        private static string SolveFirst(ParsedInput parsedInput)
        {
            var sumInvalidValues = parsedInput.NearbyTickets.SelectMany(ticket => ticket.Values)
                .AsParallel()
                .Where(value => parsedInput.Fields.All(field => !field.IsValidForField(value)))
                .Sum();

            return sumInvalidValues.ToString();
        }

        private static string SolveSecond(ParsedInput parsedInput)
        {
            var fields = parsedInput.NearbyTickets
                .AsParallel()
                .Select(ticket => ticket.Values.Select(value =>
                        parsedInput.Fields.Where(field => field.IsValidForField(value))
                            .ToHashSet())
                    .ToArray()
                )
                .Where(possibleFields => possibleFields.All(p => p.Any()))
                .Aggregate((prev, next) =>
                {
                    for (int i = 0; i < prev.Length; i++)
                    {
                        prev[i].IntersectWith(next[i]);
                    }

                    return prev;
                })
                .ToArray();

            var ordered = fields
                .OrderBy(val => val.Count)
                .ToArray();

            var allPrev = new HashSet<Field>();
            for (int i = 1; i < ordered.Length; i++)
            {
                var prev = ordered[i - 1];
                allPrev.UnionWith(prev);

                var curr = ordered[i];
                curr.ExceptWith(allPrev);
            }

            var fieldNames = fields.Select(fieldSet => fieldSet.Single().Name).ToArray();

            var product = 1L;
            for (int i = 0; i < fieldNames.Length; i++)
            {
                var fieldName = fieldNames[i];
                if (fieldName.StartsWith("departure"))
                {
                    product *= parsedInput.MyTicket.Values[i];
                }
            }


            return product.ToString();
        }

        private static async Task<ParsedInput> ParseInput()
        {
            using var stream = File.OpenText("./input/day16.txt");

            var fields = new List<Field>();
            Ticket myTicket;
            var nearbyTickets = new List<Ticket>();
            while (!stream.EndOfStream)
            {
                var line = await stream.ReadLineAsync();
                if (line is (null or ""))
                {
                    break;
                }

                fields.Add(ParseField(line));
            }

            // ignore "your ticket:" line
            await stream.ReadLineAsync();
            var myTicketLine = await stream.ReadLineAsync();
            myTicket = ParseTicket(myTicketLine!);

            // ignore empty line
            await stream.ReadLineAsync();
            //  ignore "nearby tickets:" line
            await stream.ReadLineAsync();

            while (!stream.EndOfStream)
            {
                var line = await stream.ReadLineAsync();
                if (line is (null or ""))
                {
                    break;
                }

                nearbyTickets.Add(ParseTicket(line));
            }

            return new ParsedInput(myTicket, nearbyTickets, fields);
        }

        private static Field ParseField(string line)
        {
            var nameValuesSplit = line.Split(": ", StringSplitOptions.TrimEntries);
            var name = nameValuesSplit[0];
            var valuesSplit = nameValuesSplit[1].Split(" or ", StringSplitOptions.TrimEntries);
            var values = valuesSplit.Select(rawValue =>
                {
                    var rawValueSplit = rawValue.Split("-");

                    return (lower: Convert.ToInt32(rawValueSplit[0], 10),
                        upper: Convert.ToInt32(rawValueSplit[1], 10));
                })
                .ToArray();

            return new Field(name, values);
        }

        private static Ticket ParseTicket(string line)
        {
            var values = line.Split(",")
                .Select(val => Convert.ToInt32(val, 10))
                .ToArray();

            return new Ticket(values);
        }

        private record ParsedInput(Ticket MyTicket,
            IReadOnlyCollection<Ticket> NearbyTickets,
            IReadOnlyList<Field> Fields);


        private record Field(string Name, IReadOnlyCollection<(int lower, int upper)> Ranges)
        {
            public bool IsValidForField(int value) => Ranges.Any(range => value >= range.lower && value <= range.upper);
        }


        private record Ticket(IReadOnlyList<int> Values);
    }
}
