SharedTextImporter[filename_String, options___] := {"Text" -> Import[filename, "Text"]};
string2json5[text_String] := LibraryFunctionLoad["libimport_x", "import_json5", LinkObject, LinkObject][text];
string2yaml[text_String] := LibraryFunctionLoad["libimport_x", "import_yaml", LinkObject, LinkObject][text];
string2toml[text_String] := LibraryFunctionLoad["libimport_x", "import_toml", LinkObject, LinkObject][text];

ImportExport`RegisterImport[
    "JSON5",
    {
        "Text" :> SharedTextImporter,
        "Data" :> JSON5DataImporter,
        JSON5DefaultImporter
    }
];

JSON5DataImporter[filename_String, options___] := Block[
    {text = Import[filename, "Text"]},
    {"Data" -> Iconize[string2json5@text, "JSON5", Method -> BinarySerialize]}
];
JSON5DefaultImporter[filename_String, options___] := Block[
    {},
    string2json5@Import[filename, "Text"]
]
