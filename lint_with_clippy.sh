#!/bin/sh
# lint the project using clippy
cargo clippy --all -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -A clippy::too-many-lines -A clippy::multiple_crate_versions -A clippy::cast-possible-wrap -D warnings
