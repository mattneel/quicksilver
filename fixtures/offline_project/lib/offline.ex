defmodule QsOffline do
  def run(value), do: QsLocalDependency.identity(value) # qs:offline-call
end

