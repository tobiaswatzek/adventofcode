using System.Collections.Immutable;

namespace AoC2025;

public class DaySeven
{
    public static async Task Run()
    {
        var diagram = (await File.ReadLinesAsync("./inputs/07.txt").Select(l =>
            l
                .Select(c => c switch
                {
                    '.' => Cell.Empty,
                    '^' => Cell.Splitter,
                    'S' => Cell.Start,
                    _ => throw new InvalidOperationException(
                        $"Unknown Cell type {c}")
                }).ToImmutableArray()).ToArrayAsync()).ToImmutableArray();


        SolvePartOne(diagram.Select(r => r.ToArray()).ToArray());
        SolvePartTwo(diagram.Select(r => r.ToArray()).ToArray());
    }

    private static void SolvePartOne(Cell[][] diagram)
    {
        var splitCount = 0L;

        for (int y = 0; y < diagram.Length - 1; y++)
        {
            for (int x = 0; x < diagram[0].Length; x++)
            {
                if (diagram[y][x] is not (Cell.Start or Cell.Beam))
                {
                    continue;
                }

                var downY = y + 1;
                if (diagram[downY][x] == Cell.Splitter)
                {
                    splitCount++;
                    if (x > 0)
                    {
                        diagram[downY][x - 1] = Cell.Beam;
                    }

                    if (x < diagram[0].Length - 1)
                    {
                        diagram[downY][x + 1] = Cell.Beam;
                    }
                }
                else
                {
                    diagram[downY][x] = Cell.Beam;
                }
            }
        }

        Console.WriteLine($"Part One: {splitCount}");
    }

    private static void SolvePartTwo(Cell[][] diagram)
    {
        // var splitCount = 0L;
        // var nodes = new HashSet<(int x, int y)>();
        // for (int y = 0; y < diagram.Length - 1; y++)
        // {
        //     for (int x = 0; x < diagram[0].Length; x++)
        //     {
        //         if (diagram[y][x] is not (Cell.Start or Cell.Beam))
        //         {
        //             continue;
        //         }
        //
        //         if (diagram[y][x] == Cell.Start)
        //         {
        //             nodes.Add((x, y));
        //         }
        //
        //         var downY = y + 1;
        //         if (diagram[downY][x] == Cell.Splitter)
        //         {
        //             splitCount++;
        //             if (x > 0)
        //             {
        //                 nodes.Add((x, downY));
        //                 diagram[downY][x - 1] = Cell.Beam;
        //             }
        //
        //             if (x < diagram[0].Length - 1)
        //             {
        //                 nodes.Add((x, downY));
        //                 diagram[downY][x + 1] = Cell.Beam;
        //             }
        //         }
        //         else
        //         {
        //             diagram[downY][x] = Cell.Beam;
        //         }
        //     }
        // }
        //
        //
        // foreach (var node in nodes)
        // {
        //     Console.WriteLine($"Node: {node}");
        // }
        //
        // Console.WriteLine($"Part Two: {splitCount}");
    }
    
    

    private enum Cell
    {
        Empty,
        Splitter,
        Beam,
        Start
    }
}