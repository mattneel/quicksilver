defmodule QsFixtures.Functions do
  def greet(name, suffix \\ "!"), do: name <> suffix # qs:greet-def

  def short(name), do: greet(name) # qs:default-short-call
  def full(name), do: greet(name, "?") # qs:default-full-call

  def captured, do: &QsFixtures.Accounts.fetch/1 # qs:capture-call

  defguard is_positive(value) when is_integer(value) and value > 0 # qs:guard-def
  def guarded(value) when is_positive(value), do: value # qs:guard-call

  def public(value), do: hidden(value) # qs:private-call
  defp hidden(value), do: value + 1 # qs:private-def

  def parse(0), do: :zero # qs:parse-clause-zero
  def parse(value) when is_integer(value), do: {:integer, value} # qs:parse-clause-integer
  def call_parse(value), do: parse(value) # qs:parse-call
end

