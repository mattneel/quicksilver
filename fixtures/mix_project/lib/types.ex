defmodule QsFixtures.Types do
  @spec accept(term()) :: {:ok, term()}
  def accept(value), do: {:ok, value} # qs:any-type-def
  def call, do: accept("a string is valid") # qs:any-type-call
end

