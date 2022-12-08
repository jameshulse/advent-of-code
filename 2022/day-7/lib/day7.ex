defmodule Day7 do
  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.reduce({"/", %{"/" => 0}}, &parse_command/2)
    |> then(fn {_pwd, filesystem} -> get_directory_total_sizes(filesystem) end)
    |> Enum.map(fn {_dir, size} -> size end)
    |> Enum.filter(fn size -> size <= 100_000 end)
    |> Enum.sum()
  end

  @disk_space 70_000_000
  @update_size 30_000_000

  def part2(input) do
    sizes =
      input
      |> String.split("\n", trim: true)
      |> Enum.reduce({"/", %{"/" => 0}}, &parse_command/2)
      |> then(fn {_pwd, filesystem} -> get_directory_total_sizes(filesystem) end)

    {_, root_size} = sizes |> Enum.find(fn {dir, _} -> dir == "/" end)

    required_space = @update_size - (@disk_space - root_size)

    sizes
    |> Enum.map(fn {_dir, size} -> size end)
    |> Enum.filter(fn size -> size >= required_space end)
    |> Enum.sort()
    |> List.first()
  end

  def get_directory_total_sizes(filesystem) do
    system_directories = Map.to_list(filesystem)

    system_directories
    |> Enum.map(fn {path, self_size} ->
      name =
        case path do
          "/" -> "/"
          _ -> path |> String.split("/") |> List.last()
        end

      child_size =
        system_directories
        |> Enum.filter(fn {child, _size} ->
          path != child && String.starts_with?(child, path)
        end)
        |> Enum.map(fn {_child, size} -> size end)
        |> Enum.sum()

      {name, self_size + child_size}
    end)
  end

  def parse_command(command, {pwd, filesystem} = state) do
    case command do
      "$ cd .." ->
        {String.replace(pwd, ~r"/\w+$", ""), filesystem}

      "$ cd /" ->
        state

      "$ cd " <> dir ->
        new_pwd =
          case pwd do
            "/" -> "/#{dir}"
            _ -> "#{pwd}/#{dir}"
          end

        {new_pwd, Map.put(filesystem, new_pwd, 0)}

      "$ ls" ->
        state

      "dir " <> _ ->
        state

      _file ->
        [size, _] = String.split(command)
        {pwd, Map.put(filesystem, pwd, filesystem[pwd] + String.to_integer(size))}
    end
  end
end
