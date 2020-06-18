use std::collections::{HashMap};
use std::str;
use std::sync::{Arc, Mutex};
use std::io::prelude::*;
use rayon::prelude::*;
use estimation::codon_structs::*;
use bio_types::strand;
use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

use std::path::Path;
use std::fs::OpenOptions;

use model::variants::*;

pub enum VariantStats {
    VariantContigStats {
        variants: HashMap<i64, HashMap<Variant, Base>>,
//        nucfrequency: HashMap<i32, BTreeMap<char, BTreeSet<i64>>>,
//        variants_in_reads: HashMap<i64, BTreeMap<i32, String>>,
//        variant_abundances: HashMap<i32, BTreeMap<String, (f64, f64)>>,
        variant_count: Vec<f64>,
        depth: Vec<i32>,
//        indels: HashMap<i32, BTreeMap<String, BTreeSet<i64>>>,
//        genotypes_per_position: HashMap<usize, usize>,
//        mean_genotypes: f64,
        tid: i32,
        total_indels: usize,
        target_name: Vec<u8>,
        target_len: f64,
//        variations_per_n: usize,
//        total_variants: usize,
        coverage: f64,
        variance: f64,
        observed_contig_length: u32,
        num_covered_bases: i32,
        num_mapped_reads: u64,
        total_mismatches: u64,
        contig_end_exclusion: u64,
        min: f64,
        max: f64,
        method: String,
        regression: (f64, f64, f64),
        // Clusters hashmap:
        // Key = Position
        // Value = K: Variant, V: (DBSCAN Cluster, HAC Index/initial cluster)
//        clusters: HashMap<i32, BTreeMap<String, (i32, usize)>>,
//        clusters_mean: HashMap<i32, f64>,
    }
}

impl VariantStats {
    pub fn new_contig_stats(min: f64, max: f64,
                            contig_end_exclusion: u64) -> VariantStats {
        VariantStats::VariantContigStats {
            variants: HashMap::new(),
            tid: 0,
            variant_count: vec!(),
            depth: vec!(),
            total_indels: 0,
            target_name: vec!(),
            target_len: 0.0,
            coverage: 0.00,
            variance: 0.00,
            observed_contig_length: 0,
            num_covered_bases: 0,
            contig_end_exclusion: contig_end_exclusion,
            num_mapped_reads: 0,
            total_mismatches: 0,
            min: min,
            max: max,
            method: "".to_string(),
            regression: (0., 0., 0.),
        }
    }
}

pub trait VariantFunctions {
    fn setup(&mut self);

    fn len(&mut self) -> usize;

    fn add_contig(&mut self,
                  variant_map: Option<&mut HashMap<i64, HashMap<Variant, Base>>>,
                  tid: i32,
                  total_indels_in_contig: usize,
                  contig_name: Vec<u8>,
                  contig_len: usize,
                  sample_idx: usize,
                  coverages: Vec<f64>,
                  ups_and_downs: Vec<i32>);

    /// Perform linear regression between total mismacthes and read depth
    fn calc_error(&mut self, ani: f32) -> usize;

//    /// Filter out variants from potential sequencing or mapping errors
//    fn filter_variants(&mut self,
//                       min_variant_depth: usize,
//                       coverage_fold: f64);

    /// Replace reference variants with most dominant variant observed within the reads
    fn polish_contig(&mut self,
                     original_contig: &Vec<u8>,
                     output_prefix: &str);

    /// Perform dN/dS calculations based on read mapping using modified Jukes-Cantor method
    fn calc_gene_mutations(&mut self,
                           gff_map: &HashMap<String, Vec<bio::io::gff::Record>>,
                           ref_sequence: &Vec<u8>,
                           codon_table: &CodonTable);

//    /// Prints out variant info for current contig
//    fn print_variants(&mut self, ref_sequence: &Vec<u8>, stoit_name: &str);
}

