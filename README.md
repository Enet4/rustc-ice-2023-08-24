# rustc ICE reproducible

> `DefId(20:797 ~ dicom_encoding[e27c]::encode::EncodeTo) does not have a "object_lifetime_default"`

This shows a case where some versions of rustc fail to document a crate.

Versions of rustc known to be affected:

- 1.72.0
- 1.73.0-nightly (1d56e3a6d 2023-07-22)
- 1.74.0-nightly (249595b75 2023-08-23)

Stable 1.71.1 does not reproduce the problem.

Command:

```sh
cargo doc
```

Output: [rustc-ice-2023-08-24.txt]

Or, to test this against the original dependency `dicom_encoding`,
which produces a different top-level error message in the ICE,
enable Cargo feature `dicom-encoding`:

```sh
cargo doc --features dicom-encoding
```

Output with `dicom-encoding`: [rustc-ice-2023-08-24.dicom-encoding.txt]

Based on `dicom-parser` from the [DICOM-rs](https://github.com/Enet4/dicom-rs) project.
