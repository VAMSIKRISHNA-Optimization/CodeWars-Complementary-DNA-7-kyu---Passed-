# CodeWars-Complementary-DNA-7-kyu---Passed-
Deoxyribonucleic acid (DNA) is a chemical found in the nucleus of cells and carries the "instructions" for the development and functioning of living organisms.

If you want to know more: http://en.wikipedia.org/wiki/DNA

In DNA strings, symbols "A" and "T" are complements of each other, as "C" and "G". Your function receives one side of the DNA (string, except for Haskell); you need to return the other complementary side. DNA strand is never empty or there is no DNA at all (again, except for Haskell).

More similar exercise are found here: http://rosalind.info/problems/list-view/ (source)

Example: (input --> output)

"ATTGC" --> "TAACG"
"GTAT" --> "CATA"


TEST CASES:
// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::dna_strand;
    use rand::{Rng, thread_rng, seq::SliceRandom};
    
    fn dotest(s: &str, expected: &str) {
        let actual = dna_strand(s);
        assert!(actual == expected, 
            "With dna = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }
    
    #[test]
    fn fixed_tests() {
        dotest("AAAA","TTTT");
        dotest("ATTGC","TAACG");
        dotest("GTAT","CATA");
        dotest("AAGG","TTCC");
        dotest("CGCG","GCGC");
        dotest("ATTGC","TAACG");
        dotest("GTATCGATCGATCGATCGATTATATTTTCGACGAGATTTAAATATATATATATACGAGAGAATACAGATAGACAGATTA","CATAGCTAGCTAGCTAGCTAATATAAAAGCTGCTCTAAATTTATATATATATATGCTCTCTTATGTCTATCTGTCTAAT");
    }

    #[test]
    fn random_tests() {
        let mut rng = thread_rng();
        let nucleotides = ['T', 'G', 'C', 'A'];
        let hash =
        HashMap::from([
        ('A', 'T'),
        ('C', 'G'),
        ('G', 'C'),
        ('T', 'A')]);
        for _ in 0..100 {
            let s = &(0..rng.gen_range(5..=50)).map(|_| *nucleotides.choose(&mut rng).unwrap()).collect::<String>();
            let expected = &s.chars().map(|c| hash[&c]).collect::<String>();
            dotest(s, expected);
        }
    }
}
