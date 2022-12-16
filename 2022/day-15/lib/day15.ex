defmodule Day15 do
  def part1(input, intersection_row) do
    input
    |> read_sensor_list()
    |> Enum.map(fn %{sensor: {sx, sy} = sensor, beacon: {bx, by}} ->
      radius = abs(sx - bx) + abs(sy - by)

      intersection(sensor, radius, intersection_row)
    end)
    |> Enum.filter(fn intersection -> intersection != :none end)
    |> IO.inspect()
    |> then(&reduce_ranges/1)
    |> IO.inspect()
    |> Enum.map(fn range ->
      size = Range.size(range)
      
      size
    end)
    |> Enum.sum()
  end

  def reduce_ranges(ranges) do
    overlap =
      Enum.zip(ranges, Enum.drop(ranges, 1))
      |> Enum.find(fn {l, r} -> !Range.disjoint?(l, r) end)

    case overlap do
      {l, r} ->
        new_range = min(l.first, r.first)..max(l.last, r.last)
        new_result = (ranges |> List.delete(l) |> List.delete(r)) ++ [new_range]

        reduce_ranges(new_result)

      nil ->
        ranges
    end
  end

  # def part2(input) do

  # end

  def read_sensor_list(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.reduce(%{sensors: [], beacons: []}, fn line, %{sensors: sensors, beacons: beacons} ->
      %{"sx" => sx, "sy" => sy, "bx" => bx, "by" => by} =
        Regex.named_captures(
          ~r/^Sensor at x=(?<sx>-?\d+), y=(?<sy>-?\d+): closest beacon is at x=(?<bx>-?\d+), y=(?<by>-?\d+)$/,
          line
        )
        
      sx = String.to_integer(sx)
      sy = String.to_integer(sy)
      bx = String.to_integer(bx)
      by = String.to_integer(by)
      radius = abs(sx - bx) + abs(sy - by)
      
      %{
        sensors: sensors ++ [{sx, sy, radius}],
        beacons: beacons ++ [{bx, by}]
      }

      %{
        sensor: {, String.to_integer(sy)},
        beacon: {String.to_integer(bx), String.to_integer(by)}
      }
    end)
  end

  def intersection({sx, sy}, radius, row) do
    cond do
      row < sy - radius ->
        :none

      row > sy + radius ->
        :none

      true ->
        width = radius - abs(sy - row)

        (sx - width)..(sx + width)
    end
  end
end

..####B######################..
