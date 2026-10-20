defmodule QsFixtures.Quoted do
  def quoted do # qs:quoted-function
    quote do
      def ghost(), do: :never_defined_here # qs:quoted-ghost
    end
  end

  def live(), do: :live # qs:quoted-live
end

