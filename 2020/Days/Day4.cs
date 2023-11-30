using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using adventofcode2020.Extensions;

namespace adventofcode2020.Days
{
    public class Day4 : IDay
    {
        public int Number { get; } = 4;

        public async Task<(string firstSolution, string secondSolution)> Solve()
        {
            var rawPassData = (await ParseAllPassData()).ToArray();
            var firstSolution = rawPassData.Count(pd => pd.IsValid()).ToString();

            var secondSolution = rawPassData.Select(ParsedPassData.FromRawData)
                .Count(pd => pd.IsValid())
                .ToString();

            return (firstSolution, secondSolution);
        }

        private static async Task<IEnumerable<RawPassData>> ParseAllPassData()
        {
            var passData = new List<RawPassData>();
            using var reader = File.OpenText("./input/day4.txt");
            while (!reader.EndOfStream)
            {
                passData.Add(await ParsePassData(reader));
            }

            return passData;
        }

        private static async Task<RawPassData> ParsePassData(TextReader reader)
        {
            var rawData = new Dictionary<TokenType, string>(8);
            var stopwatch = new Stopwatch();
            stopwatch.Start();
            while (true)
            {
                if (stopwatch.ElapsedMilliseconds > 5000)
                {
                    throw new TimeoutException("Running over 5 seconds.");
                }

                var line = await reader.ReadLineAsync();
                if (line is (null or ""))
                {
                    break;
                }

                using var lineReader = new StringReader(line);
                while (lineReader.Peek() != -1)
                {
                    var (tokenType, value) = await ReadToken(lineReader);
                    rawData.Add(tokenType, value);
                }
            }

            stopwatch.Stop();

            return new RawPassData(rawData.GetValueOrDefault(TokenType.BirthYear),
                rawData.GetValueOrDefault(TokenType.IssueYear),
                rawData.GetValueOrDefault(TokenType.ExpirationYear),
                rawData.GetValueOrDefault(TokenType.Height),
                rawData.GetValueOrDefault(TokenType.HairColor),
                rawData.GetValueOrDefault(TokenType.EyeColor),
                rawData.GetValueOrDefault(TokenType.PassportId),
                rawData.GetValueOrDefault(TokenType.CountryId));
        }

        private static async Task<(TokenType, string)> ReadToken(TextReader reader)
        {
            var chars = new char[4];
            var numRead = await reader.ReadAsync(chars);
            if (numRead < 4)
            {
                throw new InvalidOperationException($"Read less than 4 chars: '{new string(chars)}'.");
            }

            var value = new List<char>();
            var next = reader.Read();
            while (next != -1 && !char.IsWhiteSpace((char) next))
            {
                value.Add((char) next);
                next = reader.Read();
            }

            var shortTokenType = new string(chars[0..3]);
            var tokenType = shortTokenType switch
            {
                "byr" => TokenType.BirthYear,
                "iyr" => TokenType.IssueYear,
                "eyr" => TokenType.ExpirationYear,
                "hgt" => TokenType.Height,
                "hcl" => TokenType.HairColor,
                "ecl" => TokenType.EyeColor,
                "pid" => TokenType.PassportId,
                "cid" => TokenType.CountryId,
                _ => throw new InvalidOperationException($"Token type '{shortTokenType}' unknown.")
            };

            return (tokenType, new string(value.ToArray()));
        }
    }

    public enum TokenType
    {
        BirthYear,
        IssueYear,
        ExpirationYear,
        Height,
        HairColor,
        EyeColor,
        PassportId,
        CountryId
    }

    public record BirthYearData(ushort? Value)
    {
        public bool IsValid() => Value.HasValue && Value.Value.IsBetween<ushort>(1920, 2002);

        public static BirthYearData FromString(string? value) =>
            new(value is not null && ushort.TryParse(value, out var n) ? n : (ushort?) null);
    }

    public record IssueYearData(ushort? Value)
    {
        public bool IsValid() => Value.HasValue && Value.Value.IsBetween<ushort>(2010, 2020);

        public static IssueYearData FromString(string? value) =>
            new(value is not null && ushort.TryParse(value, out var n) ? n : (ushort?) null);
    }

