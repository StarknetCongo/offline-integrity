
/// # GCD library
/// `gcd` permet d'obtenir le plus grand diviseur commun de deux nombres(entiers positifs);
pub fn gcd(mut x : i128, mut y : i128) -> i128{
    if x < 0 || y < 0 {
        panic!("Tous les nombres doivent être positif");
    }

    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x.abs()
}

/// # XGCD library
/// `xgcd` permet d'obtenir l'extended euclidean algorithm pour avoir les coefficients de ...;
pub fn xgcd(a : i128, b : i128) -> (i128, i128, i128){
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x, y ) = xgcd(b % a, a);
        (gcd, y - (b / a) * x, x)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_gcd(){
        assert_eq!(2, gcd(58, 28));
        assert_eq!(gcd(5, 20), 5);
    }

    #[test]
    #[should_panic]
    fn test_panic_gcd(){
        gcd(-24, 20);
    }

    #[test]
    fn test_xgcd(){
        assert_eq!(xgcd(28, 58), (2, -2, 1));
    }
}