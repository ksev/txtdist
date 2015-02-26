#![feature(collections)]
#![feature(core)]
#![feature(test)]

//! Txtdist is small utility crate for calculating the distance between two strings.

extern crate test;

use std::collections::VecMap;
use std::iter::range_inclusive;
use std::cmp::min;

/// Calculate the distance between two strings using the damerau levenshtein algorithm. 
/// 
/// 
///
/// > The Damerau–Levenshtein distance (named after Frederick J. Damerau and Vladimir I. Levenshtein) 
/// > is a distance (string metric) between two strings, i.e., finite sequence of symbols, 
/// > given by counting the minimum number of operations needed to transform one string into the other, 
/// > where an operation is defined as an insertion, deletion, or substitution of a single character, 
/// > or a transposition of two adjacent characters. 
/// [wikipedia](http://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance)
/// 
/// # Example
///
/// ```rust
/// use txtdist::damerau_levenshtein;
///
/// let distance = damerau_levenshtein("some string", "some other string");
/// assert_eq!(distance, 6)
/// ```
pub fn damerau_levenshtein(source: &str, target: &str) -> u32 {
    let (n, m) = (source.len(), target.len());

    if n == 0 { return m as u32; }
    if m == 0 { return n as u32; }

    if n == m && source == target {
        return 0;
    }
        
    let inf = n + m;

    let mut matrix = vec![vec![0; m+2]; n+2];

    matrix[0][0] = inf;
    for i in range_inclusive(0, n) {
        matrix[i+1][0] = inf;
        matrix[i+1][1] = i;
    }
    for j in range_inclusive(0, m) {
        matrix[0][j+1] = inf;
        matrix[1][j+1] = j;
    };

    let mut last_row = VecMap::new();

    for row in range_inclusive(1, n) {
        let char_s = source.char_at(row-1);
        let mut last_match_col = 0;
        
        for col in range_inclusive(1, m) {
            let char_t = target.char_at(col-1);
            let last_match_row = *last_row.get(&(char_t as usize)).unwrap_or(&0);
            let cost = if char_s == char_t { 0 } else { 1 };

            let dist_add = matrix[row][col+1] + 1;
            let dist_del = matrix[row+1][col] + 1;
            let dist_sub = matrix[row][col] + cost; 
            let dist_trans = matrix[last_match_row][last_match_col]
                            + (row - last_match_row - 1) + 1
                            + (col - last_match_col - 1);

            let dist = min(min(dist_add, dist_del), 
                           min(dist_sub, dist_trans));

            matrix[row+1][col+1] = dist;
            
            if cost == 0 {
                last_match_col = col;
            }
        }

        last_row.insert(char_s as usize, row);
    }

    matrix[n+1][m+1] as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_damerau_levenschtein() {
        let distance = damerau_levenshtein("CA", "ABC");
        assert_eq!(distance, 2);

        let distance = damerau_levenshtein("an act", "a cat");
        assert_eq!(distance, 2);

        let distance = damerau_levenshtein("", "a cat");
        assert_eq!(distance, 5);

        let distance = damerau_levenshtein("MERCEDES-BENS", "MERCEDES-BENZ");
        assert_eq!(distance, 1);

        let distance = damerau_levenshtein("", "");
        assert_eq!(distance, 0);

        let distance = damerau_levenshtein("some string", "some string");
        assert_eq!(distance, 0);
    }

    #[bench]
    fn bench_damerau_levenschtein(b: &mut Bencher) {
        b.iter(|| damerau_levenshtein("one string", "other string"));
    }

    #[bench]
    fn bench_damerau_levenschtein_same_length(b: &mut Bencher) {
        b.iter(|| damerau_levenshtein("one string12", "other string"));
    }

    #[bench]
    fn bench_damerau_levenschtein_empty(b: &mut Bencher) {
        b.iter(|| damerau_levenshtein("", "other string"));
    }

    #[bench]
    fn bench_damerau_levenschtein_same(b: &mut Bencher) {
        b.iter(|| damerau_levenshtein("some string", "some string"));
    }
}
