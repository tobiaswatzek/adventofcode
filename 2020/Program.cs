using System;
using System.Linq;
using adventofcode2020;
using adventofcode2020.Days;
using Spectre.Console;

var dayInterface = typeof(IDay);
var days = AppDomain.CurrentDomain.GetAssemblies()
    .SelectMany(a => a.GetTypes())
    .Where(t => dayInterface.IsAssignableFrom(t) && t.IsClass && !t.IsAbstract)
    .Select(t => Activator.CreateInstance(t) as IDay)
    .Where(day => day is not null);

if (args.Length == 1 && args[0] == "latest")
{
    days = days
        .OrderByDescending(day => day!.Number)
        .Take(1);
}
else
{
    days = days.OrderBy(day => day!.Number);
}

var table = new Table();
table.AddColumns("Day", "First Answer", "Second Answer");
foreach (var day in days)
{
    var (firstSolution, secondSolution) = await day!.Solve();
    table.AddRow(day.Number.ToString(), firstSolution, secondSolution);
}

AnsiConsole.Render(table);