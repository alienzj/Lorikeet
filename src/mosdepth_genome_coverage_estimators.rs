use coverage_takers::CoverageTaker;

#[derive(Clone, Debug)]
pub enum CoverageEstimator {
    MeanGenomeCoverageEstimator{
        pileup_counts: Vec<i32>,
        total_count: u32,
        total_bases: u32,
        num_covered_bases: u32,
        num_mapped_reads: u64,
        total_mismatches: u32,
        min_fraction_covered_bases: f32,
        contig_end_exclusion: u32,
        exclude_mismatches: bool,
    },
    TrimmedMeanGenomeCoverageEstimator {
        pileup_counts: Vec<i32>,
        counts: Vec<u32>,
        observed_contig_length: u32,
        num_covered_bases: u32,
        num_mapped_reads: u64,
        min: f32,
        max: f32,
        min_fraction_covered_bases: f32,
        contig_end_exclusion: u32,
    },
    PileupCountsGenomeCoverageEstimator {
        pileup_counts: Vec<i32>,
        counts: Vec<u32>,
        observed_contig_length: u32,
        num_covered_bases: u32,
        num_mapped_reads: u64,
        min_fraction_covered_bases: f32,
        contig_end_exclusion: u32,
    },
    CoverageFractionGenomeCoverageEstimator {
        pileup_counts: Vec<i32>,
        total_bases: u32,
        num_covered_bases: u32,
        num_mapped_reads: u64,
        min_fraction_covered_bases: f32,
        contig_end_exclusion: u32,
    },
    NumCoveredBasesCoverageEstimator {
        pileup_counts: Vec<i32>,
        total_bases: u32,
        num_covered_bases: u32,
        num_mapped_reads: u64,
        min_fraction_covered_bases: f32,
        contig_end_exclusion: u32,
    },
    VarianceGenomeCoverageEstimator {
        pileup_counts: Vec<i32>,
        counts: Vec<u32>,
        observed_contig_length: u32,
        num_covered_bases: u32,
        num_mapped_reads: u64,
        min_fraction_covered_bases: f32,
        contig_end_exclusion: u32,
    },
    ReferenceLengthCalculator {
        observed_contig_length: u32,
        num_mapped_reads: u64
    },
    ReadCountCalculator {
        num_mapped_reads: u64,
    },
    ReadsPerBaseCalculator {
        observed_contig_length: u32,
        num_mapped_reads: u64
    },
}

impl CoverageEstimator {
    pub fn column_headers(&self) -> Vec<&str> {
        match self {
            CoverageEstimator::MeanGenomeCoverageEstimator{..} => {vec!("Mean")},
            CoverageEstimator::TrimmedMeanGenomeCoverageEstimator{..} => {vec!("Trimmed Mean")},
            CoverageEstimator::PileupCountsGenomeCoverageEstimator{..} => {vec!("Coverage","Bases")},
            CoverageEstimator::CoverageFractionGenomeCoverageEstimator{..} => {vec!("Covered Fraction")},
            CoverageEstimator::NumCoveredBasesCoverageEstimator{..} => {vec!("Covered Bases")},
            CoverageEstimator::VarianceGenomeCoverageEstimator{..} => {vec!("Variance")},
            CoverageEstimator::ReferenceLengthCalculator{..} => vec!("Length"),
            CoverageEstimator::ReadCountCalculator{..} => vec!("Read Count"),
            CoverageEstimator::ReadsPerBaseCalculator{..} => vec!("Reads per base"),
        }
    }
}

