![Abscissa](https://www.iqlusion.io/img/abscissa.svg)

# abscissa_derive: custom derive macros

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
[![Build Status][build-image]][build-link]
[![Appveyor Status][appveyor-image]][appveyor-link]

[crate-image]: https://img.shields.io/crates/v/abscissa_derive.svg
[crate-link]: https://crates.io/crates/abscissa_derive
[docs-image]: https://docs.rs/abscissa_derive/badge.svg
[docs-link]: https://docs.rs/abscissa_derive/
[build-image]: https://circleci.com/gh/iqlusioninc/crates.svg?style=shield
[build-link]: https://circleci.com/gh/iqlusioninc/crates
[appveyor-image]: https://ci.appveyor.com/api/projects/status/1ua33q2njho24e9h?svg=true
[appveyor-link]: https://ci.appveyor.com/project/tony-iqlusion/crates
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/master/LICENSE

This crate provides the custom derive implementations used by the
[abscissa] command-line app microframework.

It's designed to be a one-stop shop, providing all proc macros used by the
framework itself in a single crate. This ensures that proc macro upgrades
can be performed atomically (i.e. this won't ever depend on 3 versions of
`syn`), and minimizes the amount of code running as part of the build process.

[abscissa]: https://github.com/iqlusioninc/crates/tree/master/abscissa

## Features

This crate provides custom derive for the following:

* **Fail**: error trait (using the same API as [failure_derive])
* **Options**: command-line parsing (using the same API as [gumdrop_derive])

[failure_derive]: https://github.com/withoutboats/failure_derive
[gumdrop_derive]: https://github.com/murarth/gumdrop

## License

The **abscissa_derive** crate is distributed under the terms of the
Apache License (Version 2.0). It is a fork of the [failure_derive]
and [gumdrop_derive] crates, which also also distributed under the
terms of the Apache License (Version 2.0).

See [LICENSE] file in the `iqlusioninc/crates` toplevel directory for more
information.

[LICENSE]: https://github.com/iqlusioninc/crates/blob/master/LICENSE
