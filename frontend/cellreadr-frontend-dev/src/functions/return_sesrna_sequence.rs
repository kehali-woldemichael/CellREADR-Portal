// external
// internal
use crate::structs::sesrna::SesRNA;
use crate::structs::target_seq::{TargetSeq};


#[derive(Clone)]
pub struct DistancesMismatch {
    pub index_tgg: i32,
    pub vec_distance_mistmatches: Vec<i32>,
    pub distance_max_mismatch: i32,
}

#[allow(unused_variables)] 
pub fn return_sesrnas(target_sequence: String, vec_parameters: Vec<String>) -> Vec<SesRNA> {
    // Load parameters  
    // vec_parameters: target_sequence_length, min_sesrna_length, max_sesrnalength, num_mismatches_max 
    let target_seq = TargetSeq::new(target_sequence);
    //log::debug!("ORF 1 codon: START{:?}STOP", target_seq.orf_1);
    //log::debug!("ORF 2 codon: START{:?}STOP", target_seq.orf_2);
    //log::debug!("ORF 3 codon: START{:?}STOP", target_seq.orf_3);

    // Change parameters from String to i32 
    let param_sesrna_length_min: i32 = vec_parameters[1].parse().unwrap();
    let param_sesrna_length_max: i32 = vec_parameters[2].parse().unwrap();
    let param_sesrna_max_mismatch_num: i32 = vec_parameters[3].parse().unwrap();
    let param_minimum_distance_tgg_mismatch: i32 = vec_parameters[4].parse().unwrap();

    // Calculate number of codons 
    // Test if divisble by 3 ... if not then round min up ... and round max down 
    let param_sesrna_codon_length_min: i32 = &param_sesrna_length_min/3;
    let param_sesrna_codon_length_max: i32 = &param_sesrna_length_max/3;

    // 
    //log::info!("Identifying sesRNAs for ORF 1");
    let orf_1_sesrnas: Vec<SesRNA> = identify_sesrnas(target_seq.orf_1_dna, target_seq.orf_1_protein, 
                                                        param_sesrna_codon_length_min, param_sesrna_codon_length_max, 
                                                        param_sesrna_max_mismatch_num, param_minimum_distance_tgg_mismatch
                                                    );
    //log::info!("Identifying sesRNAs for ORF 2");
    let orf_2_sesrnas: Vec<SesRNA> = identify_sesrnas(target_seq.orf_2_dna, target_seq.orf_2_protein, 
                                                        param_sesrna_codon_length_min, param_sesrna_codon_length_max, 
                                                        param_sesrna_max_mismatch_num, param_minimum_distance_tgg_mismatch
                                                    );
    //log::info!("Identifying sesRNAs for ORF 3");
    let orf_3_sesrnas: Vec<SesRNA> = identify_sesrnas(target_seq.orf_3_dna, target_seq.orf_3_protein, 
                                                        param_sesrna_codon_length_min, param_sesrna_codon_length_max, 
                                                        param_sesrna_max_mismatch_num, param_minimum_distance_tgg_mismatch
                                                    );

    let mut vec_sesrnas_all: Vec<SesRNA> = [orf_1_sesrnas.as_slice(), orf_2_sesrnas.as_slice(), orf_3_sesrnas.as_slice()].concat();
    // Remove duplicates 
    vec_sesrnas_all.sort_by(|a,b| a.dna_sequence.partial_cmp(&b.dna_sequence).expect("No NaNs"));
    vec_sesrnas_all.dedup_by(|x, y| x.dna_sequence == y.dna_sequence);
    vec_sesrnas_all.sort_by(|a,b| a.index_tgg.partial_cmp(&b.index_tgg).expect("No NaNs"));

    return vec_sesrnas_all

}

