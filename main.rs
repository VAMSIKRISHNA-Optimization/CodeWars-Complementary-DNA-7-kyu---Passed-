fn dna_strand(dna: &str) -> String 
{
    dna
        .chars()
        .map(|c| 
        {
            match c
            {
                'A' => 'T',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _   => 'S',
            }
            
        })
        .collect::<String>()
}

fn main ()
{
    println!("{:?}", dna_strand("ATTGC"));
}
