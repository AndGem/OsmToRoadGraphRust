#!/bin/bash
set -xe
cargo build --verbose
wget https://download.geofabrik.de/europe/germany/bremen-latest.osm.pbf
target/release/osmtoroadgraph --verbose -n c bremen-latest.osm.pbf
