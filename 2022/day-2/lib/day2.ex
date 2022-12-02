defmodule Day2 do
  # First = opponent
  # A = Rock
  # B = Paper
  # C = Scissors

  ### Part 1

  # Second = response
  # X = Rock = 1
  # Y = Paper = 2
  # Z = Scissors = 3

  # Loss = 0
  # Draw = 3
  # Win = 6
  def part1 do
    File.read!("input")
    |> String.split("\n", trim: true)
    |> Enum.map(fn line -> String.split(line, " ", trim: true) end)
    |> Enum.map(fn [first, second] -> calculate_score(first, second) end)
    |> Enum.sum()
  end

  def calculate_score(opponent, response) do
    type_score =
      case response do
        "X" -> 1
        "Y" -> 2
        "Z" -> 3
      end

    outcome_score =
      case {opponent, response} do
        {"A", "X"} -> 3
        {"A", "Y"} -> 6
        {"A", "Z"} -> 0
        {"B", "X"} -> 0
        {"B", "Y"} -> 3
        {"B", "Z"} -> 6
        {"C", "X"} -> 6
        {"C", "Y"} -> 0
        {"C", "Z"} -> 3
      end

    type_score + outcome_score
  end

  ### Part 2

  # X = lose
  # Y = draw
  # Z = win
  def part2 do
    File.read!("input")
    |> String.split("\n", trim: true)
    |> Enum.map(fn line -> String.split(line, " ", trim: true) end)
    |> Enum.map(fn [first, second] -> follow_guide(first, second) end)
    |> Enum.sum()
  end

  def follow_guide(opponent, outcome) do
    response =
      case {opponent, outcome} do
        {"A", "X"} -> "Z"
        {"A", "Y"} -> "X"
        {"A", "Z"} -> "Y"
        {"B", "X"} -> "X"
        {"B", "Y"} -> "Y"
        {"B", "Z"} -> "Z"
        {"C", "X"} -> "Y"
        {"C", "Y"} -> "Z"
        {"C", "Z"} -> "X"
      end

    calculate_score(opponent, response)
  end
end
