namespace AdventOfCode2024.Lib;

public static class Number
{
    public static int FastParseInt(ReadOnlySpan<char> input)
    {
        int num = 0;
        
        foreach (var t in input)
        {
            num = num * 10 + (t - '0');
        }
        
        return num;
    }
}