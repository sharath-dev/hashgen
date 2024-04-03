#! /usr/bin/bash

FILENAME=benchmark_results_variable_memory.txt

rm $FILENAME

for memory_size in 1024 8192 32768
do
    ./hashgen -t 16 -o 16 -i 16 -f data.bin -m $memory_size -s 65536 -d false | tee -a $FILENAME
done