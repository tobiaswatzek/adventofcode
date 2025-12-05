namespace AoC2025;

public class DayFive
{
    public static async Task Run()
    {
        var freshRanges = new List<(long start, long end)>();
        var ingredients = new List<long>();
        var ingredientReadMode = false;
        await foreach (var line in File.ReadLinesAsync("./inputs/05.txt"))
        {
            if (!ingredientReadMode && string.IsNullOrWhiteSpace(line))
            {
                ingredientReadMode = true;
                continue;
            }

            if (ingredientReadMode)
            {
                ingredients.Add(long.Parse(line));
            }
            else
            {
                var range = line.Split('-') switch
                {
                    [var start, var end] => (start: long.Parse(start),
                        end: long.Parse(end)),
                    _ => throw new InvalidOperationException("Invalid input")
                };
                freshRanges.Add(range);
            }
        }

        SolvePartOne(freshRanges, ingredients);
        SolvePartTwo(freshRanges);
    }


    private static void SolvePartOne(
        List<(long start, long end)> freshRanges, List<long> ingredients)
    {
        var freshCount = ingredients.Count(ingredient =>
            freshRanges.Any(range =>
                ingredient >= range.start && ingredient <= range.end));

        Console.WriteLine($"Part One: {freshCount}");
    }

    private static void SolvePartTwo(List<(long start, long end)> freshRanges)
    {
        var reducedRanges = freshRanges.Aggregate(
            new List<(long start, long end)>(),
            (result, range) =>
            {
                if (result.Any(other =>
                        range.start >= other.start && range.end <= other.end))
                {
                    // there is already a range that contains the current one 
                    return result;
                }


                // remove ranges that are part of the current range
                var filtered = result.Where(other =>
                        !(range.start >= other.start && range.end <= other.end))
                    .ToList();

                var overlapping = filtered.FindAll(other =>
                    range.start <= other.end && other.start <= range.end);
                if (overlapping.Count == 0)
                {
                    filtered.Add(range);
                }
                else
                {
                    var otherStart = overlapping.Min(other => other.start);
                    var otherEnd = overlapping.Max(other => other.end);
                    
                    filtered.RemoveAll(overlapping.Contains);

                    filtered.Add((Math.Min(range.start, otherStart), Math.Max(range.end, otherEnd)));
                }

                return filtered;
            });

        var freshCount = reducedRanges.Aggregate(0L,
            (count, range) => count + (range.end - range.start) + 1);


        Console.WriteLine($"Part Two: {freshCount}");
    }
}