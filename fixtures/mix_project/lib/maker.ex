defmodule QsFixtures.Maker do
  defmacro __using__(_options) do
    quote location: :keep do
      def generated(), do: :generated # qs:generated-origin
    end
  end
end

