# Formula language

View → Interface language is the existing, persisted language preference. Choose **Español**
for Spanish formula entry and display, or **Deutsch** for German. On first run the preference
follows the system language. English and the other interface languages currently use the
English formula table and separators.

| Spanish | German | Canonical English |
| --- | --- | --- |
| `SUMA` | `SUMME` | `SUM` |
| `PROMEDIO` | `MITTELWERT` | `AVERAGE` |
| `SI` | `WENN` | `IF` |
| `CONTAR` | `ANZAHL` | `COUNT` |
| `SUMAR.SI` | `SUMMEWENN` | `SUMIF` |
| `CONTAR.SI` | `ZÄHLENWENN` | `COUNTIF` |
| `BUSCARV` | `SVERWEIS` | `VLOOKUP` |
| `HOY` | `HEUTE` | `TODAY` |
| `VERDADERO`, `FALSO` | `WAHR`, `FALSCH` | `TRUE`, `FALSE` (literals or function calls) |

German has about 410 translated function names in total (`crates/formula/src/input/de.rs`),
sourced from Microsoft's de-de Excel function pages; functions without a confirmed German name
keep their English spelling.

Names are case-insensitive. Spanish and German formulas use decimal commas (`1,5`) and
semicolon arguments and reference unions. Spanish uses backslashes between array columns
(`{1\2;3\4}`); German uses dots (`{1.2;3.4}`). For example, `=SI(VERDADERO;SUMA(1,5;2,25);0)`
and `=WENN(WAHR;SUMME(1,5;2,25);0)` both evaluate to 3.75. English mode retains comma
arguments, decimal points and comma array columns; it does not silently translate aliases.

The cell editor, formula bar, Show Formulas, autocomplete, the Formula ribbon, Insert Function, and argument hints
use the selected formula language. Common Spanish and German function help is translated;
functions without a translated entry retain their English names/help. General interface labels
and errors fall back to English where a translation is not provided. An edit captures its
language when it starts so switching the preference cannot reinterpret unfinished input.

Workbook/sheet names, table names, LET bindings and LAMBDA parameters take precedence over
aliases. Text, sheet names and table column names are unchanged. When a localized spelling
would collide with a name, displayed formulas retain the canonical spelling to preserve meaning.

Stored formulas, XLSX import/export, engine commands, control/API calls, CLI evaluation and
clipboard exchange retain strict English syntax. This also applies to advanced dialogs such as
Name Manager and conditional-format rules. Localization happens at the cell/formula-bar editor
boundary, including filling the selected range with Ctrl+Enter; it does not change the workbook
format or teach the calculation engine ambiguous aliases.
