using System.Collections.Frozen;
using AdventOfCode2024.Lib;

namespace AdventOfCode2024.Days;

public class Day1 : IAdventDay
{
    public string Part1(string input)
    {
        var (left, right) = ParseLists(input);
        
        left.Sort();
        right.Sort();

        var result = left.Zip(right)
            .Select((values) => Math.Abs(values.First - values.Second))
            .Sum();

        return result.ToString();
    }

    public string Part2(string input)
    {
        var (left, right) = ParseLists(input);

        var rightCounts = right.GroupBy(val => val)
            .Select(group => KeyValuePair.Create(group.Key, group.Count()))
            .ToFrozenDictionary();
        
        var result = left.Aggregate(0, (current, val) => current + rightCounts.GetValueOrDefault(val, 0) * val);

        return result.ToString();
    }
    
    private (List<int>, List<int>) ParseLists(string input)
    {
        var inputSpan = input.AsSpan();
        var lines = inputSpan.Split("\n");
        var left = new List<int>(1000);
        var right = new List<int>(1000);
        
        foreach (var line in lines)
        {
            left.Add(int.Parse(inputSpan[line][..5]));
            right.Add(int.Parse(inputSpan[line][8..]));
        }
        
        return (left, right);
    }
}