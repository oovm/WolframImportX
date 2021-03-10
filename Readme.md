ImportX
=======

*Extend the ability of Wolfram Language to import external resources.*

## Installation

```wolfram
release = "https://github.com/oovm/WolframImportX/releases/download/v0.1.0/ImportX-0.1.0.paclet";
PacletInstall[release, ForceVersionInstall -> True];
```

- Supports `Mac-x64`
- [Add binary release to support more platforms.](https://github.com/oovm/WolframImportX/tree/binary)

## Support Formats

| Format | Extension   |
|--------|-------------|
| JSON5  | json5, json |
| TOML   | toml        |
| YAML   | yaml, yml   |
| SVG🚧  | svg         |

### JSON5

```wolfram
<< ImportX`
json5 = PacletObject["ImportX"]["AssetLocation", "example_json5"];
Import[json5, {"JSON5", "Text"}]
Import[json5, "JSON5"]
```

### YAML

```wolfram
<< ImportX`
yaml = PacletObject["ImportX"]["AssetLocation", "example_yaml"];
Import[yaml, {"YAML", "Text"}]
Import[yaml, "YAML"]
```

### TOML

```wolfram
<< ImportX`
toml = PacletObject["ImportX"]["AssetLocation", "example_toml"];
Import[toml, {"TOML", "Text"}]
Import[toml, "TOML"]
```
