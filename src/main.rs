extern crate dpi_algorithms;
extern crate time;

use dpi_algorithms::ruleset::ruleset::Ruleset;
use dpi_algorithms::dpi_interface::algorithm::Algorithm;
use dpi_algorithms::dpi_interface::ahocorasick::AhoCorasick;
use dpi_algorithms::dpi_interface::bitap::Bitap;
use dpi_algorithms::dpi_interface::bloomfilter::BloomFilter;
use dpi_algorithms::dpi_interface::boyermoore::BoyerMoore;
use dpi_algorithms::dpi_interface::cuckoofilter::Cuckoo;
use dpi_algorithms::dpi_interface::horspool::Horspool;
use dpi_algorithms::dpi_interface::knuthmorrispratt::KnuthMorrisPratt;
use dpi_algorithms::dpi_interface::naive::Naive;
use dpi_algorithms::dpi_interface::rabinkarp::RabinKarp;
use dpi_algorithms::ruleset::ruleset::Algorithms;
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env::args;
use std::error::Error;
use std::env;
use time::{Duration, PreciseTime};


fn main() {
    //TODO: Make this redirectable from stdin.

    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        panic!("Incorrect number of arguments: {}", args.len());
    }

    let path = Path::new(&args[2]);

    let display = path.display();
    let mut f = File::open(path).unwrap();
    let mut text = [0; 1000000];
    f.read(&mut text);

    let ruleset = Ruleset::new(Algorithms::Naive, "google".to_string());

    let mut algorithm_ahocorasick = AhoCorasick::new(&ruleset);
    let mut algorithm_bitap = Bitap::new(&ruleset);
    let mut algorithm_bloom = BloomFilter::new(&ruleset);
    let mut algorithm_boyermoore = BoyerMoore::new(&ruleset);
    let mut algorithm_cuckoo = Cuckoo::new(&ruleset);
    let mut algorithm_horspool = Horspool::new(&ruleset);
    let mut algorithm_knuthmorrispratt = KnuthMorrisPratt::new(&ruleset);
    let mut algorithm_naive = Naive::new(&ruleset);
    let mut algorithm_rabinkarp = RabinKarp::new(&ruleset);

    let sub_start = PreciseTime::now();
    match args[1].to_string().as_ref() {
        "AhoCorasick" => {algorithm_ahocorasick.inspect_data(&text);},
        "Bitap" => {algorithm_bitap.inspect_data(&text);},
        "Bloom" => {algorithm_bloom.inspect_data(&text);},
        "BoyerMoore" => {algorithm_boyermoore.inspect_data(&text);},
        "Cuckoo" => {algorithm_cuckoo.inspect_data(&text);},
        "Horspool" => {algorithm_horspool.inspect_data(&text);},
        "KnuthMorrisPratt" => {algorithm_knuthmorrispratt.inspect_data(&text);},
        "Naive" => {algorithm_naive.inspect_data(&text);},
        "RabinKarp" => {algorithm_rabinkarp.inspect_data(&text);},
        _ => {panic!("Algorithm not implemented!")},
    }
    let sub_stop = PreciseTime::now();

    let elapsed_time_micro = sub_start.to(sub_stop).num_microseconds().unwrap();

    println!("{}", elapsed_time_micro);

}
