defmodule QsFixtures.Handler do
  @callback handle(term()) :: term() # qs:behavior-callback
end

defmodule QsFixtures.EchoHandler do
  @behaviour QsFixtures.Handler

  @impl true
  def handle(value), do: value # qs:behavior-implementation
end

