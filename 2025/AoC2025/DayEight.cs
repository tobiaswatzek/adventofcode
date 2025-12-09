namespace AoC2025;

public class DayEight
{
    private readonly record struct Point(int X, int Y, int Z)
    {
        public double EucledianDistanceTo(Point other) => Math.Sqrt(
            Math.Pow(X - other.X, 2) + Math.Pow(Y - other.Y, 2) +
            Math.Pow(Z - other.Z, 2));
    }

    public static async Task Run()
    {
        var junctionBoxes = await File.ReadLinesAsync("./inputs/08.txt")
            .Select(l =>
                l.Split(',').Select(int.Parse).ToArray() switch
                {
                    [var x, var y, var z] => new Point(x, y, z),
                    _ => throw new InvalidOperationException(
                        "invalid coordinates")
                }).ToHashSetAsync();


        SolvePartOne(junctionBoxes);
        SolvePartTwo(junctionBoxes);
    }

    private class PointsAndDistance(
        Point a,
        Point b,
        double distance) : IComparable<PointsAndDistance>,
        IEquatable<PointsAndDistance>
    {
        public Point A { get; } = a;
        public Point B { get; } = b;
        public double Distance { get; } = distance;

        public bool Equals(PointsAndDistance? other)
        {
            if (other is null)
            {
                return false;
            }

            if (ReferenceEquals(this, other))
            {
                return true;
            }

            return ((A.Equals(other.A) && B.Equals(other.B)) ||
                    (A.Equals(other.B) && B.Equals(other.A))) &&
                   Distance.Equals(other.Distance);
        }

        public int CompareTo(PointsAndDistance? other) =>
            Distance.CompareTo(other?.Distance);

        public override string ToString()
        {
            return
                $"{nameof(A)}: {A}, {nameof(B)}: {B}, {nameof(Distance)}: {Distance}";
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(A.GetHashCode() + B.GetHashCode(),
                Distance);
        }

        public override bool Equals(object? obj)
        {
            return Equals(obj as PointsAndDistance);
        }
    }

    private static void SolvePartOne(HashSet<Point> junctionBoxes)
    {
        var distances = junctionBoxes.Select(junctionBox => junctionBoxes
                .Where(p => p != junctionBox)
                .Select(other => new PointsAndDistance(junctionBox, other,
                    junctionBox.EucledianDistanceTo(other)))
            )
            .SelectMany(p => p)
            .Distinct()
            .Order();

        var circuits = new List<HashSet<Point>>();
        const int maximum = 1000;
        foreach (var valueTuple in distances.Take(maximum))
        {
            var existingCircuits = circuits.FindAll(c =>
                c.Contains(valueTuple.A) || c.Contains(valueTuple.B));

            var circuit = new HashSet<Point>();

            switch (existingCircuits)
            {
                case []:
                    circuits.Add(circuit);
                    break;
                case [var c]:
                    circuit = c;
                    break;
                case [var c1, var c2]:
                    circuits.Remove(c2);
                    c1.UnionWith(c2);
                    break;
                default:
                    throw new InvalidOperationException(
                        "At most 2 circuits should be found to merge.");
            }

            circuit.Add(valueTuple.A);
            circuit.Add(valueTuple.B);
        }

        var solution = circuits.OrderByDescending(c => c.Count).Take(3)
            .Aggregate(1, (product, c) => product * c.Count);

        Console.WriteLine($"Part One: {solution}");
    }

    private static void SolvePartTwo(HashSet<Point> junctionBoxes)
    {
        var distances = junctionBoxes.Select(junctionBox => junctionBoxes
                .Where(p => p != junctionBox)
                .Select(other => new PointsAndDistance(junctionBox, other,
                    junctionBox.EucledianDistanceTo(other)))
            )
            .SelectMany(p => p)
            .Distinct()
            .Order();

        ulong solution = 0;
        var circuits = new List<HashSet<Point>>();
        foreach (var valueTuple in distances)
        {
            var existingCircuits = circuits.FindAll(c =>
                c.Contains(valueTuple.A) || c.Contains(valueTuple.B));

            var circuit = new HashSet<Point>();

            switch (existingCircuits)
            {
                case []:
                    circuits.Add(circuit);
                    break;
                case [var c]:
                    circuit = c;
                    break;
                case [var c1, var c2]:
                    circuits.Remove(c2);
                    c1.UnionWith(c2);
                    break;
                default:
                    throw new InvalidOperationException(
                        "At most 2 circuits should be found to merge.");
            }

            circuit.Add(valueTuple.A);
            circuit.Add(valueTuple.B);

            if (circuits is [var singleC] && singleC.Count == junctionBoxes.Count)
            {
                solution =  (ulong)valueTuple.A.X * (ulong)valueTuple.B.X;
                break;
            }
        }

        Console.WriteLine($"Part Two: {solution}");
    }
}