impl CoverageEstimator {
    pub fn new_estimator_mean(
        min_fraction_covered_bases: f32,
        contig_end_exclusion: u32,
        exclude_mismatches: bool)
        -> CoverageEstimator {
        CoverageEstimator::MeanGenomeCoverageEstimator {
            pileup_counts: vec!(),
            total_count: 0,
            total_bases: 0,
            num_covered_bases: 0,
            num_mapped_reads: 0,
            total_mismatches: 0,
            min_fraction_covered_bases: min_fraction_covered_bases,
            contig_end_exclusion: contig_end_exclusion,
            exclude_mismatches: exclude_mismatches,
        }
    }
    pub fn new_estimator_trimmed_mean(
        min: f32, max: f32, min_fraction_covered_bases: f32,
        contig_end_exclusion: u32)
        -> CoverageEstimator {
        CoverageEstimator::TrimmedMeanGenomeCoverageEstimator {
            pileup_counts: vec!(),
            counts: vec!(),
            observed_contig_length: 0,
            num_covered_bases: 0,
            num_mapped_reads: 0,
            min_fraction_covered_bases: min_fraction_covered_bases,
            min: min,
            max: max,
            contig_end_exclusion: contig_end_exclusion,
        }
    }
    pub fn new_estimator_pileup_counts(
        min_fraction_covered_bases: f32,
        contig_end_exclusion: u32)
        -> CoverageEstimator {
        CoverageEstimator::PileupCountsGenomeCoverageEstimator {
            pileup_counts: vec!(),
            counts: vec!(),
            observed_contig_length: 0,
            num_covered_bases: 0,
            num_mapped_reads: 0,
            min_fraction_covered_bases: min_fraction_covered_bases,
            contig_end_exclusion: contig_end_exclusion,
        }
    }
    pub fn new_estimator_covered_fraction(
        min_fraction_covered_bases: f32,
        contig_end_exclusion: u32)
        -> CoverageEstimator {
        CoverageEstimator::CoverageFractionGenomeCoverageEstimator {
            pileup_counts: vec!(),
            total_bases: 0,
            num_covered_bases: 0,
            num_mapped_reads: 0,
            min_fraction_covered_bases: min_fraction_covered_bases,
            contig_end_exclusion: contig_end_exclusion,
        }
    }
    pub fn new_estimator_covered_bases(
        min_fraction_covered_bases: f32,
        contig_end_exclusion: u32)
        -> CoverageEstimator {
        CoverageEstimator::NumCoveredBasesCoverageEstimator {
            pileup_counts: vec!(),
            total_bases: 0,
            num_covered_bases: 0,
            num_mapped_reads: 0,
            min_fraction_covered_bases: min_fraction_covered_bases,
            contig_end_exclusion: contig_end_exclusion,
        }
    }
    pub fn new_estimator_variance(
        min_fraction_covered_bases: f32,
        contig_end_exclusion: u32)
        -> CoverageEstimator {
        CoverageEstimator::VarianceGenomeCoverageEstimator {
            pileup_counts: vec!(),
            counts: vec!(),
            observed_contig_length: 0,
            num_covered_bases: 0,
            num_mapped_reads: 0,
            min_fraction_covered_bases: min_fraction_covered_bases,
            contig_end_exclusion: contig_end_exclusion,
        }
    }
    pub fn new_estimator_length() -> CoverageEstimator {
        CoverageEstimator::ReferenceLengthCalculator {
            observed_contig_length: 0,
            num_mapped_reads: 0,
        }
    }
    pub fn new_estimator_read_count() -> CoverageEstimator {
        CoverageEstimator::ReadCountCalculator {
            num_mapped_reads: 0
        }
    }
    pub fn new_estimator_reads_per_base() -> CoverageEstimator {
        CoverageEstimator::ReadsPerBaseCalculator {
            observed_contig_length: 0,
            num_mapped_reads: 0
        }
    }
}

pub trait MosdepthGenomeCoverageEstimator {

    fn setup(&mut self);

    fn add_contig(
        &mut self, ups_and_downs: &Vec<i32>,
        num_mapped_reads: u64,
        total_mismatches: u32);

    fn calculate_coverage(&mut self, unobserved_contig_length: u32) -> f32;


    fn print_coverage<T: CoverageTaker>(
        &self,
        coverage: &f32,
        coverage_taker: &mut T);

    fn print_zero_coverage<T: CoverageTaker>(
        &self, coverage_taker: &mut T, entry_length: u32);

