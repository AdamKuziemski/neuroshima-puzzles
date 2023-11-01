# Neuroshima Toolkit
A collection of puzzles and tools for Neuroshima and other RPGs focused on hacking.

## Usage

### Cargo
In you console, type:
```bash
cargo run -- [option] [parameters]
```

### Compiled application
```bash
neuroshima-toolkit [option] [parameters]
```

### Options

#### Digital lock
The `lock` option opens a simulator of a digital lock (usually a keypad).

The PC has to guess the code below a certain number of steps. The lock tells the PC if their guess is a higher or a lower number than the code. Every code can be guessed in `ceil(log n)` tries (it's essentially a binary search).

###### Parameters:
* `--digits [number from 2 to 7]` (short: `-d [number]`) - default: 3
* `--code [lock code]` (short: `-c [code]`) - by default it's randomized, can't be longer than 7 digits

**Note:** if the actual code is shorter than the number of digits, it gets prefilled with zeroes to fill the length, e.g. if the code is 5 digits long and the app's random choice was 32, then the actual code is `00032` and that's what the PCs will need to input.

###### Examples:
* `neuroshima-toolkit lock --digits 3` will be a random lock with a three-digit code.
* `neuroshima-toolkit lock --code 1234` will be a four digit code: `1234`.

#### Building control
The `building` opens an interface that gives the PC control over specific devices on the network, which is defined in a json file (see [dummy_building.json](data/buildings//dummy_building.json)).

###### Controls
Use arrow keys to navigate, 

###### Parameters:
* `--file nameOfJSONBuildingConfigFile` opens the JSON file and presents it in the interface (you don't need to add the `.json` extension, just the file name).
