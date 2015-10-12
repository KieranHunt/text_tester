#! /bin/bash

cargo build > /dev/null;

for i in {1..20}
    do
    for algorithm in BoyerMoore Horspool KnuthMorrisPratt Naive RabinKarp Bitap Bloom
    do
        for book in $(find /mnt/datapool/kieran/books -name '*.txt');
        do
            echo "cargo run $algorithm $book >> "/mnt/datapool/kieran/results/books/$algorithm-$i-`echo $book | rev | cut -d/ -f1 | rev`";"
            cargo run $algorithm $book >> "/mnt/datapool/kieran/results/books/$algorithm-$i-`echo $book | rev | cut -d/ -f1 | rev`";
        done
    done
done
