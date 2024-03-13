pub fn gcd(mut x : i32, mut y : i32) -> i32{
    while y != 0 {
        let temp = y; // 20, 5
        y = x % y; //5, 0
        x = temp; //20, 5
    }
    x.abs()
}

pub fn xgcd(a : i64, b : i64) -> (i64, i64, i64){
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
    fn test_xgcd(){
        assert_eq!(xgcd(28, 58), (2, -2, 1));
    }
}