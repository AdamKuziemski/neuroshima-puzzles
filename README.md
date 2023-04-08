# neuroshima-puzzles
Kolekcja zagadek do Neuroshimy i innych RPG z łamaniem zabezpieczeń elektronicznych

## Użycie
Wpisz w konsoli:

```bash
cargo run -- [opcja] [parametry]
```

Jeśli uruchamiasz zbudowaną aplikację:
```bash
neuroshima-puzzles [opcja] [parametry]
```

### Opcje

#### Zamek cyfrowy
Opcja `lock` to symulacja otwierania cyfrowego zamka.

**Parametry:**
* `--digits [liczba od 2 do 7]` (skrót: `-d [liczba]`) - domyślnie: 3
* `--code [kod zamka]` (skrót: `-c [kod]`) - domyślnie losowy, długość nie może przekroczyć 7

**Przykłady:**
* `neuroshima-puzzles lock --digits 3` daje nam losowy trzycyfrowy zamek z losowym kodem.
* `neuroshima-puzzles lock --code 1234` daje nam czterocyfrowy zamek o kodzie `1234`.

Postać gracza musi odgadnąć szyfr w określonej liczbie ruchów, a zamek odpowiada, czy zgadywana liczba jest niższa czy wyższa od szyfru. Każdy szyfr można zgadnąć w `ceil(log n)` ruchach (jest to zasadniczo wyszukiwanie binarne).

#### Sterowanie budynkiem
Opcja `building` daje nam opcję sterowania budynkiem.

**Parametry:** 
* `--file ścieżkaDoPliku` pozwala nam otworzyć plik w formacie JSON.
