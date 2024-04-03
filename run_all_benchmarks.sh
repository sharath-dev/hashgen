#! /usr/bin/bash

echo $'Running benchmarks to generate 1 GB file containing Blake3 Hashes'
./benchmark_1_gb.sh

echo $'\nRunning benchmarks to generate 64 GB file containing Blake3 Hashes'
./benchmark_64_gb.sh

echo $'\nRunning benchmarks to generate 64 GB file containing Blake3 Hashes with varying memory'
./benchmark_64_gb_variable_memory.sh 