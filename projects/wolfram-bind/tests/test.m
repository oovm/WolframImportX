(* ::Package:: *)

SetDirectory@NotebookDirectory[];
PacletInstall[CreatePacletArchive["../"],ForceVersionInstall->True]


<<ImportX`
Import["test.yaml","YAML"]//Dataset
Import["test.json5","JSON5"]//Dataset
Import["test.toml","TOML"]//Dataset



