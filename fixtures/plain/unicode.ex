defmodule QsFixtures.Unicode do
  def run, do: "🧪café" |> QsFixtures.Accounts.fetch() # qs:unicode-call
end

