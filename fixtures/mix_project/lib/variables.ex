defmodule QsFixtures.Variables do
  def rebound(input) do
    value = input # qs:rebind-first
    value = value + 1 # qs:rebind-second
    {:new, value} # qs:rebind-result
  end

  def pinned(id, tuple) do # qs:pin-parameter
    {^id, payload} = tuple # qs:pin-query
    payload
  end

  def with_scope(outer, result) do
    value = outer # qs:with-outer-binding

    inner =
      with {:ok, value} <- result do # qs:with-generator
        value # qs:with-inner-read
      end

    {value, inner} # qs:with-outer-read
  end

  def shadow(value) do # qs:shadow-outer-parameter
    function = fn value -> value + 1 end # qs:shadow-anonymous
    {value, function.(3)} # qs:shadow-outer-read
  end
end

