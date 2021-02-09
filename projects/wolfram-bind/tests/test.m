(* ::Package:: *)

SetDirectory@NotebookDirectory[];
PacletInstall[CreatePacletArchive["../"], ForceVersionInstall -> True];
First@PacletFind["ImportX"]


<< ImportX`;
Import["test.yaml", "YAML"] // Dataset
Import["test.json5", "JSON5"] // Dataset
Import["test.toml", "TOML"] // Dataset


toml = PacletObject["ImportX"]["AssetLocation", "example_toml"];
Import[toml, {"TOML", "Text"}]
Import[toml, "TOML"]



