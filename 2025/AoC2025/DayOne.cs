namespace AoC2025;

public class DayOne
{
    public static async Task Run()
    {
        var lines = await File.ReadAllLinesAsync("./inputs/01.txt");
        var instructions = lines.Select(line =>
        {
            var direction = line[0] == 'L' ? Direction.Left : Direction.Right;
            var steps = int.Parse(line[1..]);
            return (direction, steps);
        }).ToList();

        SolvePartOne(instructions);
        SolvePartTwo(instructions);
    }


    private static void SolvePartOne(
        List<(Direction direction, int steps)> valueTuples)
    {
        var number = 50;
        var zeroCounter = 0;
        foreach (var (direction, steps) in valueTuples)
        {
            if (direction == Direction.Left)
            {
                number -= steps;
            }
            else
            {
                number += steps;
            }

            number %= 100;

            if (number == 0)
            {
                zeroCounter++;
            }
        }

        Console.WriteLine($"Part One: {zeroCounter}");
    }

    private static void SolvePartTwo(
        List<(Direction direction, int steps)> valueTuples)
    {
        var number = 50;
        var zeroCounter = 0;
        foreach (var (direction, steps) in valueTuples)
        {
            for (int i = 0; i < steps; i++)
            {
                if (direction == Direction.Left)
                {
                    number--;
                }
                else
                {
                    number++;
                }

                if (number % 100 == 0)
                {
                    zeroCounter++;
                }
            }

            number %= 100;
        }

        Console.WriteLine($"Part Two: {zeroCounter}");
    }


    private enum Direction
    {
        Left,
        Right
    }
}