ut
----

ut is a command line tool to handle a unix timestamp.

[![Latest Version](https://img.shields.io/crates/v/ut-cli.svg)](https://crates.io/crates/ut-cli)
![ci](https://github.com/yoshihitoh/ut-cli/workflows/ci/badge.svg)
![Dependabot](https://api.dependabot.com/badges/status?host=github&repo=yoshihitoh/ut-cli)

### Motivation
There is a number of times to generate/parse unix timestamps.
I think `date` command exists to handle these situations. But there are a few problems that they are small, but vital for me.
- cannot use same options between macOS and Linux.
- hard to remember usage. (it might be happen because of above problem.)

That's why I made a new command line tool `ut-cli`.

I hope ut-cli works well when developers need to use the command which requires timestamps like aws-cli.

### Example usage

Search logs from specific time period.
``` bash
# from yesterday to today
$ aws logs filter-log-events \
    --log-group-name <LOG_GROUP_NAME> \
    --log-stream-names <LOG_STREAM_NAMES> \
    --query <QUERY> \
    --start-time $(ut -p ms g -b yesterday) \
    --end-time $(ut -p ms g -b today)
```

### Installation

If you have rust toolchain, ut-cli can be installed with cargo.
``` bash
$ cargo install ut-cli
```

or clone the repository and build it.

``` bash
$ git clone https://github.com/yoshihitoh/ut-cli
$ cd ut-cli
$ cargo build --release
$ ./target/release/ut --version
ut 0.1.6
```

Also there are pre-built binary for Linux and macOS.
See [releases](https://github.com/yoshihitoh/ut-cli/releases).

### Usage
``` bash
ut-cli 0.1.6
yoshihitoh <yoshihito.arih@gmail.com>
A command line tool to handle unix timestamp.

USAGE:
    ut [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -u, --utc        Use utc timezone.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --offset <OFFSET>          Use given value as timezone offset.
    -p, --precision <PRECISION>
            Set the precision of output timestamp.


SUBCOMMANDS:
    generate    Generate unix timestamp with given options.
    help        Prints this message or the help of the given subcommand(s)
    parse       Parse a unix timestamp and print it in human readable format.
```

You can set options via envrionment variables.

| name         | equiv option   | example 
|:------------:|:--------------:|:-----------
| UT_OFFSET    | -o/--offset    | 09:00
| UT_PRECISION | -p/--precision | millisecond

```bash
# set variables
$ export UT_OFFSET='09:00'
$ export UT_PRECISION=millisecond

# run command without `-o` and `-p` option
$ ut p $(ut g)
```

is equivalent to

```bash
$ ut -o '09:00' -p millisecond p $(ut -o '09:00' -p millisecond g)
```


There are two subcommands available for now.
- [generate(g)](#generate-a-unix-timestamp)
- [parse(p)](#parse-a-unix-timestamp)

#### Generate a unix timestamp

Generate a unix timestamp of the midnight of today.
``` bash
$ ut generate -b today
1560870000

# You can use `-p` option to show it in millisecond.
$ ut -p ms generate -b today
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

# You can modify a timestamp with a timestamp argument.
$ ut g -d 1min 1561174200
1561174260    # 1min(=60second) difference.
```

#### Parse a unix timestamp

Parse a unix timestamp and print it in human readable format.
``` bash
$ ut p $(ut g -b today)
2019-06-19 00:00:00 (+09:00)

# You can parse timestamp in milliseconds.
$ ut -p ms p $(ut -p ms g -b today -d 11h -d 22min -d 33s -d 444ms)
2019-06-19 11:22:33.444 (+09:00)
```

#### Change timezone

##### Local timezone
If you don't set timezone options, ut command uses local timezone.

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

You can use fixed offset timezone on any environment.
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
