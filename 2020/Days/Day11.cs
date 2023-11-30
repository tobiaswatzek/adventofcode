using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public class Day11 : IDay
    {
        public int Number { get; } = 11;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var stopwatch = new Stopwatch();
            stopwatch.Start();
            var firstSolution = await SolveFirst();
            stopwatch.Stop();
            Console.WriteLine($"First: {stopwatch.Elapsed:c}");
            stopwatch.Restart();
            var secondSolution = await SolveSecond();
            stopwatch.Stop();
            Console.WriteLine($"Second: {stopwatch.Elapsed:c}");

            return (firstSolution, secondSolution);
        }

        private static async Task<string> SolveFirst()
        {
            var waitingArea = await ParseInput((cells, width) => new FirstWaitingArea(cells, width));

            var lastResult = RepeatSeatingUntilUnchanged(waitingArea);

            return lastResult.ToString();
        }


        private static async Task<string> SolveSecond()
        {
            var waitingArea = await ParseInput((cells, width) => new SecondWaitingArea(cells, width));

            var lastResult = RepeatSeatingUntilUnchanged(waitingArea);

            return lastResult.ToString();
        }

        private static int RepeatSeatingUntilUnchanged(WaitingArea waitingArea)
        {
            int lastResult = -1;

            while (lastResult != waitingArea.NumberOfOccupiedSeats)
            {
                lastResult = waitingArea.NumberOfOccupiedSeats;
                waitingArea.SimulateSeatingPattern();
            }

            return lastResult;
        }

        private static async Task<WaitingArea> ParseInput(Func<IEnumerable<Cell>, int, WaitingArea> buildWaitingArea)
        {
            var lines = await File.ReadAllLinesAsync("./input/day11.txt");
            var width = lines[0].Length;
            var cells = lines.SelectMany(line => line.Select(c => c switch
            {
                'L' => Cell.EmptySeat,
                '#' => Cell.OccupiedSeat,
                '.' => Cell.Floor,
                _ => throw new ArgumentOutOfRangeException(nameof(c), c, "Cell type not known.")
            }));

            return buildWaitingArea(cells, width);
        }


        private abstract class WaitingArea
        {
            protected ImmutableArray<Cell> Cells;

            protected readonly int Width;

            public int NumberOfOccupiedSeats => Cells.Count(cell => cell == Cell.OccupiedSeat);

            protected WaitingArea(IEnumerable<Cell> cells, int width)
            {
                Cells = cells.ToImmutableArray();
                Width = width;
            }

            public abstract void SimulateSeatingPattern();

            protected Cell GetFromPoint(Point2D point)
            {
                var index = point.To1DIndex(Width);
                return index < 0 || index >= Cells.Length ? Cell.Abyss : Cells[index];
            }
        }

        private class FirstWaitingArea : WaitingArea
        {
            private static readonly Direction[] Directions = Enum.GetValues<Direction>();

            public FirstWaitingArea(IEnumerable<Cell> cells, int width) : base(cells, width)
            {
            }

            public override void SimulateSeatingPattern()
            {
                Cells = Cells
                    .AsParallel()
                    .AsOrdered()
                    .Select(ProcessCell)
                    .ToImmutableArray();

                Cell ProcessCell(Cell cell, int index)
                {
                    return cell switch
                    {
                        Cell.EmptySeat when GetAdjacentCells(index).All(c => c != Cell.OccupiedSeat) => Cell
                            .OccupiedSeat,
                        Cell.OccupiedSeat when GetAdjacentCells(index).Count(c => c == Cell.OccupiedSeat) >= 4 => Cell
                            .EmptySeat,
                        _ => cell
                    };
                }
            }

            private IEnumerable<Cell> GetAdjacentCells(int index)
            {
                var point = Point2D.From1DIndex(index, Width);
                foreach (var direction in Directions)
                {
                    yield return GetFromPoint(point.InDirection(direction));
                }
            }
        }

        private class SecondWaitingArea : WaitingArea
        {
            private static readonly Direction[] Directions = Enum.GetValues<Direction>();

            public SecondWaitingArea(IEnumerable<Cell> cells, int width) : base(cells, width)
            {
            }

            public override void SimulateSeatingPattern()
            {
                Cells = Cells
                    .AsParallel()
                    .AsOrdered()
                    .Select(ProcessCell)
                    .ToImmutableArray();

                Cell ProcessCell(Cell cell, int index)
                {
                    return cell switch
                    {
                        Cell.EmptySeat when GetVisibleSeats(index).All(c => c != Cell.OccupiedSeat) => Cell
                            .OccupiedSeat,
                        Cell.OccupiedSeat when GetVisibleSeats(index).Count(c => c == Cell.OccupiedSeat) >= 5 => Cell
                            .EmptySeat,
                        _ => cell
                    };
                }
            }


            private IEnumerable<Cell> GetVisibleSeats(int index)
            {
                var point = Point2D.From1DIndex(index, Width);
                foreach (var direction in Directions)
                {
                    yield return FirstNonFloorCellInDirection(direction);
                }

                Cell FirstNonFloorCellInDirection(Direction direction)
                {
                    Point2D currentPoint = point;
                    Cell cell;
                    do
                    {
                        currentPoint = currentPoint.InDirection(direction);
                        cell = GetFromPoint(currentPoint);
                    } while (cell == Cell.Floor);

                    return cell;
                }
            }
        }

        private enum Direction
        {
            North,
            NorthEast,
            East,
            SouthEast,
            South,
            SouthWest,
            West,
            NorthWest,
        }

        private record Point2D(int X, int Y)
        {
            public Point2D InDirection(Direction direction) => direction switch
            {
                Direction.North => new Point2D(X, Y - 1),
                Direction.NorthEast => new Point2D(X + 1, Y - 1),
                Direction.East => new Point2D(X + 1, Y),
                Direction.SouthEast => new Point2D(X + 1, Y + 1),
                Direction.South => new Point2D(X, Y + 1),
                Direction.SouthWest => new Point2D(X - 1, Y + 1),
                Direction.West => new Point2D(X - 1, Y),
                Direction.NorthWest => new Point2D(X - 1, Y - 1),
                _ => throw new ArgumentOutOfRangeException(nameof(direction), direction, "Direction unknown.")
            };

            public int To1DIndex(int width) => X < 0 || Y < 0 || X >= width || Y >= width ? -1 : X + width * Y;

            public static Point2D From1DIndex(int index, int width)
            {
                return new(index % width, index / width);
            }
        }


        private enum Cell
        {
            Abyss,
            Floor,
            EmptySeat,
            OccupiedSeat
        }
    }
}
