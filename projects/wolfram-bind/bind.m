BeginPackage["ImportX`"];

SharedTextImporter[filename_String, options___] := {"Text" -> Import[filename, "Text"]};
SharedBinaryImporter[filename_String, options___] := {"Binary" -> Import[filename, "Binary"]};
string2json5[text_String] := LibraryFunctionLoad["libimport_x", "import_json5", LinkObject, LinkObject][text];
string2yaml[text_String] := LibraryFunctionLoad["libimport_x", "import_yaml", LinkObject, LinkObject][text];
string2toml[text_String] := LibraryFunctionLoad["libimport_x", "import_toml", LinkObject, LinkObject][text];

ImportExport`RegisterImport[
    "JSON5",
    {
        "Elements" :> Function[{}, Sort@{"Elements", "Text", "Data"}],
        "Text" :> SharedTextImporter,
        "Data" :> JSON5DataImporter,
        JSON5DefaultImporter
    }
];

JSON5DataImporter[filename_String, options___] := Block[
    {text = Import[filename, "Text"]},
    {"Data" -> Dataset[string2json5@text]}
];
JSON5DefaultImporter[filename_String, options___] := Block[
    {},
    string2json5@Import[filename, "Text"]
];

ImportExport`RegisterImport[
    "TOML",
    {
        "Elements" :> Function[{}, Sort@{"Elements", "Text", "Data"}],
        "Text" :> SharedTextImporter,
        "Data" :> TomlDataImporter,
        TomlDefaultImporter
    }
];

TomlDataImporter[filename_String, options___] := Block[
    {text = Import[filename, "Text"]},
    {"Data" -> Dataset[string2toml@text]}
];
TomlDefaultImporter[filename_String, options___] := Block[
    {},
    string2toml@Import[filename, "Text"]
];

ImportExport`RegisterImport[
    "YAML",
    {
        "Elements" :> Function[{}, Sort@{"Elements", "Text", "Data"}],
        "Text" :> SharedTextImporter,
        "Data" :> YamlDataImporter,
        YamlDefaultImporter
    }
];

YamlDataImporter[filename_String, options___] := Block[
    {text = Import[filename, "Text"]},
    {"Data" -> Dataset[string2yaml@text]}
];
YamlDefaultImporter[filename_String, options___] := Block[
    {},
    string2yaml@Import[filename, "Text"]
];

EndPackage[]