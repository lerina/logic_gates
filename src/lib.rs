#![doc(html_logo_url = "https://www.razafy.com/logo_lerina_64x64.png#50p50")]

/// # Logic Gate `and`
///
/// Implements a boolean `and` gate 
/// taking as input two bits 
/// and returns a bit as output
/// 
/// ![logic and gate](../../../assets/AND_ANSI.svg)
/// 
/// # Example
///
/// ```
/// use logic_gates::and;
///
/// assert_eq!(and(0,0), 0);
/// assert_eq!(and(0,1), 0);
/// assert_eq!(and(1,0), 0);
/// assert_eq!(and(1,1), 1);
/// ```
/// 
pub fn and(a: u8, b: u8) -> u8 {
    //unimplemented!()
    match (a, b) {
        (1, 1) => 1,
        _ => 0
    }
}

/// # Logic Gate `xor`
///
/// Implements a boolean `xor` gate 
/// taking as input two bits 
/// and returning a bit as output
///
/// ![logic xor gate](../../../assets/XOR_ANSI.svg#50p50)
///
/// # Example
///
/// ```
/// use logic_gates::xor;
///
/// assert_eq!(xor(0,0), 0);
/// assert_eq!(xor(0,1), 1);
/// assert_eq!(xor(1,0), 1);
/// assert_eq!(xor(1,1), 0);
/// ```
/// 
pub fn xor(a: u8, b: u8) -> u8 {
    //unimplemented!()
    match (a, b) {
        (1, 0) | (0, 1) => 1,
        _ => 0
    }
}


#[cfg(test)]
mod tests {
    use crate::{xor, and};
    
    #[test]
    fn test_and() {
        assert_eq!(1, and(1, 1));
        assert_eq!(0, and(0, 1));
        assert_eq!(0, and(1, 0));
        assert_eq!(0, and(0, 0));
    }

    #[test]
    fn test_xor() {
        assert_eq!(1, xor(1, 0));
        assert_eq!(0, xor(0, 0));
        assert_eq!(0, xor(1, 1));
        assert_eq!(1, xor(0, 1));
    }
}