    fn copy(&self) -> CoverageEstimator;

    fn num_mapped_reads(&self) -> u64;
}

impl MosdepthGenomeCoverageEstimator for CoverageEstimator {
    fn setup(&mut self) {
        debug!("Running setup..");
        match self {
            CoverageEstimator::MeanGenomeCoverageEstimator {
                ref mut pileup_counts,
                ref mut total_count,
                ref mut total_bases,
                ref mut num_covered_bases,
                ref mut num_mapped_reads,
                ref mut total_mismatches, ..
            } => {
                *pileup_counts = vec!();
                *total_count = 0;
                *total_bases = 0;
                *num_covered_bases = 0;
                *num_mapped_reads = 0;
                *total_mismatches = 0;
            },
            CoverageEstimator::TrimmedMeanGenomeCoverageEstimator {
                ref mut pileup_counts,
                ref mut counts,
                ref mut observed_contig_length,
                ref mut num_covered_bases,
                ref mut num_mapped_reads, ..
            } | CoverageEstimator::PileupCountsGenomeCoverageEstimator {
                ref mut pileup_counts,
                ref mut counts,
                ref mut observed_contig_length,
                ref mut num_covered_bases,
                ref mut num_mapped_reads, ..
            } | CoverageEstimator::VarianceGenomeCoverageEstimator {
                ref mut pileup_counts,
                ref mut observed_contig_length,
                ref mut counts,
                ref mut num_covered_bases,
                ref mut num_mapped_reads, ..
            } => {
                *pileup_counts = vec!();
                *counts = vec!();
                *observed_contig_length = 0;
                *num_covered_bases = 0;
                *num_mapped_reads = 0;
            },
            CoverageEstimator::CoverageFractionGenomeCoverageEstimator {
                ref mut total_bases,
                ref mut num_covered_bases,
                ref mut num_mapped_reads, ..
            } | CoverageEstimator::NumCoveredBasesCoverageEstimator{
                ref mut total_bases,
                ref mut num_covered_bases,
                ref mut num_mapped_reads, ..
            } => {
                *total_bases = 0;
                *num_covered_bases = 0;
                *num_mapped_reads = 0;
            },
            CoverageEstimator::ReferenceLengthCalculator {
                ref mut observed_contig_length,
                ref mut num_mapped_reads,
            } | CoverageEstimator::ReadsPerBaseCalculator {
                ref mut observed_contig_length,
                ref mut num_mapped_reads,
            } => {
                *observed_contig_length = 0;
                *num_mapped_reads = 0;
            },
            CoverageEstimator::ReadCountCalculator {
                ref mut num_mapped_reads
            } => {
                *num_mapped_reads = 0;
            }
        }
    }