impl VariantFunctions for VariantStats {
    fn setup(&mut self) {
        match self {
            VariantStats::VariantContigStats {
                ref mut variants,
                ref mut variant_count,
                ref mut depth,
                ref mut tid,
                ref mut total_indels,
                ref mut target_name,
                ref mut target_len,
                ref mut coverage,
                ref mut num_covered_bases,
                ref mut num_mapped_reads, ..
            } => {
                *variants = HashMap::new();
                *variant_count = Vec::new();
                *depth = vec!();
                *tid = 0;
                *total_indels = 0;
                *target_name = vec!();
                *target_len = 0.0;
                *coverage = 0.00;
                *num_covered_bases = 0;
                *num_mapped_reads = 0;
            }
        }
    }

    fn len(&mut self) -> usize {
        match self {
            VariantStats::VariantContigStats {
                ref mut variants,
                ..
            } => {
                variants.len()
            }
        }
    }

    fn add_contig(&mut self,
                  variant_map: Option<&mut HashMap<i64, HashMap<Variant, Base>>>,
                  target_id: i32,
                  total_indels_in_contig: usize,
                  contig_name: Vec<u8>,
                  contig_len: usize,
                  sample_idx: usize,
                  coverages: Vec<f64>,
                  ups_and_downs: Vec<i32>) {
        match self {
            VariantStats::VariantContigStats {
                ref mut variants,
                ref mut variant_count,
                ref mut depth,
                ref mut tid,
                ref mut total_indels,
                ref mut target_name,
                ref mut target_len,
                ref mut variance,
                ref mut coverage,
                ref mut method,
                ..
            } => {

                *tid = target_id;
                *total_indels = total_indels_in_contig;
                *target_name = contig_name;
                *target_len = contig_len as f64;
                *coverage = coverages[1] as f64;
                *variance = coverages[2] as f64;
                *method = method.to_string();
                *variants = match variant_map {
                    Some(map) => map.clone(),
                    _ => HashMap::new(),
                };

                let variant_count_safe = Arc::new(Mutex::new(vec![0.; ups_and_downs.len()]));
                *depth = vec![0; ups_and_downs.len()];
                let mut cumulative_sum = 0;
                for (pos, current) in ups_and_downs.iter().enumerate() {
                    cumulative_sum += *current;
                    depth[pos] = cumulative_sum;
                }

                // Calculate how many reads have variant at each position
                // to go into linear regression predicting read error rate
                depth.par_iter_mut().enumerate().for_each(|(pos, d)| {
                    let mut variant_count_safe = variant_count_safe.lock().unwrap();
                    let mut var_set = match variants.get(&(pos as i64)) {
                        Some(set) => set.to_owned(),
                        None => HashMap::new(),
                    };
                    // Add depth of variant to count file if variant is present
                    for (var, base) in var_set.iter_mut() {
                        let var_depth = match base.variant {
                            Variant::None => 0,
                            _ => base.depth.par_iter().sum(),
                        };
                        if var_depth != 0 {
                            variant_count_safe[pos] += var_depth as f64;
                        }
                    }
                });

                *variant_count = variant_count_safe.lock().unwrap().to_vec();

                debug!("new contig added {} with coverage {} and variance {}", tid, coverage, variance);
            }
        }
    }

    fn calc_error(&mut self, ani: f32) -> usize {
        match self {
            VariantStats::VariantContigStats {
                variant_count,
                depth,
                regression,
                target_len,
                ..
            } => {
                // convert depth to f64 for this to work
                let depth_64: Vec<f64> = depth.par_iter().map(|x| *x as f64).collect();
                let data = vec![("Y", variant_count.clone()), ("X", depth_64.clone())];

                let data = RegressionDataBuilder::new()
                    .build_from(data).expect("Unable to build regression from data");
                let formula = "Y ~ X";
                let model = FormulaRegressionBuilder::new()
                    .data(&data)
                    .formula(formula)
                    .fit()
                    .expect("Unable to fit data to formula");
                let parameters = model.parameters;
                let standard_errors = model.se.pairs();
                let pvalues = model.pvalues;
                debug!("Linear regression results: \n params {:?} \n se {:?} \n p-values {:?}",
                         parameters,
                         standard_errors,
                         pvalues.pairs());
                // return results as intercept, effect size, se
                *regression = (parameters.intercept_value as f64,
                               parameters.regressor_values[0] as f64,
                               standard_errors[0].1 as f64);

                if ani > 0. {
                    // calculate the minimum number of variant sites needed for specified ANI
                    // ANI is not divided by 100 here as it is already betwen 0 and 1
                    let variants_for_ani = (*target_len as f32 * (1. - ani)) as usize;
                    info!("Variants needed for ANI {}", variants_for_ani);

                    // Sort variant count vec descending
                    variant_count.sort_by(|a, b| b.partial_cmp(a).unwrap());

                    variant_count[variants_for_ani] as usize
                } else {
                    0
                }

            }
        }
    }

