defmodule QsFixtures.Accounts do
  def fetch(id), do: {:account, id} # qs:accounts-fetch-def
end

defmodule QsFixtures.Alternative do
  def fetch(id), do: {:alternative, id} # qs:alternative-fetch-def
end

