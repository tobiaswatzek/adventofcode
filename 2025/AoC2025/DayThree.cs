using System.Diagnostics;

namespace AoC2025;

public static class DayThree
{
    public static async Task Run()
    {
        var batteryBanks = await File.ReadLinesAsync("./inputs/03.txt")
            .Select(line => line.Select(c => int.Parse(c.ToString())).ToArray())
            .ToListAsync();


        SolvePartOne(batteryBanks);
    }

    private static void SolvePartOne(List<int[]> batteryBanks)
    {
        long totalJoltage = 0;
        foreach (var batteryBank in batteryBanks)
        {
            var first = batteryBank[..^1].Select((battery, index) => (battery, index)).MaxBy(t => t.battery);
            var last = batteryBank[(first.index + 1)..].Max();
            var joltage = int.Parse($"{first.battery}{last}");
            Console.WriteLine(joltage);
            totalJoltage += joltage;
        }

        Console.WriteLine($"Part One: {totalJoltage}");
    }
}