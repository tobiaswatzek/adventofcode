namespace AoC2025;

public class DayNine
{

    public static async Task Run()
    {
        var points = await File.ReadLinesAsync("./inputs/09.txt")
            .Select(l =>
                l.Split(',').Select(long.Parse).ToArray() switch
                {
                    [var x, var y] => new Point2D(x, y),
                    _ => throw new InvalidOperationException(
                        "invalid coordinates")
                }).ToHashSetAsync();


        SolvePartOne(points);
    }

    private readonly record struct Point2D(long X, long Y);

    private readonly record struct Pair
    {
        public Point2D A { get; }
        public Point2D B { get; }

        public double Distance =>
            Math.Sqrt(Math.Pow(A.X - B.X, 2) + Math.Pow(A.Y - B.Y, 2d));

        public Pair(Point2D first, Point2D second)
        {
            if (first.X < second.X && first.Y < second.Y)
            {
                A = first;
                B = second;
            }
            else
            {
                A = second;
                B = first;
            }
        }
    }


    private static void SolvePartOne(HashSet<Point2D> points)
    {
        var candidates = new HashSet<Pair>();

        foreach (var point in points)
        {
            foreach (var other in points.Where(p =>
                         !(point.X == p.X && point.Y == p.Y)))
            {
                candidates.Add(new Pair(point, other));
            }
        }

        var maxArea = candidates.Where(p => p.A.X != p.B.X && p.A.Y != p.B.Y)
            .Select(p =>
            {
                var c1 = p.A;
                var c2 = new Point2D(p.A.X, p.B.Y);
                var c3 = p.B;

                var s1 = new Pair(c1, c2).Distance + 1;
                var s2 = new Pair(c2, c3).Distance + 1;

                var area = s1 * s2;

                return area;
            }).Max();
        
        Console.WriteLine($"Part One: {maxArea}");
    }

    private static void SolvePartTwo(HashSet<Point2D> points)
    {
        Console.WriteLine($"Part Two: -");
    }
}