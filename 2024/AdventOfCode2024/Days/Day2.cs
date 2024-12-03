using System.Collections.Frozen;
using System.Collections.Immutable;
using AdventOfCode2024.Lib;

namespace AdventOfCode2024.Days;

public class Day2 : IAdventDay
{
    enum Direction
    {
        Unknown,
        Ascending,
        Descending
    }

    enum ReportResult
    {
        Safe,
        Unsafe,
    }
    
    public string Part1(string input)
    {
        var inputSpan = input.AsSpan();
        var lines = inputSpan.Split("\n");

        int safeCount = 0;
        
        foreach (var lineRange in lines)
        {
            if (AnalyseReport(inputSpan[lineRange]) == ReportResult.Safe)
            {
                safeCount++;
            }
        }

        return safeCount.ToString();
    }

    private ReportResult AnalyseReport(ReadOnlySpan<char> report, bool withDampener = false)
    {
        var values = report.Split(" ");
        var currentDirection = Direction.Unknown;
        int? lastValue = null;
        bool dampenerTriggered = !withDampener; // Consider the dampener triggered if there is no dampener

        foreach (var valueRange in values)
        {
            int currentValue = Number.FastParseInt(report[valueRange]);

            if (lastValue is null)
            {
                lastValue = currentValue;
                continue; // Skip the first value
            }

            int change = Math.Abs((int)(currentValue - lastValue));
            var pendingResult = ReportResult.Safe;

            if (change < 1 || change > 3)
            {
                pendingResult = ReportResult.Unsafe;
            }

            var nextDirection = currentValue > lastValue ? Direction.Ascending : Direction.Descending;

            if (currentDirection == Direction.Unknown)
            {
                currentDirection = nextDirection;
            }
            else if (currentDirection != nextDirection)
            {
                pendingResult = ReportResult.Unsafe;
            }

            if (pendingResult == ReportResult.Unsafe && dampenerTriggered)
            {
                return pendingResult;
            }

            lastValue = currentValue;
        }

        return ReportResult.Safe;
    }

    public string Part2(string input)
    {
        var inputSpan = input.AsSpan();
        var lines = inputSpan.Split("\n");

        int safeCount = 0;
        
        foreach (var lineRange in lines)
        {
            if (AnalyseReport(inputSpan[lineRange], true) == ReportResult.Safe)
            {
                safeCount++;
            }
        }

        return safeCount.ToString();
    }
}