    fn add_contig(
        &mut self, ups_and_downs: &Vec<i32>,
        num_mapped_reads_in_contig: u64,
        total_mismatches_in_contig: u32) {

        match self {
            CoverageEstimator::MeanGenomeCoverageEstimator {
                ref mut pileup_counts,
                ref mut total_count,
                ref mut total_bases,
                ref mut num_covered_bases,
                ref mut num_mapped_reads,
                ref mut total_mismatches,
                contig_end_exclusion, ..
            } => {
                *num_mapped_reads += num_mapped_reads_in_contig;
                *total_mismatches += total_mismatches_in_contig;
                let len = ups_and_downs.len();
                match *contig_end_exclusion*2 < len as u32 {
                    true => {
                        *total_bases += len as u32 - 2* *contig_end_exclusion},
                    false => {
                        debug!("Contig too short - less than twice the contig-end-exclusion");
                        return; //contig is all ends, too short
                    }
                }
                let mut cumulative_sum: i32 = 0;
                let mut cumulative_array: Vec<i32> = vec![0; ups_and_downs.len() as usize];
                let start_from = *contig_end_exclusion as usize;
                let end_at = len - *contig_end_exclusion as usize - 1;
                for i in 0..len {
                    let current = ups_and_downs[i as usize];
                    cumulative_sum += current;
                    cumulative_array[i as usize] = cumulative_sum;
                    if i >= start_from && i <= end_at {
                        if cumulative_sum > 0 {
                            *num_covered_bases += 1
                        }
                        *total_count += cumulative_sum as u32;
                    }
                }

                *pileup_counts = cumulative_array;
                debug!("After adding contig, have total_count {}, total_bases {}, \
                        num_covered_bases {}, mismatches {}",
                       total_count, total_bases, num_covered_bases, total_mismatches_in_contig);
            },
            CoverageEstimator::TrimmedMeanGenomeCoverageEstimator {
                ref mut pileup_counts,
                ref mut counts,
                ref mut observed_contig_length,
                ref mut num_covered_bases,
                ref mut num_mapped_reads,
                contig_end_exclusion, ..
            } | CoverageEstimator::PileupCountsGenomeCoverageEstimator {
                ref mut pileup_counts,
                ref mut counts,
                ref mut observed_contig_length,
                ref mut num_covered_bases,
                ref mut num_mapped_reads,
                contig_end_exclusion, ..
            } | CoverageEstimator::VarianceGenomeCoverageEstimator {
                ref mut pileup_counts,
                ref mut counts,
                ref mut observed_contig_length,
                ref mut num_covered_bases,
                ref mut num_mapped_reads,
                contig_end_exclusion, ..
            } => {
                *num_mapped_reads = num_mapped_reads_in_contig;
                let len1 = ups_and_downs.len();
                match *contig_end_exclusion*2 < len1 as u32 {
                    true => {
                        debug!("Adding len1 {}", len1);
                        *observed_contig_length += len1 as u32 - 2* *contig_end_exclusion},
                    false => {
                        debug!("Contig too short - less than twice the contig-end-exclusion");
                        return; //contig is all ends, too short
                    }
                }
                debug!("Total observed length now {}", *observed_contig_length);
                let cumulative_array: Vec<i32> = vec![0; ups_and_downs.len() as usize];
                let mut cumulative_sum: i32 = 0;
                let start_from = *contig_end_exclusion as usize;
                let end_at = len1 - *contig_end_exclusion as usize - 1;
                for (i, current) in ups_and_downs.iter().enumerate() {
                    if *current != 0 {
                        debug!("cumulative sum {} and current {}", cumulative_sum, current);
                        debug!("At i some, ups and downs {:?}", ups_and_downs);
                    }
                    cumulative_sum += current;
                    if i >= start_from && i <= end_at {
                        if cumulative_sum > 0 {
                            *num_covered_bases += 1
                        }
                        if counts.len() <= cumulative_sum as usize {
                            (*counts).resize(cumulative_sum as usize +1, 0);
                        }
                        (*counts)[cumulative_sum as usize] += 1
                    }
                }
                println!("{:?}", counts);
                *pileup_counts = cumulative_array;
            },
            CoverageEstimator::CoverageFractionGenomeCoverageEstimator {
                ref mut pileup_counts,
                ref mut total_bases,
                ref mut num_covered_bases,
                ref mut num_mapped_reads,
                contig_end_exclusion, ..
            } | CoverageEstimator::NumCoveredBasesCoverageEstimator{
                ref mut pileup_counts,
                ref mut total_bases,
                ref mut num_covered_bases,
                ref mut num_mapped_reads,
                contig_end_exclusion, ..
            } => {
                *num_mapped_reads = num_mapped_reads_in_contig;
                let len = ups_and_downs.len();
                match *contig_end_exclusion*2 < len as u32 {
                    true => {
                        *total_bases += len as u32 - 2* *contig_end_exclusion},
                    false => {
                        debug!("Contig too short - less than twice the contig-end-exclusion");
                        return; //contig is all ends, too short
                    }
                }
                let mut cumulative_sum: i32 = 0;
                let cumulative_array: Vec<i32> = vec![0; ups_and_downs.len() as usize];
                let start_from = *contig_end_exclusion as usize;
                let end_at = len - *contig_end_exclusion as usize - 1;
                for i in 0..len {
                    let current = ups_and_downs[i as usize];
                    cumulative_sum += current;
                    if i >= start_from && i <= end_at {
                        if cumulative_sum > 0 {
                            *num_covered_bases += 1
                        }
                    }
                }
                *pileup_counts = cumulative_array;
            },
            CoverageEstimator::ReferenceLengthCalculator {
                ref mut observed_contig_length,
                ref mut num_mapped_reads,
            } | CoverageEstimator::ReadsPerBaseCalculator {
                ref mut observed_contig_length,
                ref mut num_mapped_reads,
            } => {
                *observed_contig_length += ups_and_downs.len() as u32;
                *num_mapped_reads += num_mapped_reads_in_contig;
            },
            CoverageEstimator::ReadCountCalculator {
                ref mut num_mapped_reads
            } => {
                *num_mapped_reads += num_mapped_reads_in_contig;
            }
        }
    }