    fn polish_contig(&mut self,
                     original_contig: &Vec<u8>,
                     output_prefix: &str) {
        match self {
            VariantStats::VariantContigStats {
                ref mut variants,
                target_name,
                ..
            } => {
                let file_name = output_prefix.to_string()
                    + &"_polished.fna".to_owned();

                let file_path = Path::new(&file_name);

                // Open haplotype file or create one

                let mut file_open = OpenOptions::new().append(true).create(true)
                    .open(file_path).expect("No Read or Write Permission in current directory");
                file_open.write(b">").unwrap();
                file_open.write(target_name).unwrap();
                file_open.write(b"\n").unwrap();


                let mut contig = String::new();

                let mut variations = 0;
                let mut skip_n = 0;
                let mut skip_cnt = 0;
                // Generate the consensus genome by checking each variant
                // Variant has to be in more than 0.5 of population
                for (pos, base) in original_contig.iter().enumerate() {
                    if skip_cnt < skip_n {
                        skip_cnt += 1;
                    } else {
                        let mut max_var = Variant::None;
                        let mut max_abund = 0.0;
                        skip_n = 0;
                        skip_cnt = 0;
                        if variants.contains_key(&(pos as i64)){
                            let alleles = &variants[&(pos as i64)];
                            for (var, base) in alleles.iter() {
                                if base.freq[0] > max_abund {
                                    max_var = var.clone();
                                    max_abund = base.freq[0];
                                }
                            }
                            if max_abund >= 0.5 {
                                match max_var {
                                    Variant::Deletion(size) => {
                                        // Skip the next n bases but rescue the reference prefix
                                        skip_n = size - 1;
                                        skip_cnt = 0;
                                        variations += 1;
                                    },
                                    Variant::Insertion(alt) | Variant::Insertion(alt) => {
                                        // Insertions have a reference prefix that needs to be removed
                                        let removed_first_base = str::from_utf8(
                                            &alt[1..]).unwrap();
                                        contig = contig + removed_first_base;
                                        variations += 1;
                                    },
                                    Variant::None => {
                                        contig = contig + str::from_utf8(&[*base]).unwrap();
                                    },
                                    Variant::SNV(alt) => {
                                        contig = contig + str::from_utf8(&[alt]).unwrap();
                                        variations += 1;
                                    },
                                    _ => {
                                        contig = contig + str::from_utf8(&[*base]).unwrap();
                                    }
                                }
                            } else {
                                contig = contig + str::from_utf8(&[*base]).unwrap();
                            }
                        } else {
                            contig = contig + str::from_utf8(&[*base]).unwrap();
                        }
                    }
                };
//                contig = contig + "\n";
                for line in contig.as_bytes().to_vec()[..].chunks(60).into_iter(){
                    file_open.write(line).unwrap();
                    file_open.write(b"\n").unwrap();
                };
//                writeln!(file_open, "{:?}", contig);
            }
        }
    }