    public record ExpirationYearData(ushort? Value)
    {
        public bool IsValid() => Value.HasValue && Value.Value.IsBetween<ushort>(2020, 2030);

        public static ExpirationYearData FromString(string? value) =>
            new(value is not null && ushort.TryParse(value, out var n) ? n : (ushort?) null);
    }

    public enum HeightUnit
    {
        Centimeters,
        Inch
    }

    public abstract record HeightData(ushort? Value)
    {
        public abstract HeightUnit Unit { get; }
        public abstract bool IsValid();

        private static readonly Regex HeightRegex =
            new("^(?<num>\\d+)(?<unit>(?:cm|in))$", RegexOptions.Compiled | RegexOptions.IgnoreCase);

        public static HeightData FromString(string? value)
        {
            if (value is null)
            {
                return new HeightCentimeters(0);
            }

            var match = HeightRegex.Match(value);

            if (!match.Success)
            {
                return new HeightCentimeters(0);
            }

            var number = ushort.TryParse(match.Groups["num"].Value, out var n) ? n : (ushort?) null;
            var unit = match.Groups["unit"].Value;

            return unit switch
            {
                "cm" => new HeightCentimeters(number),
                "in" => new HeightInch(number),
                _ => throw new InvalidOperationException()
            };
        }
    }

    public record HeightCentimeters(ushort? Value) : HeightData(Value)
    {
        public override HeightUnit Unit { get; } = HeightUnit.Centimeters;

        public override bool IsValid() => Value.HasValue && Value.Value.IsBetween<ushort>(150, 193);
    }

    public record HeightInch(ushort? Value) : HeightData(Value)
    {
        public override HeightUnit Unit { get; } = HeightUnit.Inch;
        public override bool IsValid() => Value.HasValue && Value.Value.IsBetween<ushort>(59, 76);
    }

    public record HairColorData(string? Value)
    {
        public bool IsValid() => Value is not null &&
                                 Regex.IsMatch(Value, "^#[0-9a-f]{6}$", RegexOptions.IgnoreCase);
    }

    public record EyeColorData(string? Value)
    {
        public bool IsValid() => Value is not null &&
                                 Regex.IsMatch(Value, "^amb|blu|brn|gry|grn|hzl|oth$", RegexOptions.IgnoreCase);
    }

    public record PassportIdData(string? Value)
    {
        public bool IsValid() => Value is not null &&
                                 Regex.IsMatch(Value, "^[0-9]{9}$");
    }


    public sealed record ParsedPassData(BirthYearData BirthYear,
        IssueYearData IssueYear,
        ExpirationYearData ExpirationYear,
        HeightData Height,
        HairColorData HairColor,
        EyeColorData EyeColor,
        PassportIdData PassportId,
        string? CountryId)
    {
        public bool IsValid() => BirthYear.IsValid() &&
                                 IssueYear.IsValid() &&
                                 ExpirationYear.IsValid() &&
                                 Height.IsValid() &&
                                 HairColor.IsValid() &&
                                 EyeColor.IsValid() &&
                                 PassportId.IsValid();

        public static ParsedPassData FromRawData(RawPassData rawPassData)
        {
            var (birthYear, issueYear, expirationYear, height, hairColor, eyeColor, passportId, countryId) =
                rawPassData;
            return new ParsedPassData(BirthYearData.FromString(birthYear),
                IssueYearData.FromString(issueYear),
                ExpirationYearData.FromString(expirationYear),
                HeightData.FromString(height),
                new HairColorData(hairColor),
                new EyeColorData(eyeColor),
                new PassportIdData(passportId),
                countryId);
        }
    }

    public sealed record RawPassData(string? BirthYear,
        string? IssueYear,
        string? ExpirationYear,
        string? Height,
        string? HairColor,
        string? EyeColor,
        string? PassportId,
        string? CountryId)
    {
        public bool IsValid() => BirthYear is not null &&
                                 IssueYear is not null &&
                                 ExpirationYear is not null &&
                                 Height is not null &&
                                 HairColor is not null &&
                                 EyeColor is not null &&
                                 PassportId is not null;
    }
}
