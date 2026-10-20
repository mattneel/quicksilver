defmodule QsFixtures.DynamicTarget do
  def known(value), do: value # qs:dynamic-target-def
end

defmodule QsFixtures.Dynamic do
  def dispatch(function, value) do
    apply(QsFixtures.DynamicTarget, function, [value]) # qs:dynamic-apply
  end

  def remote(module, value), do: module.fetch(value) # qs:unknown-remote
end