    fn calculate_coverage(&mut self, unobserved_contig_length: u32) -> f32 {
        match self {
            CoverageEstimator::MeanGenomeCoverageEstimator {
                pileup_counts: _,
                total_count,
                total_bases,
                num_covered_bases,
                num_mapped_reads: _,
                total_mismatches,
                contig_end_exclusion: _,
                min_fraction_covered_bases,
                exclude_mismatches
            } => {
                debug!("Calculating coverage with unobserved {}, \
                        total bases {}, num_covered_bases {}, total_count {}, \
                        total_mismatches {}",
                       unobserved_contig_length, total_bases, num_covered_bases,
                       total_count, total_mismatches);
                let final_total_bases = *total_bases + unobserved_contig_length;
                if final_total_bases == 0 ||
                    (*num_covered_bases as f32 / final_total_bases as f32) < *min_fraction_covered_bases {
                    return 0.0
                } else {
                        return match exclude_mismatches {
                            true => (*total_count - *total_mismatches) as f32,
                            false => *total_count as f32
                        } / final_total_bases as f32
                }
            },
            CoverageEstimator::TrimmedMeanGenomeCoverageEstimator {
                pileup_counts: _,
                counts,
                observed_contig_length,
                num_covered_bases,
                num_mapped_reads: _,
                contig_end_exclusion: _,
                min_fraction_covered_bases,
                min,
                max
            } => {
                let total_bases = *observed_contig_length + unobserved_contig_length;
                debug!("Calculating coverage with num_covered_bases {}, observed_length {}, unobserved_length {} and counts {:?}",
                       num_covered_bases, observed_contig_length, unobserved_contig_length, counts);
                let answer = match total_bases {
                    0 => 0.0,
                    _ => {
                        if (*num_covered_bases as f32 / total_bases as f32) < *min_fraction_covered_bases {
                            0.0
                        } else {
                            let min_index: usize = (*min * total_bases as f32).floor() as usize;
                            let max_index: usize = (*max * total_bases as f32).ceil() as usize;
                            if *num_covered_bases == 0 {return 0.0;}
                            counts[0] += unobserved_contig_length;

                            let mut num_accounted_for: usize = 0;
                            let mut total: usize = 0;
                            let mut started = false;
                            let mut i = 0;
                            for num_covered in counts.iter() {
                                num_accounted_for += *num_covered as usize;
                                debug!("start: i {}, num_accounted_for {}, total {}, min {}, max {}", i, num_accounted_for, total, min_index, max_index);
                                if num_accounted_for >= min_index {
                                    debug!("inside");
                                    if started {
                                        if num_accounted_for > max_index {
                                            debug!("num_accounted_for {}, *num_covered {}",
                                                   num_accounted_for, *num_covered);
                                            let num_excess = num_accounted_for - *num_covered as usize;
                                            let num_wanted = match max_index >= num_excess {
                                                true => max_index - num_excess + 1,
                                                false => 0
                                            };
                                            debug!("num wanted1: {}", num_wanted);
                                            total += num_wanted * i;
                                            break;
                                        } else {
                                            total += *num_covered as usize * i;
                                        }
                                    } else {
                                        if num_accounted_for > max_index {
                                            // all coverages are the same in the trimmed set
                                            total = (max_index-min_index+1) * i;
                                            started = true
                                        } else if num_accounted_for < min_index {
                                            debug!("too few on first")
                                        } else {
                                            let num_wanted = num_accounted_for - min_index + 1;
                                            debug!("num wanted2: {}", num_wanted);
                                            total = num_wanted * i;
                                            started = true;
                                        }
                                    }
                                }
                                debug!("end i {}, num_accounted_for {}, total {}", i, num_accounted_for, total);

                                i += 1;
                            }
                            total as f32 / (max_index-min_index) as f32
                        }
                    }
                };
                return answer
            },
            CoverageEstimator::PileupCountsGenomeCoverageEstimator {
                pileup_counts: _,
                counts: _,
                observed_contig_length,
                num_covered_bases,
                num_mapped_reads: _,
                contig_end_exclusion: _,
                min_fraction_covered_bases
            } => {
                // No need to actually calculate any kind of coverage, just return
                // whether any coverage was detected
                match observed_contig_length {
                    0 => 0.0,
                    _ => {
                        let total_bases = *observed_contig_length + unobserved_contig_length;
                        if (*num_covered_bases as f32 / total_bases as f32) < *min_fraction_covered_bases {
                            0.0
                        } else {
                            // Hack: Return the number of zero coverage bases as the
                            // coverage, plus 1 so it is definitely non-zero, so that
                            // the print_genome function knows this info.
                            (total_bases - *num_covered_bases + 1) as f32
                        }
                    }
                }
            },
            CoverageEstimator::CoverageFractionGenomeCoverageEstimator {
                pileup_counts: _,
                total_bases,
                num_covered_bases,
                num_mapped_reads: _,
                contig_end_exclusion: _,
                min_fraction_covered_bases
            } => {
                let final_total_bases = *total_bases + unobserved_contig_length;
                if final_total_bases == 0 ||
                    (*num_covered_bases as f32 / final_total_bases as f32) < *min_fraction_covered_bases {
                        return 0.0
                    } else {
                        return *num_covered_bases as f32 / final_total_bases as f32
                    }
            },
            CoverageEstimator::NumCoveredBasesCoverageEstimator {
                pileup_counts: _,
                total_bases,
                num_covered_bases,
                num_mapped_reads: _,
                contig_end_exclusion: _,
                min_fraction_covered_bases
            } => {
                let final_total_bases = *total_bases + unobserved_contig_length;
                if final_total_bases == 0 ||
                    (*num_covered_bases as f32 / final_total_bases as f32) < *min_fraction_covered_bases {
                        return 0.0
                    } else {
                        return *num_covered_bases as f32
                    }
            },
            CoverageEstimator::VarianceGenomeCoverageEstimator {
                pileup_counts: _,
                observed_contig_length,
                ref mut counts,
                num_covered_bases,
                num_mapped_reads: _,
                contig_end_exclusion: _,
                min_fraction_covered_bases
            } => {
                let total_bases = *observed_contig_length + unobserved_contig_length;
                debug!("Calculating coverage with observed length {}, unobserved_length {} and counts {:?}", num_covered_bases, unobserved_contig_length, counts);
                match total_bases {
                    0 => 0.0,
                    _ => {
                        if (*num_covered_bases as f32 / total_bases as f32) < *min_fraction_covered_bases {
                            0.0
                        } else if total_bases < 3 {
                            0.0
                        } else {
                            counts[0] += unobserved_contig_length;
                            // Calculate variance using the shifted method
                            let mut k = 0;
                            // Ensure K is within the range of coverages - take the
                            // lowest coverage.
                            while counts[k] == 0 {
                                k += 1;
                            }
                            let mut ex = 0;
                            let mut ex2 = 0;
                            for (x, num_covered) in counts.iter().enumerate() {
                                if *num_covered == 0 { continue }
                                let nc = *num_covered as usize;
                                ex += (x-k) * nc;
                                ex2 += (x-k)*(x-k) * nc;
                            }
                            // Return sample variance not population variance since
                            // almost all MAGs are incomplete.
                            (ex2 as f32 - (ex*ex) as f32/total_bases as f32) / (total_bases - 1) as f32
                        }
                    }
                }
            },
            CoverageEstimator::ReferenceLengthCalculator {
                observed_contig_length, ..
            } => {
                (*observed_contig_length + unobserved_contig_length) as f32
            },
            CoverageEstimator::ReadCountCalculator {
                num_mapped_reads
            } => {
                *num_mapped_reads as f32
            }
            CoverageEstimator::ReadsPerBaseCalculator {
                observed_contig_length,
                num_mapped_reads,
            } => {
                *num_mapped_reads as f32 /
                    (*observed_contig_length + unobserved_contig_length) as f32
            },
        }
    }

