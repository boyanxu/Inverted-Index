mod parser;
mod indexer;
mod utils;
mod disk_io;
mod external_sorter;
mod bin_indexer;
mod term_query_processor;

use std::fs;
use std::path::Path;
use disk_io::{process_gzip_file, merge_sorted_postings};
use bin_indexer::build_bin_index;
use crate::term_query_processor::TermQueryProcessor;

// Function to clean up the postings_data folder
fn cleanup_postings_data_folder() -> std::io::Result<()> {
    let dir = Path::new("postings_data");
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }
    Ok(())
}

fn main() {

    /* Assignment 2: Build the Inverted Index

    if let Err(e) = cleanup_postings_data_folder() {
        eprintln!("Error cleaning up postings_data folder: {}", e);
    }

    let file_path = "data/msmarco-docs.trec.gz";
    if let Err(e) = process_gzip_file(file_path) {
        eprintln!("Error processing file: {}", e);
    }

    // After processing the file, apply the external merge sort on the batches
    if let Err(e) = merge_sorted_postings() {
        eprintln!("Error merging sorted postings: {}", e);
    }

    // Build binary inverted index and store in 'data/' directory
    if let Err(e) = build_bin_index("data/merged_postings.data", "data/bin_index.data",
                                    "data/bin_lexicon.data", "data/bin_directory.data") {
        eprintln!("Error building binary inverted index: {}", e);
    }

    */

    // Query processor
    let term = "box";
    let mut tqp = TermQueryProcessor::new("data/bin_index.data", "data/bin_lexicon.data", "data/bin_directory.data",
                                          "data/doc_metadata.data");
    match tqp.query_term_all_postings(term) {
        Ok(postings) => {
            println!("Postings for term '{}': {:?}", term, postings);
        },
        Err(e) => {
            eprintln!("Error querying term: {}", e);
        }
    }

    tqp.conjunctive_query("developed cool");
}