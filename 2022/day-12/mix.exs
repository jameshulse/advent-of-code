defmodule Day12.MixProject do
  use Mix.Project

  def project do
    [
      app: :day_12,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      aliases: [test: ["test --no-start"]]
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:eastar, "~> 0.5.1"}
    ]
  end
end
