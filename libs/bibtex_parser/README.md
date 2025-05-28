# Bibtex-Parser
Dieses Modul liest eine `.bib` Datei ein und gibt sie als Struktur `Document` zurück.

> [!WARNING]
> Die Structure wird nicht validiert. Für Validierung benutze `bibtex_validator`.

## Technische Informationen
Zum Parsen der `.bib` Dateien wird eine Pest Grammatik verwendet. Diese Grammatik ist in `bibtex.pest` definiert.
