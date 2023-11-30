using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public class Day3 : IDay
    {
        public int Number { get; } = 3;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var lines = await File.ReadAllLinesAsync("./input/day3.txt");

            var map = InfiniteWideMap.FromLines(lines);
            
            var first = SolveFirst(map);
            var second = SolveSecond(map);

            return (first, second);
        }

        private static string SolveFirst(InfiniteWideMap map)
        {
            return CountTrees(map, 3, 1).ToString();
        }

        private static string SolveSecond(InfiniteWideMap map)
        {
            var treeCounts = new[] {
                 CountTrees(map, 1, 1),
                 CountTrees(map, 3, 1),
                 CountTrees(map, 5, 1),
                 CountTrees(map, 7, 1),
                 CountTrees(map, 1, 2)
            };
            
            return treeCounts.Aggregate((ulong)1, (acc, i) => acc * i).ToString();
        }

        private static ulong CountTrees(InfiniteWideMap map, int stepsX, int stepsY)
        {
            var position = (x: 0, y: 0);
            FieldType? fieldType;
            ulong treeCount = 0;
            var stopwatch = new Stopwatch();
            stopwatch.Start();
            do
            {
                if (stopwatch.ElapsedMilliseconds > 5000)
                {
                    throw new TimeoutException("Map not traversed in 5 seconds.");
                }
                
                fieldType = map.GetFieldAt(position.x, position.y);
                if (fieldType == FieldType.Tree)
                {
                    treeCount++;
                }

                position = (position.x + stepsX, position.y + stepsY);
            } while (fieldType is not FieldType.Abyss);
            stopwatch.Stop();
            
            return treeCount;
        }
    }
    
    public enum FieldType
    {
        Abyss,
        OpenSquare,
        Tree
    }

    public record InfiniteWideMap
    {
        private readonly ImmutableArray<FieldType> fields;
        public int Width { get; }
        public int Height { get; }

        private InfiniteWideMap(IEnumerable<FieldType> fields, int width, int height)
        {
            this.fields = fields.ToImmutableArray();
            Width = width;
            Height = height;
        }

        public static InfiniteWideMap FromLines(IEnumerable<string> lines)
        {
            var linesArray = lines.ToImmutableArray();
            
            var width = linesArray.FirstOrDefault()?.Length ?? throw new InvalidOperationException("No fields read.");
            var height = linesArray.Length;
            
            var fields = linesArray.SelectMany(line => line.Select(c =>
            {
                return c switch
                {
                    '.' => FieldType.OpenSquare,
                    '#' => FieldType.Tree,
                    _ => throw new InvalidOperationException("Input corrupt.")
                };
            }));

            return new InfiniteWideMap(fields, width, height);
        }
        
        public FieldType GetFieldAt(int x, int y)
        {
            if (y >= Height || y < 0)
            {
                return FieldType.Abyss;
            }
            
            return fields[Width * y + (x % Width)];
        }
    } 
}
