mod fasta;
mod utils;
mod alignment;
mod algorithms;

use clap::{Parser, Subcommand};

/// Multiple sequence alignment using minimum spanning trees
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Reference implementation for Gusfield's 2-approximation algorithm
    Ref {
        /// FASTA files
        #[clap(group = "input", value_parser)]
        records: String,
        /// Substitution matrix
        #[clap(short, long, value_parser)]
        submat: String,
        /// Should we maximize the cost ?
        #[clap(short, long, value_parser, default_value_t = false)]
        maximize: bool,
    },

    /// Gusfield's 2-approximation algorithm using minimum spanning trees
    Mst {
        // Some additional args can be added in the future,
        // specific to this subcommand (e.g. the order we choose).
        /// FASTA files
        #[clap(group = "input", value_parser)]
        records: String,
        /// Substitution matrix
        #[clap(short, long, value_parser)]
        submat: String,
        /// Should we maximize the cost ?
        #[clap(short, long, value_parser, default_value_t = false)]
        maximize: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Ref {
            records,
            submat,
            maximize,
        } => {
            println!(
                "we should process the 'ref' subcommand with parameters: {:?},{:?},{:?}",
                records, submat, maximize
            );
            let sm = utils::read_submatrix_file(submat);
            for (key, value) in sm.iter() {
                println!("{}:{:?}", key, value);
            }
            let records = fasta::read_fasta_file(records);
            for record in &records {
                println!("{}", record);
            }
            let alignment = alignment::pairwise_alignment(
                &records[0],
                &records[1],
                &sm,
                5,
                *maximize,
            ).expect("pairwise alignment failed");
            println!("Cost of alignment ({}, {}) = {}", alignment.seq1.name, alignment.seq2.name, alignment.score);
            println!("Sequence1: {}", alignment.seq1.sequence);
            println!("Sequence2: {}", alignment.seq2.sequence);


        }
        Commands::Mst {
            records,
            submat,
            maximize,
        } => {
            println!(
                "we should process the 'mst' subcommand with parameters: {:?},{:?},{:?}",
                records, submat, maximize
            );
        }
    }
}
