defmodule QsFixtures.Malformed do
  def intact(), do: :ok # qs:malformed-intact

  def broken() do
    [1, 2 # qs:malformed-error
  end
end