    fn copy(&self) -> CoverageEstimator {
        match self {
            CoverageEstimator::MeanGenomeCoverageEstimator {
                pileup_counts: _,
                total_count: _,
                total_bases: _,
                num_covered_bases: _,
                num_mapped_reads: _,
                total_mismatches: _,
                contig_end_exclusion,
                min_fraction_covered_bases,
                exclude_mismatches,
            } =>  {
                CoverageEstimator::new_estimator_mean(
                    *min_fraction_covered_bases,
                    *contig_end_exclusion,
                    *exclude_mismatches)
            },
            CoverageEstimator::TrimmedMeanGenomeCoverageEstimator {
                pileup_counts: _,
                counts: _,
                observed_contig_length: _,
                num_covered_bases: _,
                num_mapped_reads: _,
                contig_end_exclusion,
                min_fraction_covered_bases,
                min,
                max
            } => {
                CoverageEstimator::new_estimator_trimmed_mean(
                    *min, *max, *min_fraction_covered_bases, *contig_end_exclusion)
            },
            CoverageEstimator::PileupCountsGenomeCoverageEstimator {
                pileup_counts: _,
                counts: _,
                observed_contig_length: _,
                num_covered_bases: _,
                num_mapped_reads: _,
                contig_end_exclusion,
                min_fraction_covered_bases
            } => {
                CoverageEstimator::new_estimator_pileup_counts(
                    *min_fraction_covered_bases, *contig_end_exclusion)
            },
            CoverageEstimator::CoverageFractionGenomeCoverageEstimator {
                pileup_counts: _,
                total_bases: _,
                num_covered_bases: _,
                num_mapped_reads: _,
                contig_end_exclusion,
                min_fraction_covered_bases
            } => {
                CoverageEstimator::new_estimator_covered_fraction(
                    *min_fraction_covered_bases, *contig_end_exclusion)
            },
            CoverageEstimator::NumCoveredBasesCoverageEstimator {
                pileup_counts: _,
                total_bases: _,
                num_covered_bases: _,
                num_mapped_reads: _,
                contig_end_exclusion,
                min_fraction_covered_bases
            } => {
                CoverageEstimator::new_estimator_covered_bases(
                    *min_fraction_covered_bases, *contig_end_exclusion)
            },
            CoverageEstimator::VarianceGenomeCoverageEstimator {
                pileup_counts: _,
                observed_contig_length: _,
                counts: _,
                num_covered_bases: _,
                num_mapped_reads: _,
                contig_end_exclusion,
                min_fraction_covered_bases
            } => {
                CoverageEstimator::new_estimator_variance(
                    *min_fraction_covered_bases, *contig_end_exclusion)
            },
            CoverageEstimator::ReferenceLengthCalculator {..} => {
                CoverageEstimator::new_estimator_length()
            },
            CoverageEstimator::ReadCountCalculator {..} => {
                CoverageEstimator::new_estimator_read_count()
            },
            CoverageEstimator::ReadsPerBaseCalculator {..} => {
                CoverageEstimator::new_estimator_reads_per_base()
            }
        }
    }

