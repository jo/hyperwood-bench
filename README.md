# Hyperwood Bench
Our simplistic yet elegant bench is the first-ever Hyperwood design, embodying the project's essence of simplicity, functionality, and aesthetic clarity.

## Usage
```
Usage: hyperwood-bench --width <WIDTH> --depth <DEPTH> --height <HEIGHT> --variant <VARIANT>

Options:
      --width <WIDTH>      The width of the bench
      --depth <DEPTH>      The depth of the bench
      --height <HEIGHT>    The height of the bench. Must be an integer, and greater than 3
      --variant <VARIANT>  Provide the slat variant to use. Specify each three dimension, eg 0.06x0.04x0.06
  -h, --help               Print help
  -V, --version            Print version
```

## Example
```
$ hyperwood-bench --variant 0.06x0.04x0.06 --width 17 --height 6 --depth 9
Hyperwood Exchange Format
Version 1
hyperwood.org
Bench
{"width":17,"depth":9,"height":6}
{"x":0.06,"y":0.04,"z":0.06}
{"width":1.02,"depth":0.35999998,"height":0.35999998}
4
Keel
Seat
Shelf
Leg
3 4 1 11 0 0 4 0
0 0 6 17 0 0 0 1
0 2 6 17 0 0 2 1
0 4 6 17 0 0 4 1
0 6 6 17 0 0 6 1
0 8 6 17 0 0 8 1
2 2 2 13 0 0 2 2
2 4 2 13 0 0 4 2
2 6 2 13 0 0 6 2
3 1 0 0 0 6 1 3
14 1 0 0 0 6 1 3
3 3 1 0 0 5 3 3
14 3 1 0 0 5 3 3
3 5 1 0 0 5 5 3
14 5 1 0 0 5 5 3
3 7 0 0 0 6 7 3
14 7 0 0 0 6 7 3
```
