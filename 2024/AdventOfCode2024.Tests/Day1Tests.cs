﻿using AdventOfCode2024.Days;

namespace AdventOfCode2024.Tests;

public class Day1Tests
{
    [Fact]
    public void Part1_Sample_IsCorrect()
    {
        var day = new Day1();
        var result = day.Part1("""
                                          3   4
                                          4   3
                                          2   5
                                          1   3
                                          3   9
                                          3   3
                                          """);

        Assert.Equal("11", result);
    }
    
    [Fact]
    public void Part2_Sample_IsCorrect()
    {
        var day = new Day1();
        var result = day.Part2("""
                                          3   4
                                          4   3
                                          2   5
                                          1   3
                                          3   9
                                          3   3
                                          """);

        Assert.Equal("31", result);
    }
}