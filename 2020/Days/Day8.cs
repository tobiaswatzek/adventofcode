using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace adventofcode2020.Days
{
    public class Day8 : IDay
    {
        public int Number { get; } = 8;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var instructions = (await ParseInstructions()).ToImmutableArray();
            var firstSolution = SolveFirst(instructions);
            var secondSolution = SolveSecond(instructions);
            return (firstSolution, secondSolution);
        }

        private static string SolveFirst(IReadOnlyList<Instruction> instructions)
        {
            var environment = RunInstructions(instructions);

            return environment.Accumulator.ToString();
        }

        private static string SolveSecond(IImmutableList<Instruction> instructions)
        {
            for (int i = 0; i < instructions.Count; i++)
            {
                var swappedInstruction = instructions[i] switch
                {
                    Jump jmp => (Instruction) new NoOperation(jmp.Address, jmp.Value),
                    NoOperation nop => new Jump(nop.Address, nop.Value),
                    Accumulation acc => acc,
                    _ => throw new InvalidOperationException("Instruction not known.")
                };

                var updatedInstructions = instructions.SetItem(i, swappedInstruction);

                var environment = RunInstructions(updatedInstructions);

                if (!environment.Aborted)
                {
                    return environment.Accumulator.ToString();
                }
            }

            throw new InvalidOperationException("No solution found.");
        }

        private static Environment RunInstructions(IReadOnlyList<Instruction> instructions)
        {
            var instructionHistory = new HashSet<Instruction>();
            var environment = new Environment(0);
            var instruction = instructions[0];
            while (instruction is not null)
            {
                if (instructionHistory.Contains(instruction))
                {
                    environment.Abort();
                    break;
                }

                instructionHistory.Add(instruction);
                instruction = instruction.Execute(environment, instructions);
            }

            return environment;
        }

        private static async Task<IEnumerable<Instruction>> ParseInstructions()
        {
            var lines = await File.ReadAllLinesAsync("./input/day8.txt");
            return lines.Select((line, address) =>
            {
                var parts = line.Split(" ", StringSplitOptions.TrimEntries);
                var instructionName = parts[0];
                var instructionValue = int.Parse(parts[1], NumberStyles.Integer);

                return instructionName switch
                {
                    "jmp" => (Instruction) new Jump(address, instructionValue),
                    "acc" => new Accumulation(address, instructionValue),
                    "nop" => new NoOperation(address, instructionValue),
                    _ => throw new InvalidOperationException($"Instruction name '{instructionName}' not found.")
                };
            });
        }
    }


    public class Environment
    {
        public long Accumulator { get; private set; }

        public bool Aborted { get; private set; }

        public Environment(long initialAccumulatorValue)
        {
            Accumulator = initialAccumulatorValue;
        }

        public void AddToAccumulator(int n)
        {
            Accumulator += n;
        }

        public void Abort()
        {
            Aborted = true;
        }
    }

    public abstract record Instruction(int Address, int Value)
    {
        public Instruction? Execute(Environment environment, IEnumerable<Instruction> allInstructions)
        {
            return ExecuteInternal(environment, allInstructions);
        }

        protected abstract Instruction? ExecuteInternal(Environment environment,
            IEnumerable<Instruction> allInstructions);
    }

    public record Accumulation(int Address, int Value) : Instruction(Address, Value)
    {
        protected override Instruction? ExecuteInternal(Environment environment,
            IEnumerable<Instruction> allInstructions)
        {
            environment.AddToAccumulator(Value);
            return allInstructions.ElementAtOrDefault(Address + 1);
        }
    }

    public record Jump(int Address, int Value) : Instruction(Address, Value)
    {
        protected override Instruction? ExecuteInternal(Environment environment,
            IEnumerable<Instruction> allInstructions)
        {
            return allInstructions.ElementAtOrDefault(Address + Value);
        }
    }


    public record NoOperation(int Address, int Value) : Instruction(Address, Value)
    {
        protected override Instruction? ExecuteInternal(Environment environment,
            IEnumerable<Instruction> allInstructions)
        {
            return allInstructions.ElementAtOrDefault(Address + 1);
        }
    }
}
