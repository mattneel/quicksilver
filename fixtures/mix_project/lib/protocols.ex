defprotocol QsFixtures.Label do
  def label(value) # qs:protocol-callback
end

defimpl QsFixtures.Label, for: Integer do
  def label(value), do: Integer.to_string(value) # qs:protocol-integer-impl
end

defmodule QsFixtures.ProtocolCaller do
  def run(value), do: QsFixtures.Label.label(value) # qs:protocol-call
end

