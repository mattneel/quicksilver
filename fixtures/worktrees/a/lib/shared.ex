defmodule QsFixtures.Shared do
  def value(), do: :worktree_a # qs:worktree-a-def
  def call(), do: value() # qs:worktree-a-call
end

