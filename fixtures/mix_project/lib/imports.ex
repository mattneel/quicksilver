defmodule QsFixtures.ImportOnly do
  import QsFixtures.Math, only: [plus: 2]

  def add(left, right), do: plus(left, right) # qs:import-only-call
  def piped(value), do: value |> plus(2) # qs:pipe-call
end

defmodule QsFixtures.ImportExcept do
  import QsFixtures.Math, except: [times: 2]

  def calculate(left, right) do
    {plus(left, right), times(left, right)} # qs:import-except-call
  end

  defp times(left, right), do: {:local, left, right} # qs:local-times-def
end

