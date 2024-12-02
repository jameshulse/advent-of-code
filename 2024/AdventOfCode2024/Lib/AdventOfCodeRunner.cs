using System.Diagnostics;

namespace AdventOfCode2024.Lib;

public static class AdventOfCodeRunner
{
    public static void Run<T>()
        where T : IAdventDay, new()
    {
        var input = ReadInput<T>();
        var day = new T();
        
        Console.WriteLine($"Running {typeof(T).Name}");
        Console.WriteLine("============");
        Console.WriteLine();

        Console.WriteLine("Part 1");
        
        RunPart(() => day.Part1(input));
        
        Console.WriteLine();
        Console.WriteLine("Part 2");
        
        RunPart(() => day.Part2(input));
    }

    private static void RunPart(Func<string> run)
    {
        try
        {
            var sw = Stopwatch.StartNew();
            
            Console.WriteLine($"    Solution:    {run()}");
            Console.WriteLine($"    Time taken:  {sw.ElapsedMilliseconds} ms");
        }
        catch (NotImplementedException)
        {
            Console.WriteLine("    Not implemented");
        }
    }

    private static string ReadInput<T>() where T : IAdventDay, new()
    {
        var dayName = typeof(T).Name.ToLower();

        return File.ReadAllText($"Data/{dayName}.txt");
    }
}