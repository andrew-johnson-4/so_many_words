So Many Words!!
================================

[![Crates.IO](https://img.shields.io/crates/v/so_many_words.svg)](https://crates.rs/crates/so_many_words)
[![Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/so_many_words/)
[![Build Nightly](https://github.com/andrew-johnson-4/so_many_words/workflows/BuildNightly/badge.svg)](https://github.com/andrew-johnson-4/so_many_words)
[![Build](https://github.com/andrew-johnson-4/so_many_words/workflows/Build/badge.svg)](https://github.com/andrew-johnson-4/so_many_words)

This writes a lot of words, not so much for reading. Maybe this could eventually be useful for directed automatic translation.

```rust
cargo run --bin tokenize [input]
cargo run --bin stem [language] [input]
cargo run --bin detect [input]
cargo run --bin eudex [input]
cargo run --bin build_phoneme [language] [input]
cargo run --bin search_phoneme [terms]
```

Partial support for these language: Arabic, Danish, Dutch, English, French, German
Greek, Hungarian, Italian, Norwegian, Portuguese, Romanian, Russian, Spanish, Swedish
Tamil, Turkish

If you would like to contribute to this project or more generally to any Open NLP
Project then checkout my [TODO](https://www.subarctic.org/finding_ways_to_help_out_other_open_nlp_projects.html)
page for open issues.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in so_many_words by you,
shall be dual licensed under the MIT and Apache 2.0 license without any additional terms or conditions.
