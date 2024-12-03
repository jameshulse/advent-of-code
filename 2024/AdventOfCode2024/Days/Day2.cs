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
            var line = inputSpan[lineRange];

            if (AnalyseReport(line) == (ReportResult.Safe, null))
            {
                safeCount++;
            }
        }

        return safeCount.ToString();
    }

    private (ReportResult result, int? failedIndex) AnalyseReport(ReadOnlySpan<char> report, int? ignoreIndex = null)
    {
        var values = report.Split(" ");
        var currentDirection = Direction.Unknown;
        int currentIndex = 0;
        int? lastValue = null;

        foreach (var valueRange in values)
        {
            int currentValue = Number.FastParseInt(report[valueRange]);

            if (lastValue is null && ignoreIndex != currentIndex)
            {
                lastValue = currentValue;
                currentIndex++;
                continue; // Skip the first value
            }

            if (currentIndex != ignoreIndex)
            {
                int change = Math.Abs((int)(currentValue - lastValue)!);

                if (change < 1 || change > 3)
                {
                    return (ReportResult.Unsafe, currentIndex);
                }

                var nextDirection = currentValue > lastValue ? Direction.Ascending : Direction.Descending;

                if (currentDirection == Direction.Unknown)
                {
                    currentDirection = nextDirection;
                }
                else if (currentDirection != nextDirection)
                {
                    return (ReportResult.Unsafe, currentIndex);
                }
                
                lastValue = currentValue;
            }
            
            currentIndex++;
        }

        return (ReportResult.Safe, null);
    }

    private ReportResult AnalyseReportWithDampening(ReadOnlySpan<char> report)
    {
        foreach (var ignoreIndex in new int?[] { null, 0, 1, 2, 3, 4, 5, 6, 7})
        {
            if (AnalyseReport(report, ignoreIndex) == (ReportResult.Safe, null))
            {
                return ReportResult.Safe;
            }
        }

        return ReportResult.Unsafe;
    }
    
    public string Part2(string input)
    {
        var inputSpan = input.AsSpan();
        var lines = inputSpan.Split("\n");

        int safeCount = 0;
        
        foreach (var lineRange in lines)
        {
            var line = inputSpan[lineRange];
            
            if (AnalyseReportWithDampening(line) == ReportResult.Safe)
            {
                safeCount++;
            }
        }

        return safeCount.ToString();
    }
}