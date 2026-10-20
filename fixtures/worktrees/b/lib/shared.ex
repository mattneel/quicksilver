defmodule QsFixtures.Shared do
  def value(), do: :worktree_b # qs:worktree-b-def
  def call(), do: value() # qs:worktree-b-call
end

