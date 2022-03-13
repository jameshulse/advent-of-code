defmodule Day1 do
  def part1 do
    {:ok, contents} = File.read("input")

    contents
    |> String.split("\n", trim: true)
    |> Enum.map(&String.to_integer(&1))
    |> Enum.map(&calculate_fuel(&1))
    |> Enum.sum()
  end

  @type mass :: integer

  def calculate_fuel(mass) do
    trunc(mass / 3) - 2
  end

  def part2 do
    {:ok, contents} = File.read("input")

    contents
    |> String.split("\n", trim: true)
    |> Enum.map(&String.to_integer(&1))
    |> Enum.map(&recurse_fuel(&1))
    |> Enum.sum()
  end

  def recurse_fuel(curr) do
    fuel = calculate_fuel(curr)

    if fuel <= 0 do
      0
    else
      fuel + recurse_fuel(fuel)
    end
  end
end
