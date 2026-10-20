defmodule QsOffline.MixProject do
  use Mix.Project

  def project do
    [
      app: :qs_offline,
      version: "0.1.0",
      deps: [{:qs_local_dependency, path: "../offline_dependency"}]
    ]
  end

  def application, do: []
end

