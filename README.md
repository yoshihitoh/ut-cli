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

### TODO
- Add more information on README
- CI/CD