    fn print_coverage<T: CoverageTaker>(
        &self,
        coverage: &f32,
        coverage_taker:&mut T) {

        match self {
            CoverageEstimator::MeanGenomeCoverageEstimator {..} |
            CoverageEstimator::TrimmedMeanGenomeCoverageEstimator{..} |
            CoverageEstimator::CoverageFractionGenomeCoverageEstimator{..} |
            CoverageEstimator::NumCoveredBasesCoverageEstimator{..} |
            CoverageEstimator::VarianceGenomeCoverageEstimator{..} |
            CoverageEstimator::ReferenceLengthCalculator{..} |
            CoverageEstimator::ReadCountCalculator{..} |
            CoverageEstimator::ReadsPerBaseCalculator{..} => {
                coverage_taker.add_single_coverage(*coverage);
            },
            CoverageEstimator::PileupCountsGenomeCoverageEstimator {
                counts, ..
            } => {
                let mut i = 0;
                debug!("{:?}", counts);
                for num_covered in counts.iter() {
                    let cov: u32 = match i {
                        0 => {
                            let c = coverage.floor() as u32;
                            match c {
                                0 => 0,
                                _ => c - 1
                            }
                        },
                        _ => *num_covered
                    };
                    coverage_taker.add_coverage_entry(i, cov);
                    i += 1
                }
            },
        }
    }

