#!/bin/bash
set -xe
cargo test --verbose
cargo build --release
wget https://download.geofabrik.de/europe/germany/bremen-latest.osm.pbf
target/debug/osmtoroadgraph --verbose -n c bremen-latest.osm.pbf
