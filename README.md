HystEEE
=======

[![Build Status](https://travis-ci.org/migrax/HystEEE.svg?branch=master)](https://travis-ci.org/migrax/HystEEE)

A Rustified Simulator for 10Gb/s EEE with Configurable Hysteresis

## Overview

This is a new 10 Gb/s EEE simulator adapted to the behavior of current comercial equipment.

The program is able to simulate both setup hysteresis time and constant delay
before entering LPI mode.

## USAGE:
    eee-hyst [OPTIONS] [INPUT]

### FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

### OPTIONS:
    -l, --log <LOG>          Log output filename, if present.
    -o, --output <OUTPUT>    Traffic output file. Same format as INPUT. Uses
                             stdout if not present.
    -d, --delay <DELAY>      Time since first scheduled packet in LPI until
                             resuming normal mode in µs [default: 0]
    -h, --hyst <HYST>        Time before entering LPI in µs [default: 0]

### ARGS:
    <INPUT>    Traffic input file to use. Format "time (s) length (bytes)".
               Use '-' for stdin.

## Legal

Copyright ⓒ 2017–2018 Miguel Rodríguez Pérez <miguel@det.uvigo.gal>.

This simulator is licensed under the GNU General Public License, version 3 (GPL-3.0). For information see LICENSE.txt
