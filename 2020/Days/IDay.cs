using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public interface IDay
    {
        public int Number { get; }

        public Task<(string firstSolution, string secondSolution)> Solve();
    }
}