    fn print_zero_coverage<T: CoverageTaker>(&self, coverage_taker: &mut T, entry_length: u32) {
        match self {
            CoverageEstimator::MeanGenomeCoverageEstimator{..} |
            CoverageEstimator::TrimmedMeanGenomeCoverageEstimator{..} |
            CoverageEstimator::CoverageFractionGenomeCoverageEstimator{..} |
            CoverageEstimator::NumCoveredBasesCoverageEstimator{..} |
            CoverageEstimator::VarianceGenomeCoverageEstimator{..} |
            CoverageEstimator::ReadCountCalculator{..} |
            CoverageEstimator::ReadsPerBaseCalculator{..} => {
                coverage_taker.add_single_coverage(0.0);
            },
            CoverageEstimator::PileupCountsGenomeCoverageEstimator{..} => {},
            CoverageEstimator::ReferenceLengthCalculator{..} => {
                coverage_taker.add_single_coverage(entry_length as f32);
            }
        }
    }

    fn num_mapped_reads(&self) -> u64 {
        match self {
            CoverageEstimator::MeanGenomeCoverageEstimator {
                total_count: _,
                total_bases: _,
                num_covered_bases: _,
                num_mapped_reads, ..
            } |
            CoverageEstimator::TrimmedMeanGenomeCoverageEstimator {
                counts: _,
                observed_contig_length: _,
                num_covered_bases: _,
                num_mapped_reads, ..
            } |
            CoverageEstimator::PileupCountsGenomeCoverageEstimator {
                counts: _,
                observed_contig_length: _,
                num_covered_bases: _,
                num_mapped_reads, ..
            } |
            CoverageEstimator::CoverageFractionGenomeCoverageEstimator {
                total_bases: _,
                num_covered_bases: _,
                num_mapped_reads, ..
            } |
            CoverageEstimator::NumCoveredBasesCoverageEstimator {
                total_bases: _,
                num_covered_bases: _,
                num_mapped_reads, ..
            } |
            CoverageEstimator::VarianceGenomeCoverageEstimator {
                observed_contig_length: _,
                counts: _,
                num_covered_bases: _,
                num_mapped_reads, ..
            } |
            CoverageEstimator::ReferenceLengthCalculator {
                observed_contig_length: _,
                num_mapped_reads,
            } |
            CoverageEstimator::ReadCountCalculator {
                num_mapped_reads
            } |
            CoverageEstimator::ReadsPerBaseCalculator {
                observed_contig_length: _,
                num_mapped_reads,
            } => {
                *num_mapped_reads
            },
        }
    }
}
