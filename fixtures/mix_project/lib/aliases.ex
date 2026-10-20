defmodule QsFixtures.Aliases do
  alias QsFixtures.Accounts, as: Accounts
  alias Accounts, as: Again

  def direct(id), do: Accounts.fetch(id) # qs:alias-as-call
  def chained(id), do: Again.fetch(id) # qs:alias-chain-call

  def grouped(id) do
    alias QsFixtures.{Accounts, Math}
    {Accounts.fetch(id), Math.plus(1, 2)} # qs:alias-group-call
  end

  def inner(id) do
    alias QsFixtures.Alternative, as: Accounts
    Accounts.fetch(id) # qs:alias-inner-call
  end

  def outer(id), do: Accounts.fetch(id) # qs:alias-outer-call
end

