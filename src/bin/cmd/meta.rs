
use log::*;
use preqc_pack::{fastqc, hasher};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use structopt::StructOpt;

use blake2::Blake2b;
use md5::Md5;

/// A collection of metadata, such as file size, md5sum
#[derive(StructOpt, PartialEq, Debug)]
#[structopt(setting=structopt::clap::AppSettings::ColoredHelp, name="PreQC Tool Suite - Hasher", author="Jingcheng Yang <yjcyxky@163.com>")]
pub struct Arguments {
  /// Bam file to process
  #[structopt(name = "FILE")]
  input: String,

  /// A hash algorithms for output file.
  #[structopt(name="algorithm", short="m", long="algorithm", possible_values=&["md5sum", "blake2b"], default_value="md5sum")]
  algorithm: String,

  /// Which module will be called.
  #[structopt(name="which", short="w", long="which", possible_values=&["checksum", "fastqc", "all"], default_value="all")]
  which: String,
}

#[derive(Serialize, Deserialize)]
struct QCPack {
  fastqc: fastqc::FastQC,
  filemeta: hasher::Meta,
}

fn checksum(input: &str, algorithm: &str) -> hasher::Meta {
  // Get filemeta
  let mut file = fs::File::open(input).unwrap();
  let meta = match algorithm {
    "blake2b" => hasher::process::<Blake2b, _>(&mut file),
    _ => hasher::process::<Md5, _>(&mut file),
  };
  meta
}

fn fastqc(input: &str) -> fastqc::FastQC {
  let mut fastqc_metrics = fastqc::init_fastqc(0);
  if preqc_pack::is_fastq_file(input) {
    // Generate fastqc metrics
    fastqc_metrics = fastqc::compute_data_size(input);
  } else if preqc_pack::is_fastq_gz_file(input) {
    // fastqc_metrics = fastqc::compute_gz_data_size(input);
    fastqc_metrics = fastqc::compute_data_size_par(input);
  } else {
    error!("Not a valid fastq/fastq.gz file")
  }

  fastqc_metrics
}

pub fn run(args: &Arguments) {
  if Path::new(&args.input).exists() {
    // TODO: Multi threads?
    let fastqc_metrics = fastqc::init_fastqc(0);
    let meta = hasher::init_meta();

    let mut qc_pack = QCPack {
      fastqc: fastqc_metrics,
      filemeta: meta,
    };

    if args.which == "checksum" {
      qc_pack.filemeta = checksum(&args.input, &args.algorithm);
    } else if args.which == "fastqc" {
      qc_pack.fastqc = fastqc(&args.input);
    } else {
      qc_pack.filemeta = checksum(&args.input, &args.algorithm);
      qc_pack.fastqc = fastqc(&args.input);
    }

    println!("{}", serde_json::to_string(&qc_pack).unwrap());
  } else {
    error!("{} - Not Found: {:?}", module_path!(), args.input);
  }
}
