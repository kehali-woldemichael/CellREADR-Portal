// external
use std::collections::HashMap;
use itertools::Itertools;


#[derive(Default)]
pub struct TargetSeq<'a> {
    pub target_dna: String,
    pub rc_dna: String,
    pub length: i32, 
    pub orf_1_dna: String, // needed named lifetime parameter ... lifetime is at least as long as struct itself 
    pub orf_1_protein: Vec<&'a str>, // needed named lifetime parameter ... lifetime is at least as long as struct itself 
    pub orf_2_dna: String, // needed named lifetime parameter ... lifetime is at least as long as struct itself 
    pub orf_2_protein: Vec<&'a str>, // needed named lifetime parameter ... lifetime is at least as long as struct itself 
    pub orf_3_dna: String, // needed named lifetime parameter ... lifetime is at least as long as struct itself 
    pub orf_3_protein: Vec<&'a str>, // needed named lifetime parameter ... lifetime is at least as long as struct itself 
}

impl TargetSeq<'_> { // anonymous lifetime paramter
    pub fn return_reverse_complement(sequence: &String) -> String {
        let base_pairing_table: HashMap<&str, &str> = HashMap::from([
            ("A", "T"), 
            ("T", "A"), 
            ("C", "G"), 
            ("G", "C"),            
        ]);

        // Reverse sequence and use base_pairing_table to return reverse complement 
        sequence
            .chars()
            .rev()
            .into_iter()
            .map(|x| base_pairing_table[x.to_string().as_str()].to_string())
            .collect::<String>()
    }

    pub fn return_translated(sequence: String) -> Vec<&'static str> {
        let translation_table: HashMap<&str, &str> = HashMap::from([
            ("TAA", "0"), ("TGA", "0"), ("TAG", "0"),            
            ("TTT", "F"), ("TTC", "F"),
            ("TTA", "L"), ("TTG", "L"), ("CTT", "L"), ("CTC", "L"), ("CTA", "L"), ("CTG", "L"), 
            ("ATT", "I"), ("ATC", "I"), ("ATA", "I"), 
            ("ATG", "M"),
            ("GTT", "V"), ("GTC", "V"), ("GTA", "V"), ("GTG", "V"), 
            ("TCT", "S"), ("TCC", "S"), ("TCA", "S"), ("TCG", "S"), 
            ("CCT", "P"), ("CCC", "P"), ("CCA", "P"), ("CCG", "P"), 
            ("ACT", "T"), ("ACC", "T"), ("ACA", "T"), ("ACG", "T"), 
            ("GCT", "A"), ("GCC", "A"), ("GCA", "A"), ("GCG", "A"), 
            ("TAT", "Y"), ("TAC", "Y"), 
            ("CAT", "H"), ("CAC", "H"), 
            ("CAA", "Q"), ("CAG", "Q"), 
            ("AAC", "N"), ("AAT", "N"),
            ("AAA", "K"), ("AAG", "K"), 
            ("GAT", "D"), ("GAC", "D"), 
            ("GAA", "E"), ("GAG", "E"), 
            ("TGT", "C"), ("TGC", "C"), 
            ("TGG", "W"),
            ("CGT", "R"), ("CGC", "R"), ("CGA", "R"), ("CGG", "R"), ("AGA", "R"), ("AGG", "R"), 
            ("AGC", "S"), ("AGT", "S"), 
            ("GGT", "G"), ("GGC", "G"), ("GGA", "G"), ("GGG", "G"), 
        ]);

        sequence
            .chars()
            .chunks(3)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .map(|codon| translation_table[codon.as_str()])
            .collect::<Vec<&str>>()
    }

    pub fn new(dna: String) -> TargetSeq<'static> {
        let mut dna_seq = String::from(dna.trim().to_uppercase()); //sometimes trailing white spaces added when reading in
        let mut seq_len: i32 = dna_seq.len() as i32;
        
        // Truncating sequence if not divisble by 3 ... i.e. to allow for translation 
        let remainder_seq_length: i32 = (dna_seq.len() as i32) % 3;
        if remainder_seq_length != 0 {
            dna_seq = dna_seq[0..((seq_len - remainder_seq_length) as usize)].to_string();
            seq_len = dna_seq.len() as i32;
        } else {
            //log::info!("Target seq divisible by 3");
        }

        //log::debug!("Target sequence is START:{}:STOP", dna_seq);
        
        let end_orf2: usize = usize::try_from(seq_len - 2).unwrap();
        let end_orf3: usize = usize::try_from(seq_len - 1).unwrap();
        //log::debug!("Target sequence length is: {}", seq_len);
    
        let rc_seq: String = Self::return_reverse_complement(&dna_seq);
        //log::debug!("Reverse complement of target sequence = START:{}:STOP", rc_seq);

        TargetSeq {
            target_dna: dna_seq,
            rc_dna: rc_seq.clone(),
            length: seq_len,
            orf_1_dna: rc_seq.clone(),
            orf_1_protein: Self::return_translated(rc_seq.clone()),
            orf_2_dna: rc_seq[1..(end_orf2)].to_string(),
            orf_2_protein: Self::return_translated(rc_seq[1..(end_orf2)].to_string()),
            orf_3_dna: rc_seq[2..(end_orf3)].to_string(),
            orf_3_protein: Self::return_translated(rc_seq[2..(end_orf3)].to_string()),
        }
    }
}