    fn calc_gene_mutations(&mut self,
                           gff_map: &HashMap<String, Vec<bio::io::gff::Record>>,
                           ref_sequence: &Vec<u8>,
                           codon_table: &CodonTable) {
        match self {
            VariantStats::VariantContigStats {
                variants,
                tid: _,
                target_name,
                depth,
                ..
            } => {
                let contig_name = String::from_utf8(target_name.clone())
                    .expect("Cannot create string from target_name");
                let placeholder = Vec::new();
                let gff_records = match gff_map.get(&contig_name){
                    Some(records) => records,
                    None => &placeholder,
                };
                debug!("Calculating population dN/dS from reads for {} genes", gff_records.len());
                gff_records.par_iter().enumerate().for_each(|(_id, gene)| {
                    let dnds = codon_table.find_mutations(gene, variants, ref_sequence, depth);
                    let strand = gene.strand().expect("No strandedness found");
                    let frame: usize = gene.frame().parse().unwrap();
                    let start = gene.start().clone() as usize - 1;
                    let end = gene.end().clone() as usize;

                    let strand_symbol = match strand {
                        strand::Strand::Forward | strand::Strand::Unknown => {
                            '+'.to_string()
                        },
                        strand::Strand::Reverse => {
                            '-'.to_string()
                        }

                    };
                    let gene_id = &gene.attributes()["ID"].split("_").collect::<Vec<&str>>()[1];
                    let mut contig = gene.seqname().to_owned();
                    contig = contig + "_";

//                    for cursor in start..end+1 {
//                        let variant_map = match variants.get(&(cursor as i64)){
//                            Some(map) => map,
//                            None => continue,
//                        };
//                        if variant_map.len() > 0 {
//                            let mut print_stream = print_stream.lock().unwrap();
//
//
//                            for variant in variant_map {
//                                if variant.to_owned().contains("R"){
//                                    continue
//                                }
//                                write!(print_stream, "{}\t{}\t{}\t{}\t{}\t{:.3}\t{}\t",
//                                         contig.clone()+gene_id, gene.start(),
//                                         gene.end(), frame, strand_symbol, dnds, cursor).expect("Unable to write to stream");
//
//
//                                if variant.to_owned().contains("N") {
//                                    writeln!(print_stream, "{}\t{}\t{}\t{}\t{}",
//                                           variant,
//                                           str::from_utf8(
//                                               &ref_sequence[cursor..cursor
//                                                   + variant.len() as usize]).unwrap(),
//                                           abundance.0, abundance.1, "D").expect("Unable to write to stream");
//
//                                } else if indel_map.contains_key(variant) {
//                                     writeln!(print_stream,"{}\t{}\t{:.3}\t{}\t{}",
//                                           variant,
//                                           str::from_utf8(
//                                               &[ref_sequence[cursor]]).unwrap(),
//                                           abundance.0, abundance.1, "I").expect("Unable to write to stream");
//                                } else {
//                                    writeln!(print_stream, "{}\t{}\t{}\t{}\t{}",
//                                             variant,
//                                             ref_sequence[cursor] as char,
//                                             abundance.0, abundance.1, "S").expect("Unable to write to stream");
//                                }
//                            }
//                        }
//                    }
                });

            }
        }

    }
}

// helper function to get the index of condensed matrix from it square form
#[allow(dead_code)]
fn condensed_index(i: usize, j: usize, n: usize) -> Option<usize>{
    if i == j {
        return None
    } else {
        return Some(n*i - i*(i+1)/2 + j - 1 - i)
//        return Some(n*(n-1)/2 - (n - row_i)*(n - row_i - 1)/2 + col_j - row_i - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use std::fs::File;
    use coverm::mapping_parameters::*;
    use coverm::shard_bam_reader::*;
    use coverm::genome_exclusion::*;
    use coverm::bam_generator::*;

//    fn test_with_stream<R: NamedBamReader + Send,
//        G: NamedBamReaderGenerator<R> + Send>(
//        expected: &str,
//        bam_readers: Vec<G>,
//        mut reference: bio::io::fasta::IndexedReader<File>,
//        proper_pairs_only: bool,
//        n_threads: usize,
//        coverage_fold: f32,
//        min_var_depth: usize,
//        min: f32,
//        max: f32,
//        mode: &str,
//        include_indels: bool,
//        include_soft_clipping: bool) {
////        let mut stream = Cursor::new(Vec::new());
//        {
//            reads_mapped_vec = pileup_variants(
//                bam_readers,
//                &mut coverage_taker,
//                coverage_estimators,
//                print_zero_coverage_contigs,
//                flag_filters,
//                false,
//                );
//        }
////        assert_eq!(expected, str::from_utf8(stream.get_ref()).unwrap());
//    }
}