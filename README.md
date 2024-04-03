[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/C5s9grq-)
### CS553 Cloud Computing Assignment 4 Repo
**Illinois Institute of Technology**  

**Students**:  
* Sharath Devasahayam (fsharathdevasahayam@hawk.iit.edu) 

### Directory Contents
This directory contains the final report named as ```hw4-report.pdf``` which contains all results in a table format. It contains the summarized version of the findings in terms of scalability, concurrency, and performance. 

The source code is located in the ```src``` directory.
* ```main.rs``` is the script for the hashgen program. 

The repository also contains 4 scripts which run the benchmarks for a 1 GB workload, 64 GB workload, 64 GB workload with varying memory limits, and a final comprehensive script which runs all 57 experiments.
* ```benchmark_1_gb.sh``` runs the 27 benchmarks for the 1 GB workload.
* ```benchmark_64_gb.sh``` runs the 27 benchmarks for the 64 GB workload.
* ```benchmark_64_gb_variable_memory.sh``` runs 3 benchmarks for the 64 GB workload with memory limits set to 1024 MB, 8192 MB, and 32768 MB respectively.
* ```run_all_benchmarks.sh``` runs all of the above 3 benchmarks.

The raw output logs are located in the ```benchmark_results_1_gb.txt```, ```benchmark_results_64_gb.txt```, and ```benchmark_results_variable_memory.txt``` 
* ```benchmark_results_1_gb.txt``` contains the results for the 1 GB workload.
* ```benchmark_results_64_gb.txt``` contains the results for the 64 GB workload.
* ```benchmark_results_variable_memory.txt``` contains the results for the 64 GB workload with 1024 MB, 8192 MB, and 32768 MB.

#### Building the binary

To build the ```hashgen``` binary run the following command
```
./build.sh
```
#### Running the hashgen binary
##### Help command
```./hashgen -h``` returns the following
```
Usage: hashgen [OPTIONS] --filename <FILENAME> -t <NUM_THREADS_HASH> -o <NUM_THREADS_SORT> -i <NUM_THREADS_WRITE> --memory-size <MEMORY_SIZE> -s <FILESIZE>

Options:
  -f, --filename <FILENAME>        Specify the filename
  -t <NUM_THREADS_HASH>            Specify Number of Hash Threads
  -o <NUM_THREADS_SORT>            Specify the number of Sort Threads
  -i <NUM_THREADS_WRITE>           Specify the number of Write Threads
  -m, --memory-size <MEMORY_SIZE>  Specify the maximum amount of memory to be used in MB
  -s <FILESIZE>                    Specify the filesize in MB
  -d, --debug <DEBUG>              Turns on debug mode with true, off with false
  -h, --help                       Print help
  -V, --version                    Print version
```

##### Example Command
To run the hashgen command with 16 threads for generating hashes, sorting, and writing a 1024 MB file named ```data.bin``` with a memory limit of 128 MB is
```
./hashgen -t 16 -o 16 -i 16 -f data.bin -m 128 -s 1024 -d false
```

The output will look similar to this
```
hashgen t16 o16 i16 m128 s1024 1.387571727s 1.6189522s 47.713770549s 50.723314106s
```

The debug flag can be used to run the program in debug mode which displays the ETA, current ongoing process, and the memory being used.
```
./hashgen -t 16 -o 16 -i 16 -f data.bin -m 128 -s 1024 -d true  
```

The output will look similar to this
```
NUM_THREADS_HASH=16
NUM_THREADS_SORT=16
NUM_THREADS_WRITE=16
FILENAME=data.bin
MEMORY_SIZE=128MB
FILESIZE=1024MB
RECORD_SIZE=16B
HASH_SIZE=10B
NONCE_SIZE=6B
BUCKET_SIZE=128MB
[00:00:00] ######----------------------------------       1/8        [ETA:00:00:00]
Hashing used 128.06154 MB of RAM.
Sorting used 128.15591 MB of RAM.
Writing used 128.16972 MB of RAM.
[00:00:01] ###########-----------------------------       2/8       Writing [ETA:00:00:02]
Hashing used 128.16907 MB of RAM.
Sorting used 128.11807 MB of RAM.
Writing used 128.12173 MB of RAM.
[00:00:03] ################------------------------       3/8       Writing [ETA:00:00:03]
Hashing used 128.14839 MB of RAM.
Sorting used 128.11197 MB of RAM.
Writing used 128.14526 MB of RAM.
[00:00:04] #####################-------------------       4/8       Writing [ETA:00:00:03]
Hashing used 128.16156 MB of RAM.
Sorting used 128.116 MB of RAM.
Writing used 128.11096 MB of RAM.
[00:00:05] ##########################--------------       5/8       Writing [ETA:00:00:02]
Hashing used 128.17902 MB of RAM.
Sorting used 128.14622 MB of RAM.
Writing used 128.12952 MB of RAM.
[00:00:06] ###############################---------       6/8       Writing [ETA:00:00:01]
Hashing used 128.16022 MB of RAM.
Sorting used 128.10965 MB of RAM.
Writing used 128.14603 MB of RAM.
[00:00:08] ####################################----       7/8       Writing [ETA:00:00:01]
Hashing used 128.12154 MB of RAM.
Sorting used 128.10767 MB of RAM.
Writing used 128.11823 MB of RAM.
[00:00:09] ########################################       8/8       Writing [ETA:00:00:00]
Hashing used 128.14279 MB of RAM.
Sorting used 128.11693 MB of RAM.
Writing used 128.13339 MB of RAM.
hashgen t16 o16 i16 m128 s1024 867.796902ms 1.155451189s 7.941404996s 9.970400917s
The max amount that was used 128.23059 MB
```

