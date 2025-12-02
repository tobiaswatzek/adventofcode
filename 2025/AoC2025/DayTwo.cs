using System.Diagnostics;

namespace AoC2025;

public static class DayTwo
{
    public static async Task Run()
    {
        var line = await File.ReadLinesAsync("./inputs/02.txt").SingleAsync();
        var ranges = line.Split(',').Select(sequence =>
            sequence.Split('-') switch
            {
                [var from, var to] => (from: long.Parse(from),
                    to: long.Parse(to)),
                _ => throw new InvalidOperationException("Invalid format")
            }
        ).ToList();


        SolvePartOne(ranges);
        SolvePartTwo(ranges);
    }

    private static void SolvePartOne(List<(long from, long to)> ranges)
    {
        long solution = 0;
        foreach (var (from, to) in ranges)
        {
            for (var number = from; number <= to; number++)
            {
                var strNumber = number.ToString();
                if (strNumber.Length % 2 != 0)
                {
                    continue;
                }

                var firstHalf = strNumber[..(strNumber.Length / 2)];
                var secondHalf = strNumber[(strNumber.Length / 2)..];
                if (firstHalf == secondHalf)
                {
                    solution += number;
                }
            }
        }

        Console.WriteLine($"Part One: {solution}");
    }

    private static void SolvePartTwo(List<(long from, long to)> ranges)
    {
        long solution = 0;
        foreach (var (from, to) in ranges)
        {
            for (var number = from; number <= to; number++)
            {
                var strNumber = number.ToString();
                for (int divisor = strNumber.Length; divisor > 1; divisor--)
                {
                    if (strNumber.Length % divisor != 0)
                    {
                        continue;
                    }

                    var set = strNumber.Chunk(strNumber.Length / divisor).Select(chunk => new string(chunk))
                        .ToHashSet();


                    if (set.Count == 1)
                    {
                        solution += number;
                        break;
                    }
                }
            }
        }

        Console.WriteLine($"Part Two: {solution}");
    }
}