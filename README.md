# Test Format implementations

[![Build Status][build-image]][build-link]
[![Safety Dance][safety-image]][safety-link]
[![codecov][codecov-image]][codecov-link]
![MSRV][msrv-image]
[![EUPL 1.2 licensed][license-eupl-image]][license-eupl-link]
[![Apache 2.0 licensed][license-apache-image]][license-apache-link]
[![MIT licensed][license-mit-image]][license-mit-link]

Test `Debug` implementations on `no_std` targets, or `Debug` and `Display` on `std` via
`assert_debug_fmt!` and `assert_display_fmt!` macros:

```rust
assert_debug_fmt!(your_type, "expected debug format output");
assert_display_fmt!(your_type, "expected display format output");
```

## Requirements

- Rust 1.64+

## Code of Conduct

We abide by the [Contributor Covenant][cc] and ask that you do as well.

## License

Copyright © 2024 Markus Mayer

Triple licensed under your choice of either of:

- European Union Public Licence, Version 1.2, ([LICENSE-EUPL](LICENSE-EUPL)
  or https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[//]: # (badges)

[build-image]: https://github.com/sunsided/test-format/workflows/Rust/badge.svg

[build-link]: https://github.com/sunsided/test-format/actions

[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg

[safety-link]: https://github.com/rust-secure-code/safety-dance/

[msrv-image]: https://img.shields.io/badge/rustc-1.64+-blue.svg

[license-eupl-image]: https://img.shields.io/badge/license-EUPL_1.2-blue.svg

[license-apache-image]: https://img.shields.io/badge/license-Apache_2.0-blue.svg

[license-mit-image]: https://img.shields.io/badge/license-MIT-blue.svg

[license-apache-link]: https://github.com/sunsided/hardware-registers/blob/develop/LICENSE-APACHE

[license-mit-link]: https://github.com/sunsided/hardware-registers/blob/develop/LICENSE-MIT

[license-eupl-link]: https://github.com/sunsided/hardware-registers/blob/develop/LICENSE-EUPL

[codecov-image]: https://codecov.io/gh/sunsided/test-format/graph/badge.svg?token=hpv93Da1CU

[codecov-link]: https://codecov.io/gh/sunsided/test-format

[cc]: https://contributor-covenant.org
