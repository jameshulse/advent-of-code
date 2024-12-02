using System.Collections.Frozen;
using System.Collections.Immutable;
using AdventOfCode2024.Lib;

namespace AdventOfCode2024.Days;

public class Day1 : IAdventDay
{
    public string Part1(string input)
    {
        var (left, right) = ParseLists(input);
        
        left.Sort();
        right.Sort();
        
        var result = left
            .Zip(right)
            .Aggregate(0, (current, values) => current + Math.Abs(values.First - values.Second));

        return result.ToString();
    }

    public string Part2(string input)
    {
        var (left, right) = ParseLists(input);

        var rightCounts = right
            .GroupBy(val => val)
            .Select(group => KeyValuePair.Create(group.Key, group.Count()))
            .ToDictionary();
        
        var result = left.Aggregate(0, (current, val) => current + rightCounts.GetValueOrDefault(val, 0) * val);

        return result.ToString();
    }
    
    private (List<int>, List<int>) ParseLists(string input)
    {
        var inputSpan = input.AsSpan();
        int numberWidth = inputSpan.IndexOf(" ");
        
        var lines = inputSpan.Split("\n");
        
        var left = new List<int>(1000);
        var right = new List<int>(1000);
        
        foreach (var line in lines)
        {
            left.Add(int.Parse(inputSpan[line][..numberWidth]));
            right.Add(int.Parse(inputSpan[line][(numberWidth + 3)..])); // +3 to skip the space
        }
        
        return (left, right);
    }
}