#[allow(unused)]
pub fn identify_sesrnas(dna_sequence: String, codon_sequence: Vec<&str>, 
                        min_length: i32, max_length: i32, 
                        max_mismatch_num: i32, dist_tgg_mismatch: i32,
                    ) -> Vec<SesRNA> { 
    //log::debug!("RC codon sequence = {:?}", codon_sequence);
    let tgg_indicies: Vec<i32> = return_indices(&codon_sequence, "W");
    //log::debug!("Indicies TGG = {:?}", tgg_indicies);
    let start_indicies: Vec<i32> = return_indices(&codon_sequence, "M");
    //log::debug!("Indicies START = {:?}", start_indicies);
    let stop_indicies: Vec<i32> = return_indices(&codon_sequence, "0");
    //log::debug!("Indicies STOP = {:?}", stop_indicies);

    let empty_vector: Vec<SesRNA> = vec![];

    // Return empty vector if no TGGs
    if tgg_indicies.len() == 0 {
        //log::warn!("No TGGs in thiS reading frame");
        return empty_vector
    } else {
        // check that TGG is not too close to edge 
        let length_orf: i32 = codon_sequence.len() as i32; 
        //log::debug!("Length ORF = {}", length_orf);
        let min_distance_edge: i32 = ((min_length as f32)/(2.0)).ceil() as i32;
        //log::debug!("Test = {}", min_distance_edge);

        let vec_pass_tgg_indices: Vec<&i32> = tgg_indicies
                    .iter()
                    .filter(|x: &&i32| **x > min_distance_edge) // checks for distance from edge 
                    .filter(|x: &&i32| **x < (length_orf - min_distance_edge))
                    .collect();

        //log::debug!("Indicies TGG filtered for distance from edge= {:?}", vec_pass_tgg_indices);

        // Return empty vector if no TGGs
        if vec_pass_tgg_indices.len() == 0 {
            //log::warn!("All TGGs too close to edge to fullfill min sesRNA requirement");
            return empty_vector
        } else {
            // For each TGG, identify distance from to TGG to all mismatches (START & STOP codon) 
            // And identify distance of "max mismatch" ... i.e. mismatch that is the limit of number of maximum mismatches 
            // e.g. if limit if 3 ... then "max mismatch" index would be index of 3rd closest mismatch 
            let vec_struct_distance_mismatches: Vec<DistancesMismatch> = vec_pass_tgg_indices.iter()
                                .map(|x| distances_to_mismatches(&x, &start_indicies, &stop_indicies, &max_mismatch_num))
                                .collect::<Vec<DistancesMismatch>>();

            // check if TGG passes filter
            let filtered_struct: Vec<DistancesMismatch> = vec_struct_distance_mismatches.iter()
                                                                .filter(|x: &&DistancesMismatch| filter_tggs((**x).clone(), min_length, max_length, max_mismatch_num, dist_tgg_mismatch)) 
                                                                .map(|x| x.clone())
                                                                .collect::<Vec<DistancesMismatch>>();

            // fetch sesRNA sequence
            let vec_sesrnas: Vec<SesRNA> = filtered_struct.iter()
                .map(|x| return_sesrna_sequence(&x.index_tgg, &x.distance_max_mismatch, &x.vec_distance_mistmatches,
                        &dna_sequence, (dna_sequence.len() as i32), &start_indicies, &stop_indicies,
                        &min_length, &max_length, &max_mismatch_num))
                .collect::<Vec<SesRNA>>();

            return vec_sesrnas
        }

    }
     
}

