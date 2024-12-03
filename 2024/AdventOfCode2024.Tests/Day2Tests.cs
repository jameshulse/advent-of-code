using AdventOfCode2024.Days;

namespace AdventOfCode2024.Tests;

public class Day2Tests
{
    private const string ExampleInput = """
                                        7 6 4 2 1
                                        1 2 7 8 9
                                        9 7 6 2 1
                                        1 3 2 4 5
                                        8 6 4 4 1
                                        1 3 6 7 9
                                        """;
    
    [Fact]
    public void Part1_Sample_IsCorrect()
    {
        var day = new Day2();
        var result = day.Part1(ExampleInput);

        Assert.Equal("2", result);
    }
    
    [Fact]
    public void Part2_Sample_IsCorrect()
    {
        var day = new Day2();
        var result = day.Part2(ExampleInput);

        Assert.Equal("4", result);
    }
}