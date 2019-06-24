ut
----

ut is a command line tool to handle a unix timestamp.

### Installation

clone the repository and build it.

``` bash
$ git clone https://github.com/yoshihitoh/ut
$ cd ut
$ cargo build --release
$ ./target/relase ut --version
ut 0.1.0
```

### Usage
``` bash
ut 0.1.2
yoshihitoh <yoshihito.arih@gmail.com>


USAGE:
    ut [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -u, --utc        Use utc timezone.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --offset <OFFSET>    Use given value as timezone offset.

SUBCOMMANDS:
    generate    Generate unix timestamp with given options.
    help        Prints this message or the help of the given subcommand(s)
    parse       Parse a unix timestamp and print it in human readable format.
```

See also

- [generate(g)](#generate-a-unix-timestamp)
- [parse(p)](#parse-a-unix-timestamp)

#### Generate a unix timestamp

Generate a unix timestamp of the midnight of today.
``` bash
$ ut generate -b today
1560870000

# You can use `-p` option to show it in millisecond.
$ ut generate -b today -p ms
1560870000000
```

You can specify time deltas with `-d` option.
``` bash
# 3days, 12hours, 30minutes later from the midnight of today.
$ ut g -b today -d 3day -d 12hour -d 30minute
1561174200

# You can use short name on time unit.
$ ut g -b today -d 3d -d 12h -d 30min
1561174200
```

#### Parse a unix timestamp

Parse a unix timestamp and print it in human readable format.
``` bash
$ ut p $(ut g -b today)
2019-06-19 00:00:00 (+09:00)

# You can parse timestamp in milliseconds.
$ ut p -p ms $(ut g -b today -p ms -d 11h -d 22min -d 33s -d 444ms)
2019-06-19 11:22:33.444 (+09:00)
```

#### Change timezone

##### Local timezone
If you don't set timezone options, use local timezone.

In Japan(UTC+9):
``` bash
$ ut g --ymd 2019-06-24
1561302000

$ ut p 1561302000
2019-06-24 00:00:00 (+09:00)
```

You can use `-u` or `--utc` option to use UTC timezone.
``` bash
$ ut --utc p 1561302000
2019-06-23 15:00:00 (UTC)
```

You can use fixed timezone on any environment.
``` bash
# Generate PST timestamp
$ ut -o -8 g --ymd 2019-06-24
1561363200

# Parse as PST timestamp
$ ut -o -8 p 1561363200
2019-06-24 00:00:00 (-08:00)

# Parse as UTC timestamp
$ ut -o 0 p 1561363200
2019-06-24 08:00:00 (+00:00)
```

### TODO
- Add more information on README
