fn repetitions(dna: &str) -> u32 {
    let mut prev_char: Option<char> = None;
    let mut cont = 0;
    let mut max = 0;

    for char in dna.chars() {
        if let Some(prev) = prev_char {
            if prev == char {
                cont += 1;
            } else {
                cont = 1;
                prev_char = Some(char);
            }
        } else {
            prev_char = Some(char);
            cont = 1;
        }

        if cont > max {
            max = cont;
        }
    }
    max
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_repetitions_basic() {
        // Prueba básica con una secuencia de ADN
        let dna = "ATTCGGGTA";
        assert_eq!(repetitions(dna), 3);
    }

    #[test]
    fn test_repetitions_empty() {
        // Prueba con una secuencia de ADN vacía
        let dna = "";
        assert_eq!(repetitions(dna), 0);
    }

    #[test]
    fn test_repetitions_single_character() {
        // Prueba con una secuencia de ADN que contiene un solo carácter
        let dna = "AAAAAAA";
        assert_eq!(repetitions(dna), 7);
    }

    #[test]
    fn test_repetitions_no_repetitions() {
        // Prueba con una secuencia de ADN sin repeticiones
        let dna = "ACGTACGT";
        assert_eq!(repetitions(dna), 1);
    }

    #[test]
    fn test_repetitions_longest_repetition() {
        // Prueba con una secuencia de ADN con la repetición más larga al final
        let dna = "GATCGGAATTTTTGGGGG";
        assert_eq!(repetitions(dna), 5);
    }
}
