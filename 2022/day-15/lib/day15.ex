defmodule Day15 do
  def part1(input, intersection_row) do
    %{sensors: sensors, beacons: beacons} = read_sensor_list(input)

    sensors
    |> Enum.map(fn {sx, sy, radius} ->
      intersection({sx, sy}, radius, intersection_row)
    end)
    |> Enum.filter(fn intersection -> intersection != :none end)
    |> Enum.sort()
    |> then(&reduce_ranges/1)
    |> Enum.map(fn range ->
      width = Range.size(range)

      beacons_inline =
        Enum.count(beacons, fn {bx, by} -> bx in range and by == intersection_row end)

      width - beacons_inline
    end)
    |> Enum.sum()
  end

  def overlap?(r1, r2) do
    cond do
      r1.first < r2.first ->
        r1.last + 1 >= r2.first

      r2.first < r1.first ->
        r2.last + 1 >= r1.first

      true ->
        true
    end
  end

  def reduce_ranges(ranges) do
    ranges = ranges |> Enum.sort()

    overlap =
      Enum.zip(ranges, Enum.drop(ranges, 1))
      |> Enum.find(fn {l, r} -> overlap?(l, r) end)

    case overlap do
      {l, r} ->
        new_range = min(l.first, r.first)..max(l.last, r.last)
        new_result = (ranges |> List.delete(l) |> List.delete(r)) ++ [new_range]

        reduce_ranges(new_result)

      nil ->
        ranges
    end
  end

  def part2(input, search_range) do
    %{sensors: sensors, beacons: beacons} = read_sensor_list(input)

    search_range
    |> Stream.map(fn row ->
      coverage =
        sensors
        |> Enum.map(fn {sx, sy, radius} -> intersection({sx, sy}, radius, row) end)
        |> Enum.filter(fn intersection -> intersection != :none end)
        |> Enum.map(fn intersection ->
          # Constrain intersection to search range
          max(search_range.first, intersection.first)..min(intersection.last, search_range.last)
        end)
        # Add beacons so that we don't count them as empty space
        |> Enum.concat(
          beacons
          |> Enum.flat_map(fn {bx, by} ->
            if by == row and bx in search_range do
              [bx..bx]
            else
              []
            end
          end)
        )
        |> Enum.sort()
        |> then(&reduce_ranges/1)

      {row, coverage}
    end)
    |> Stream.filter(fn {_row, coverage} -> length(coverage) > 1 end)
    |> Stream.take(1)
    |> Enum.to_list()
    |> List.first()
    |> IO.inspect()
    |> then(fn {row, [r1, r2]} ->
      col =
        cond do
          r1.first < r2.first -> r1.last + 1
          r2.first < r1.first -> r2.last + 1
        end

      col * 4_000_000 + row
    end)
    |> IO.inspect()
  end

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
        beacons: Enum.uniq(beacons ++ [{bx, by}])
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
