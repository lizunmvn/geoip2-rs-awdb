<div align="center">
	<h1>GeoIP2 Reader for Rust</h1>
	<p>
		<strong>This library reads MaxMind GeoIP2 databases</strong>
	</p>

[![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/geoip2-awdb.svg)](https://crates.io/crates/geoip2-awdb)


</div>

## New Ability

Add ability to read .awdb

## Usage

```toml
[dependencies]
geoip2-awdb = "0.2.0"
```

See [examples/lookup.rs](examples/lookup.rs) for a basic example.

## Benchmarks

Benchmarks required `nightly` Rust.

Place `GeoIP2-Country.mmdb` and `GeoIP2-City.mmdb` in the `testdata` folder, then run:
```
cargo bench
```
Same as '.awdb' file

Tested on paid DB on cargo 1.56.0-nightly, Intel i7-7700, Debian 9.1.

### [IncSW/geoip2-rs](https://github.com/IncSW/geoip2-rs)
`default`
```
city      2,175 ns/iter (+/- 124)
country   1,123 ns/iter (+/- 111)
```
`unsafe-str`
```
city      1,113 ns/iter (+/- 76)
country     524 ns/iter (+/- 31)
```

### [oschwald/maxminddb-rust](https://github.com/oschwald/maxminddb-rust).
`default`
```
city      4,224 ns/iter (+/- 153)
country   2,270 ns/iter (+/- 158)
```
`unsafe-str-decode`
```
city      3,266 ns/iter (+/- 191)
country   1,802 ns/iter (+/- 75)
```

## License

[MIT License](LICENSE).
