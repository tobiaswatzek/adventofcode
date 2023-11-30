using System;
using System.Globalization;
using System.IO;
using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public class Day12 : IDay
    {
        public int Number { get; } = 12;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var firstSolution = await SolveFirst();
            var secondSolution = await SolveSecond();

            return (firstSolution, secondSolution);
        }

        private static async Task<string> SolveFirst()
        {
            var ship = new Ship(Direction.East, 0, 0);
            await RunCommands(ship);
            
            var traveledDistance = ship.ManhattanDistanceTo(0, 0);

            return traveledDistance.ToString();
        }
        
        private static async Task<string> SolveSecond()
        {
            var ship = new ShipWithWaypoint(0, 0, 10, -1);
            await RunCommands(ship);
            
            var traveledDistance = ship.ManhattanDistanceTo(0, 0);

            return traveledDistance.ToString();
        }

        private static async Task RunCommands(IShip ship)
        {
            using var reader = File.OpenText("./input/day12.txt");
            while (!reader.EndOfStream)
            {
                var line = await reader.ReadLineAsync();
                if (line is (null or ""))
                {
                    continue;
                }

                var command = line.Substring(0, 1);
                var value = int.Parse(line.Substring(1), NumberStyles.Integer);

                switch (command)
                {
                    case "N":
                        ship.MoveInDirection(Direction.North, value);
                        break;
                    case "E":
                        ship.MoveInDirection(Direction.East, value);
                        break;
                    case "S":
                        ship.MoveInDirection(Direction.South, value);
                        break;
                    case "W":
                        ship.MoveInDirection(Direction.West, value);
                        break;
                    case "L":
                        ship.TurnLeft(value);
                        break;
                    case "R":
                        ship.TurnRight(value);
                        break;
                    case "F":
                        ship.MoveForward(value);
                        break;
                    default: throw new ArgumentOutOfRangeException(nameof(command), command, "Command not known.");
                }
            }
        }

        private enum Direction
        {
            North = 0,
            East = 90,
            South = 180,
            West = 270
        }

        private interface IShip
        {
            public void MoveInDirection(Direction direction, int value);
            public void MoveForward(int value);
            public void TurnRight(int degrees);
            public void TurnLeft(int degrees);
            public long ManhattanDistanceTo(int x, int y);
        }

        private class Ship : IShip
        {
            private Direction Facing { get; set; }
            private int X { get; set; }
            private int Y { get; set; }

            public Ship(Direction facing, int x, int y)
            {
                Facing = facing;
                X = x;
                Y = y;
            }

            public void MoveInDirection(Direction direction, int value)
            {
                switch (direction)
                {
                    case Direction.North:
                        X = X;
                        Y -= value;
                        break;
                    case Direction.East:
                        X += value;
                        Y = Y;
                        break;
                    case Direction.South:
                        X = X;
                        Y += value;
                        break;
                    case Direction.West:
                        X -= value;
                        Y = Y;
                        break;
                    default:
                        throw new ArgumentOutOfRangeException(nameof(direction), direction, "Direction not known.");
                }
            }

            public void MoveForward(int value)
            {
                MoveInDirection(Facing, value);
            }

            public void TurnRight(int degrees)
            {
                var directionToFace = ((int) Facing + degrees) % 360;
                if (!Enum.IsDefined(typeof(Direction), directionToFace))
                {
                    throw new InvalidOperationException($"Direction with value {directionToFace} is not defined.");
                }

                Facing = (Direction) directionToFace;
            }

            public void TurnLeft(int degrees)
            {
                var directionToFace = (360 + ((int) Facing - degrees)) % 360;
                if (!Enum.IsDefined(typeof(Direction), directionToFace))
                {
                    throw new InvalidOperationException($"Direction with value {directionToFace} is not defined.");
                }

                Facing = (Direction) directionToFace;
            }

            public long ManhattanDistanceTo(int x, int y)
            {
                return Math.Abs(x - X) + Math.Abs(y - Y);
            }
        }

        private class ShipWithWaypoint : IShip
        {
            public ShipWithWaypoint(int shipX, int shipY, int waypointX, int waypointY)
            {
                ShipX = shipX;
                ShipY = shipY;
                Waypoint = (waypointX, waypointY);
            }

            private int ShipX { get; set; }
            private int ShipY { get; set; }
            private (int X, int Y) Waypoint { get; set; }

            public void MoveInDirection(Direction direction, int value)
            {
                Waypoint = direction switch
                {
                    Direction.North => (Waypoint.X, Waypoint.Y - value),
                    Direction.East => (Waypoint.X + value, Waypoint.Y),
                    Direction.South => (Waypoint.X, Waypoint.Y + value),
                    Direction.West => (Waypoint.X - value, Waypoint.Y),
                    _ => throw new ArgumentOutOfRangeException(nameof(direction), direction, "Direction not known.")
                };
            }

            public void MoveForward(int value)
            {
                ShipX += Waypoint.X * value;
                ShipY += Waypoint.Y * value;
            }

            public void TurnRight(int degrees)
            {
                Waypoint = degrees switch
                {
                    90 => (-Waypoint.Y, Waypoint.X),
                    180 => (-Waypoint.X, -Waypoint.Y),
                    270 => (Waypoint.Y, -Waypoint.X),
                    360 => Waypoint,
                    _ => throw new ArgumentOutOfRangeException(nameof(degrees), degrees, "Degree not supported.")
                };
            }

            public void TurnLeft(int degrees)
            {
                Waypoint = degrees switch
                {
                    90 => (Waypoint.Y, -Waypoint.X),
                    180 => (-Waypoint.X, -Waypoint.Y),
                    270 => (-Waypoint.Y, Waypoint.X),
                    360 => Waypoint,
                    _ => throw new ArgumentOutOfRangeException(nameof(degrees), degrees, "Degree not supported.")
                };
            }

            public long ManhattanDistanceTo(int x, int y)
            {
                return Math.Abs(x - ShipX) + Math.Abs(y - ShipY);
            }
        }
    }
}
