/// Calculate a number representing the permutation of the slice
/// The slice must contain the elements offset..offset+N
/// A result of 0 indicates that the slice is ordered
pub fn calculate_permutation<T: Copy + Into<usize>, const N: usize>(
    mut slice: [T; N],
    offset: usize,
) -> usize {
    let mut b = 0;

    for j in (1..N).rev() {
        let k = slice
            .into_iter()
            .position(|x| x.into() == j + offset)
            .unwrap();
        let diff = (k + 1) % (j + 1);
        if diff != 0 {
            slice[0..=j].rotate_left(diff);
        }

        b = ((j + 1) * b) + diff;
    }

    b
}

/// Reorder a slice from the default order to the order corresponding to a particular permutation
pub fn reorder_to_permutation<T, const N: usize>(slice: &mut [T; N], mut permutation: usize) {
    if permutation == 0 {
        return;
    }
    for j in 1..N {
        let k = permutation % (j + 1);
        permutation /= j + 1;
        if k > 0 {
            slice[0..=j].rotate_right(k)
        }
    }
}

/// Calculate a representation of this orientation as a single number
pub fn calculate_number_representation<T: Copy + Into<usize>, const N: usize>(
    slice: &[T; N],
    num_values: usize,
) -> usize {
    let mut a: usize = 0;

    for x in slice.iter().take(N - 1) {
        //Do not take the last element - it is controlled by parity
        a = (num_values * a) + (*x).into();
    }
    a
}

pub fn set_from_number_representation<T: Copy + From<usize>, const N: usize, const MOD: usize>(
    arr: &mut [T; N],
    rep: usize,
) {
    let mut parity: usize = 0;
    let mut rem = rep;

    for i in (0..N - 1).rev() {
        arr[i] = (rem % MOD).into();
        parity += rem % MOD;
        rem /= MOD;
    }

    //Set the last element from parity
    let last = (MOD - (parity % MOD)) % MOD;
    arr[N - 1] = last.into();
}

pub fn calculate_position_parity<T: PartialOrd, const N: usize>(slice: &[T; N]) -> u8 {
    let mut s = 0;
    for i in 1..N {
        for j in 0..i {
            if slice[i] > slice[j] {
                s += 1;
            }
        }
    }
    s % 2
}

///Calculate n choose k
pub const fn binomial(mut n: usize, k: usize) -> usize {
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
        n -= 1;
        d += 1;
    }
    r
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use ntest::test_case;

    #[test_case(0, 1, 2, 3, 0, 0)]
    #[test_case(3, 4, 5, 6, 3, 0)]
    #[test_case(1, 0, 2, 3, 0, 1)]
    #[test_case(4, 3, 5, 6, 3, 1)]
    #[test_case(3, 0, 1, 2, 0, 6)]
    #[test_case(3, 1, 0, 2, 0, 7)]
    #[test_case(2, 1, 3, 0, 0, 23)] //The highest possible of the 24 permutations

    fn test_permutation(a: usize, b: usize, c: usize, d: usize, offset: usize, expected: usize) {
        let slice: [usize; 4] = [a, b, c, d];
        let r = calculate_permutation(slice, offset);
        assert_eq!(r, expected);
    }

    #[test_case(0, 0, 1, 2, 3)]
    #[test_case(1, 1, 0, 2, 3)]
    #[test_case(6, 3, 0, 1, 2)]
    #[test_case(7, 3, 1, 0, 2)]
    #[test_case(23, 2, 1, 3, 0)]
    fn test_set_from_permutation(permutation: usize, ea: i32, eb: i32, ec: i32, ed: i32) {
        let mut slice = [0, 1, 2, 3];
        let expected = [ea, eb, ec, ed];
        reorder_to_permutation(&mut slice, permutation);

        assert_eq!(slice, expected);
    }

    #[test_case(0, 0, 1)]
    #[test_case(4, 0, 1)]
    #[test_case(4, 1, 4)]
    #[test_case(4, 2, 6)]
    #[test_case(10, 5, 252)]
    fn test_binomial(n: u16, k: u16, expected: u16) {
        let r = binomial(n, k);
        assert_eq!(r, expected);
    }
}
