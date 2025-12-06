using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace AoC2025;

public class DaySix
{
    public static async Task Run()
    {
        await SolvePartOne();
        await SolvePartTwo();
    }


    private interface Collector
    {
        public void Put(int number);

        public long CurrentTotal { get; }
    }

    private class SumCollector : Collector
    {
        public void Put(int number) => CurrentTotal += number;
        public long CurrentTotal { get; private set; }
    }

    private class ProductCollector : Collector
    {
        public void Put(int number) => CurrentTotal *= number;
        public long CurrentTotal { get; private set; } = 1;
    }

    private static async Task SolvePartOne()
    {
        var collectors = new List<Collector>();

        await foreach (var line in File.ReadLinesAsync("./inputs/06.txt")
                           .Reverse())
        {
            var columns = line.Split(" ").Select(s => s.Trim())
                .Where(s => !string.IsNullOrEmpty(s));

            if (collectors.Count == 0)
            {
                collectors = columns
                    .Select<string, Collector>(s => s switch
                    {
                        "+" => new SumCollector(),
                        "*" => new ProductCollector(),
                        _ => throw new InvalidOperationException(
                            $"Unexpected operator {s}")
                    }).ToList();
            }
            else
            {
                foreach (var (number, collector) in columns
                             .Select(int.Parse).Zip(collectors))
                {
                    collector.Put(number);
                }
            }
        }

        var total = collectors.Sum(c => c.CurrentTotal);
        
        Console.WriteLine($"Part One: {total}");
    }
    

    private static async Task SolvePartTwo()
    {

        var lines = await File.ReadAllLinesAsync("./inputs/06.txt");
        var numbers = new List<long>();
        var totals = new List<long>();
        for (int x = lines[0].Length - 1; x >= 0; x--)
        {
            var currentNumber = 0;
            for (int y = 0; y < lines.Length - 1; y++)
            {
                var c = lines[y][x];
                if (c == ' ')
                {
                    continue;
                }
                currentNumber = currentNumber * 10 + int.Parse(c.ToString());
            }
            numbers.Add(currentNumber);
            var possibleOperator = lines[^1][x];
            if (possibleOperator == ' ')
            {
                continue;
            }

            var aggregated = possibleOperator switch
            {
                '+' => numbers.Sum(),
                '*' => numbers.Aggregate((a, b) => a * b),
                _ => throw new InvalidOperationException(
                    $"Unexpected operator {possibleOperator}")
            };
            totals.Add(aggregated);
            numbers = [];
            // Skip the space
            x--;
        }

        var total = totals.Sum();
        
        Console.WriteLine($"Part Two: {total}");
    }
}