#[allow(unused)]
pub fn return_sesrna_sequence<'c>(index_tgg: &'c i32, distance_max_mismatch: &'c i32, vec_distance_mistmatches: &'c Vec<i32>,
                                    rc_dna: &'c String, length_dna: i32, indicies_start: &'c Vec<i32>, indicies_stop: &'c Vec<i32>,
                                    sesrna_minimum_length: &'c i32, sesrna_maximum_length: &'c i32, max_mismatch_num: &'c i32) -> SesRNA {

    //let vec_index_mismatches: Vec<i32> = vec_distance_mistmatches.clone().into_iter().map(|x| x + *index_tgg).collect::<Vec<i32>>(); 
    //log::info!("Vec index mismatches: {:?}", vec_index_mismatches);

    //let left_vec_mismatches: Vec<i32> = vec_distance_mistmatches.clone().into_iter().filter(|x| *x < 0).collect::<Vec<i32>>();
    //let right_vec_mismatches: Vec<i32> = vec_distance_mistmatches.clone().into_iter().filter(|x| *x > 0).collect::<Vec<i32>>();

    // Output = index of upstream and downstream DNA sequence 
    // Scenario 1: can form maximum length - [index_tgg - max_length, index_tgg + max_length]
    let half_max: i32 = ((*sesrna_maximum_length as f32)/2.0).floor() as i32;
    let half_min: i32 = ((*sesrna_minimum_length as f32)/2.0).floor() as i32;

    // Left border
    let index_left_max: usize;
    if *index_tgg >= half_max {
        index_left_max = (*index_tgg - half_max) as usize;
    } else {
        index_left_max = 0;
    }
    // left_max, (index_tgg - half_min), (index_tgg - abs(distance_max_mismatch), 

    // Right Border
    let index_right_max: usize;
    if (*index_tgg + half_max)  <= (length_dna/3) {
        index_right_max = (*index_tgg + half_max) as usize;
    } else {
        index_right_max = (length_dna/3) as usize;
    }

    // let index_left_border: usize = index_left_max;
    // let index_right_border: usize = index_right_max;

    let index_left_border: usize;
    let index_right_border: usize;
    //let index_tgg_max_mismatch: usize = (index_tgg + (*distance_max_mismatch - 1)) as usize;
    let index_tgg_max_mismatch: usize = (index_tgg + (*distance_max_mismatch)) as usize;
    //let index_second_max_mismatch: usize = ((index_tgg + vec_distance_mistmatches[*max_mismatch_num as usize + 1]) as usize);
    //log::debug!("Index of second mismatch: {}", index_second_max_mismatch);

    // When max mismatch is located closer from TGG than maximum size of sesRNA
    // Then need to figure out a size less than maximum size but more than minimum 
    // Reminder - max mismatch should not be able to occur within minimum sesRNA size 
    if distance_max_mismatch.abs() < half_max {

        // When max mismatch is to the right of TGG... i.e. positive displacement relative to index of TGG
        if *distance_max_mismatch > 0 {
            index_right_border = index_tgg_max_mismatch - 1; // subtract to exclude max distance TGG
            // Leaving meat on bone ... too aggressive when trimming 
            // Best version would have index_left_border go up to second max mismatch 
            index_left_border = (index_tgg - half_min) as usize;
            
            // For cases where there are mismatches between left border & max mismatch
            // if index_left_max < index_second_max_mismatch {
            //     index_left_border = index_left_max;
            // } else {
            //     index_left_border = index_second_max_mismatch - 1;
            // }

        // When max mismatch is to the left of TGG ... i.e. negative displacement relative to index of TGG
        } else if *distance_max_mismatch < 0 {
            index_left_border = index_tgg_max_mismatch + 1; // add to exclude max distance TGG
            index_right_border = (index_tgg + half_min) as usize;
            
           
            // To account for mismatch that is one position farther than max mismatch 
            // For when second max mismatch is less than max size of sesRNA
            // if index_right_max < index_second_max_mismatch {
            //     index_right_border = index_right_max;
            // For cases where there are mismatches between left border & max mismatch
            // } else {
            //     index_right_border = index_second_max_mismatch - 1;
            // }

        } else {
            index_left_border = index_left_max;
            index_right_border = index_right_max;
        }

    // When max mismatch is located farther from TGG maximum size of sesRNA
    // Then sesRNA can just be the maximum size of sesRNA
    } else {
        index_left_border = index_left_max;
        index_right_border = index_right_max;
    }


    //log::debug!("Left max: {}", index_left_max);
    //log::debug!("Right max: {}", index_right_max);
    //log::debug!("Left border: {}", index_left_border);
    //log::debug!("Right border: {}", index_right_border);


    let sesrna_sequence: String = rc_dna[((index_left_border*3))..(index_right_border*3)].to_string();
    let sesrna_length: usize = sesrna_sequence.len();

    let gc_content: usize = ((((sesrna_sequence.chars().filter(|c| (*c == 'G') | (*c == 'C')).count()) as f32) / (sesrna_length as f32)) * 100.0) as usize;
    //log::info!("Indicies stop: {:?}", indicies_stop);

    let count_num_start: usize = indicies_start.iter().filter(|x| **x < index_right_border as i32 && **x >= index_left_border as i32).count();
    let count_num_stop: usize = indicies_stop.iter().filter(|x| **x <  index_right_border as i32 && **x >= index_left_border as i32).count();

    // 1.1 - border 
    //log::info!("...");
    //log::info!("...");

    // define upstream
    // define downstream
    return SesRNA {
        index_tgg: (*index_tgg as usize),
        index_begin: index_left_border*3,
        index_end: index_right_border*3,
        num_mismatches: count_num_start+count_num_stop,
        num_start: count_num_start,
        num_stop: count_num_stop,
        length: sesrna_length,
        gc_content: gc_content,
        dna_sequence: sesrna_sequence,
    }

}

