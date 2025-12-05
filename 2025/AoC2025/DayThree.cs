namespace AoC2025;

public static class DayThree
{
    public static async Task Run()
    {
        var batteryBanks = await File.ReadLinesAsync("./inputs/03.txt")
            .Select(line =>
                line.Select(c => byte.Parse(c.ToString())).ToArray())
            .ToListAsync();


        SolvePartOne(batteryBanks);
        SolvePartTwo(batteryBanks);
    }

    private static void SolvePartOne(List<byte[]> batteryBanks)
    {
        long totalJoltage = 0;
        foreach (var batteryBank in batteryBanks)
        {
            var first = batteryBank[..^1]
                .Select((battery, index) => (battery, index))
                .MaxBy(t => t.battery);
            var last = batteryBank[(first.index + 1)..].Max();
            var joltage = 10 * first.battery + last;
            totalJoltage += joltage;
        }

        Console.WriteLine($"Part One: {totalJoltage}");
    }

    private static void SolvePartTwo(List<byte[]> batteryBanks)
    {
        long totalJoltage = 0;

        foreach (var batteryBank in batteryBanks)
        {
            var joltage = 0L;
            var start = 0;
            for (var position = 0; position < 12; position++)
            {
                var max = batteryBank[start..^(12 - (position + 1))]
                    .Select((battery, index) => (battery, index: start + index))
                    .MaxBy(t => t.battery);
                joltage = 10 * joltage + max.battery;
                start = max.index + 1;
            }
            
            totalJoltage += joltage;
        }


        Console.WriteLine($"Part Two: {totalJoltage}");
    }
}