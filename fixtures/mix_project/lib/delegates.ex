defmodule QsFixtures.Delegates do
  defdelegate get(id), to: QsFixtures.Accounts, as: :fetch # qs:delegate-def
end

defmodule QsFixtures.DelegateCaller do
  def run(id), do: QsFixtures.Delegates.get(id) # qs:delegate-call
end