#[allow(unused)]
pub fn filter_tggs(input_vec_distance_mismatch: DistancesMismatch, 
                    min_length: i32, max_length: i32,
                    max_mismatch: i32, min_dist: i32,
                    ) -> bool {
    // for each tgg - see if it passes criteria & return true false

    // fail: if distance of max mismatch < min_sesrna_length
    // fail: if max_num_mismatch == 0 && distance_max_mismatch max_sesrna_length 
    // pass: if distance of max mismatch > max_sesrna_length
    if input_vec_distance_mismatch.distance_max_mismatch.abs() < (min_length/2) {
        //log::warn!("TGG failed at index due to too many mismatches: {}", input_vec_distance_mismatch.index_tgg);
        //log::warn!("...");
        return false 
    } else if input_vec_distance_mismatch.vec_distance_mistmatches[0] < (min_dist/3) {
        //log::warn!("TGG failed at index due to mismatch too close to TGG: {}", input_vec_distance_mismatch.index_tgg);
        //log::warn!("...");
        return false 
    } else {
        //log::info!("TGG passed at index: {}", input_vec_distance_mismatch.index_tgg);
        // log::info!("Index of max mismatch: {}", input_vec_distance_mismatch.distance_max_mismatch);
        //log::info!("Mismatches of passed TGG: {:?}", input_vec_distance_mismatch.vec_distance_mistmatches);
        //log::info!("...");
        // log::info!("Min length: {}", min_length);
        // log::info!("Max length: {}", max_length);
        return true 
    }
}

pub fn return_indices(target: &Vec<&str>, query: &str) -> Vec<i32> {
    let indices: Vec<i32> = target
        .iter()
        .enumerate()
        .filter(|(_, &r)| r == query)
        .map(|(index, _)| index as i32)
        .collect::<Vec<_>>();
    return indices 
}

pub fn distances_to_mismatches<'a>(index_tgg: &'a i32, indicies_start: &'a Vec<i32>, indicies_stop: &'a Vec<i32>, max_num_mismatches: &'a i32) -> DistancesMismatch {
    let vec_start: Vec<i32> = indicies_start.iter().map(|x| *x as i32).map(|i| (i - index_tgg)).collect::<Vec<_>>();
    let vec_stop : Vec<i32> = indicies_stop.iter().map(|x| *x as i32).map(|i| (i - index_tgg)).collect::<Vec<_>>();
    let mut vec_all: Vec<i32> = [vec_start, vec_stop].concat();
    
    // Sort without taking sign into account
    vec_all.sort_by(|a, b| a.abs().cmp(&b.abs())); 
    //log::debug!("Vec distance mistmatches of TGG = {:?}", vec_all);
    let vec_max_mismatch: i32 = vec_all.clone()[*max_num_mismatches as usize];
    //log::debug!("Index of mismatch at limit for number of mismatches = {}", vec_max_mismatch);

    let struct_mismatches = DistancesMismatch {
        index_tgg: index_tgg.clone(),
        vec_distance_mistmatches: vec_all,
        distance_max_mismatch:vec_max_mismatch,
    };

    return struct_mismatches;
}