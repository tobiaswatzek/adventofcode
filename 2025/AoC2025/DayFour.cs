namespace AoC2025;

public static class DayFour
{
    private const char RemovalMarker = 'x';
    private const char Empty = '.';

    public static async Task Run()
    {
        var grid = await File.ReadLinesAsync("./inputs/04.txt")
            .Select(line => line.ToCharArray())
            .ToArrayAsync();


        SolvePartOne(grid);
        SolvePartTwo(grid);
    }

    private static void SolvePartOne(char[][] grid)
    {
        var height = grid.Length;
        var width = grid[0].Length;

        var removed = 0;

        for (int y = 0; y < height; y++)
        {
            for (int x = 0; x < width; x++)
            {
                if (grid[y][x] == Empty)
                {
                    continue;
                }

                var leftIsEmpty = x == 0 || grid[y][x - 1] == Empty;
                var upperLeftIsEmpty =
                    x == 0 || y == 0 || grid[y - 1][x - 1] == Empty;
                var upperIsEmpty = y == 0 || grid[y - 1][x] == Empty;
                var upperRightIsEmpty =
                    x == width - 1 || y == 0 || grid[y - 1][x + 1] == Empty;
                var rightIsEmpty =
                    x == width - 1 || grid[y][x + 1] == Empty;
                var lowerRightIsEmpty =
                    x == width - 1 || y == height - 1 ||
                    grid[y + 1][x + 1] == Empty;
                var lowerIsEmpty =
                    y == height - 1 || grid[y + 1][x] == Empty;
                var lowerLeftIsEmpty = x == 0 || y == height - 1 ||
                                       grid[y + 1][x - 1] == Empty;

                bool[] emptyPositions =
                [
                    leftIsEmpty, upperLeftIsEmpty, upperIsEmpty,
                    upperRightIsEmpty, rightIsEmpty, lowerRightIsEmpty,
                    lowerIsEmpty, lowerLeftIsEmpty
                ];

                if (emptyPositions.Count(a => a) >= 5)
                {
                    removed++;
                }
            }
        }


        Console.WriteLine($"Part One: {removed}");
    }

    private static void SolvePartTwo(char[][] grid)
    {
        var height = grid.Length;
        var width = grid[0].Length;

        var removed = 0;


        while (true)
        {
            for (int y = 0; y < height; y++)
            {
                for (int x = 0; x < width; x++)
                {
                    if (grid[y][x] == Empty)
                    {
                        continue;
                    }

                    var leftIsEmpty = x == 0 || grid[y][x - 1] == Empty;
                    var upperLeftIsEmpty =
                        x == 0 || y == 0 || grid[y - 1][x - 1] == Empty;
                    var upperIsEmpty = y == 0 || grid[y - 1][x] == Empty;
                    var upperRightIsEmpty =
                        x == width - 1 || y == 0 || grid[y - 1][x + 1] == Empty;
                    var rightIsEmpty =
                        x == width - 1 || grid[y][x + 1] == Empty;
                    var lowerRightIsEmpty =
                        x == width - 1 || y == height - 1 ||
                        grid[y + 1][x + 1] == Empty;
                    var lowerIsEmpty =
                        y == height - 1 || grid[y + 1][x] == Empty;
                    var lowerLeftIsEmpty = x == 0 || y == height - 1 ||
                                           grid[y + 1][x - 1] == Empty;

                    bool[] emptyPositions =
                    [
                        leftIsEmpty, upperLeftIsEmpty, upperIsEmpty,
                        upperRightIsEmpty, rightIsEmpty, lowerRightIsEmpty,
                        lowerIsEmpty, lowerLeftIsEmpty
                    ];

                    if (emptyPositions.Count(a => a) >= 5)
                    {
                        grid[y][x] = RemovalMarker;
                        removed++;
                    }
                }
            }

            if (grid.Any(row => row.Any(c => c == RemovalMarker)))
            {
                grid = grid.Select(row =>
                        row.Select(c => c == RemovalMarker ? Empty : c)
                            .ToArray())
                    .ToArray();
            }
            else
            {
                break;
            }
        }


        Console.WriteLine($"Part Two: {removed}");
    }
}