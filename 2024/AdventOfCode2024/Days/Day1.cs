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

        int total = 0;
        
        for (int i = 0; i < left.Count; i++)
        {
            total += Math.Abs(left[i] - right[i]);
        }
        
        return total.ToString();
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
            left.Add(Number.FastParseInt(inputSpan[line][..numberWidth]));
            right.Add(Number.FastParseInt(inputSpan[line][(numberWidth + 3)..])); // +3 to skip the space
            // left.Add(int.Parse(inputSpan[line][..numberWidth]));
            // right.Add(int.Parse(inputSpan[line][(numberWidth + 3)..])); // +3 to skip the space
        }
        
        return (left, right);
    }
}