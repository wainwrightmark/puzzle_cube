use itertools::Itertools;



/// Calculate a number representing the permutation of the slice
/// The slice must contain the elements offset..offset+N
/// A result of 0 indicates that the slice is ordered
pub fn calculate_permutation<T : Copy + Into<usize>, const N: usize>(mut slice:  [T; N], offset: usize ) -> usize{
    let mut b = 0;

    for j in (1..N).rev(){
        let k = slice.into_iter().position(|x| x.into() == j + offset).unwrap();
        let diff = (k + 1) % (j + 1);
        if diff != 0{
            slice[0..j].rotate_left(diff);
        }
        
        b = ((j + 1) * b) + diff;
    }

    b
}


///Calculate n choose k
pub const fn binomial(mut n: u16, k: u16) -> u16 {    
    if k > n {
        return 0;
    }
    if k > n - k {
        return binomial(n, n - k);
    }
    let mut r = 1;
    let mut d = 1;
    loop {
        if d > k {
            break;
        }
        r = (r * n) / d;
        n = n - 1;
        d = d + 1;
    }
    r
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use ntest::test_case;

    #[test_case(0,1,2,3, 0, 0)]
    #[test_case(3,4,5,6, 3, 0)]
    #[test_case(1,0,2,3, 0, 1)]
    #[test_case(4,3,5,6, 3, 1)]
    #[test_case(3,0,1,2, 0, 9)]      
    #[test_case(3,1,0,2, 0, 8)]
    #[test_case(1,2,3,0, 0, 23)] //The highest possible of the 24 permutations

    fn test_permutation(a: usize, b: usize, c: usize, d: usize, offset: usize, expected: usize){

        let slice: [usize; 4] = [a,b,c,d];
        let r = calculate_permutation(slice, offset);
        assert_eq!(r, expected);
    }


    #[test_case(0,0,1)]
    #[test_case(4,0,1)]
    #[test_case(4,1,4)]
    #[test_case(4,2,6)]
    #[test_case(10,5,252)]
    fn test_binomial(n:u16, k:u16, expected:u16) {
        let r = binomial(n, k);
        assert_eq!(r, expected);
    }

}