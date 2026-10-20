defmodule QsFixtures.Target do
  def echo(value), do: value # qs:noise-target-def
end

defmodule QsFixtures.ReferenceNoise do
  def echo(value), do: {:local, value} # qs:noise-local-def
  def local(value), do: echo(value) # qs:noise-local-call
  def real(value), do: QsFixtures.Target.echo(value) # qs:noise-real-call
  def capture, do: &QsFixtures.Target.echo/1 # qs:noise-capture
  def text, do: "QsFixtures.Target.echo(value)" # qs:noise-string
  # QsFixtures.Target.echo(value) qs:noise-comment
end

