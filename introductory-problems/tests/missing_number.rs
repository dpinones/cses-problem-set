
fn find_missing_number(arr: &[u32]) -> u32 {
    let n = arr.len() as u32 + 1;
    let expected_sum = (n * (n + 1)) / 2;
    let actual_sum: u32 = arr.iter().sum();

    expected_sum - actual_sum
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_missing_number_basic() {
        // Prueba básica con un arreglo de números del 1 al 5
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(find_missing_number(&arr), 6);
    }
    
    #[test]
    fn test_missing_number_empty() {
        // Prueba con un arreglo vacío
        let arr: Vec<u32> = Vec::new();
        assert_eq!(find_missing_number(&arr), 1);
    }
    
    #[test]
    fn test_missing_number_single_element() {
        // Prueba con un arreglo que contiene solo un elemento faltante
        let arr = vec![1, 2, 3, 4, 6];
        assert_eq!(find_missing_number(&arr), 5);
    }
}
