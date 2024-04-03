#! /usr/bin/bash

FILENAME=benchmark_results_1_gb.txt

rm $FILENAME

for hash_threads in 1 4 16
do
    for sort_threads in 1 4 16
    do
        for write_threads in 1 4 16
        do
            ./hashgen -t $hash_threads -o $sort_threads -i $write_threads -f data.bin -m 128 -s 1024 -d false | tee -a $FILENAME
        done
    done
done
