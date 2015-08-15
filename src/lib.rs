//! Txtdist is small utility crate for calculating the distance between two strings.

#![feature(test)]

extern crate test;

use std::ops::{ Index, IndexMut };
use std::collections::BTreeMap;
use std::cmp::min;

// A simple wrapper around vec so we can get contiguous but index it like it's 2D array.
struct N2Array<T> {
    y_size: usize,
    buf: Vec<T>
}

impl<T: Clone> N2Array<T> {
    fn new(x: usize, y: usize, value: T) -> N2Array<T> {
        N2Array { y_size: y, buf: vec![value; x * y] }
    }
}

impl<T> Index<(usize, usize)> for N2Array<T> {
    type Output = T;

    #[inline]
    fn index<'a>(&'a self, (x, y): (usize, usize)) -> &'a T {        
        &self.buf[(x * self.y_size) + y]
    }
}

impl<T> IndexMut<(usize, usize)> for N2Array<T> {
    #[inline]
    fn index_mut<'a>(&'a mut self, (x, y): (usize, usize)) -> &'a mut T {
        &mut self.buf[(x * self.y_size) + y]
    }
}

/// Calculate the distance between two strings using the levenshtein algorithm.
/// 
/// > Levenshtein distance is a string metric for measuring the difference between two sequences. 
/// > Informally, the Levenshtein distance between two words is the minimum number of single-character edits 
/// > (i.e. insertions, deletions or substitutions) required to change one word into the other.
/// [wikipedia](http://en.wikipedia.org/wiki/Levenshtein_distance)
/// 
/// # Example
///
/// ```rust
/// use txtdist::levenshtein;
///
/// let distance = levenshtein("an act", "a cat");
/// assert_eq!(distance, 3)
/// ```
pub fn levenshtein(source: &str, target: &str) -> u32 {
    let (n, m) = (source.len(), target.len());

    let mut matrix = N2Array::new(n+1, m+1, 0);

    for i in 1..n+1 {
        matrix[(i, 0)] = i;
    }

    for i in 1..m+1 {
        matrix[(0, i)] = i;
    }

    for (row, char_s) in source.chars().enumerate() {
        let row = row + 1;
        
        for (col, char_t) in target.chars().enumerate() {
            let col = col + 1;
            if char_s == char_t {
                matrix[(row, col)] = matrix[(row-1, col-1)];
            } else {
                matrix[(row, col)] = min(matrix[(row-1, col)]   + 1, 
                                     min(matrix[(row, col-1)]   + 1,
                                         matrix[(row-1, col-1)] + 1));
            }
        }
    }

    matrix[(n, m)] as u32
}

/// Calculate the distance between two strings using the damerau levenshtein algorithm. 
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
/// let distance = damerau_levenshtein("an act", "a cat");
/// assert_eq!(distance, 2)
/// ```
pub fn damerau_levenshtein(source: &str, target: &str) -> u32 {
    let (n, m) = (source.len(), target.len());

    if n == 0 { return m as u32; }
    if m == 0 { return n as u32; }

    if n == m && source == target {
        return 0;
    }
        
    let inf = n + m;
    let mut matrix = N2Array::new(n+2, m+2, 0);

    matrix[(0, 0)] = inf;
    for i in 0..n+1 {
        matrix[(i+1, 0)] = inf;
        matrix[(i+1, 1)] = i;
    }
    for j in 0..m+1 {
        matrix[(0, j+1)] = inf;
        matrix[(1, j+1)] = j;
    };

    let mut last_row = BTreeMap::new();

    for (row, char_s) in source.chars().enumerate() {
        let mut last_match_col = 0;
        let row = row + 1;
        
        for (col, char_t) in target.chars().enumerate() {
            let col = col + 1;
            let last_match_row = *last_row.get(&char_t).unwrap_or(&0);
            let cost = if char_s == char_t { 0 } else { 1 };

            let dist_add = matrix[(row, col+1)] + 1;
            let dist_del = matrix[(row+1, col)] + 1;
            let dist_sub = matrix[(row, col)] + cost; 
            let dist_trans = matrix[(last_match_row, last_match_col)]
                            + (row - last_match_row - 1) + 1
                            + (col - last_match_col - 1);

            let dist = min(min(dist_add, dist_del), 
                           min(dist_sub, dist_trans));

            matrix[(row+1, col+1)] = dist;
            
            if cost == 0 {
                last_match_col = col;
            }
        }

        last_row.insert(char_s.clone(), row);
    }

    matrix[(n+1, m+1)] as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_levenschtein() {
        let distance = levenshtein("kitten", "sitting");
        assert_eq!(distance, 3);

        let distance = levenshtein("saturday", "sunday");
        assert_eq!(distance, 3);

        let distance = levenshtein("an act", "a cat");
        assert_eq!(distance, 3);

        let distance = levenshtein("", "AA");
        assert_eq!(distance, 2);

        let distance = levenshtein("string", "string");
        assert_eq!(distance, 0);
    }

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

    #[bench]
    fn bench_damerau_levenschtein_long(b: &mut Bencher) {
        let text1 = r"
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
            Curabitur in maximus lectus. Nulla ornare metus sit amet congue feugiat. 
            Nunc sit amet mollis lectus. Integer vel mollis lacus. Nullam molestie justo dui, 
            vitae vulputate augue molestie nec. Sed non urna at augue aliquet feugiat eu ut diam. 
            Nam dignissim semper est, et dignissim lectus. Curabitur luctus mattis mauris. Lorem ipsum dolor sit amet, 
            consectetur adipiscing elit. Morbi at neque a leo mollis bibendum.

            Pellentesque mattis lacus et arcu fermentum, non volutpat orci tincidunt. 
            Nam et blandit magna. Vivamus dignissim in nisi at fringilla. 
            Donec pretium justo justo, vel placerat urna ultricies sit amet. Sed erat felis, 
            commodo non laoreet in, aliquet a lacus. Curabitur condimentum enim elit. 
            Curabitur tincidunt ligula ut quam rutrum fermentum. Donec laoreet mattis porttitor. 
            Etiam ornare urna in congue vehicula. Nam non metus sapien. 
            Morbi vel turpis volutpat, hendrerit augue sed, mattis est. 
            Sed auctor nibh lorem, vitae sagittis nibh egestas eget.
        ";

        let text2 = r"
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
            Curabitur in maximus lectus. Nulla ornare metus sit amet congue feugiat. 
            Nunc sit amet mollis lectus. Integer vel mollis lacus. Nullam molestie justo dui, 
            vitae vulputate augue molestie nec. Sed non urna at augue aliquet feugiat eu ut diam. 
            Nam dignissim semper est, et eignissim lectus. Curabitur luctus mattis mauris. Lorem ipsum dolor sit amet, 
            consectetur adipiscing elit. Morbi at neque a leo mollis bibendum.

            Pellentesque mattis lacus et arcu fermentum, non volutpat orci tincidunt. 
            Nam et blandit magna. Vivamus dignmssim in nisi at fringilla. 
            Donec pretium justo justo, vel placerat urna ultricies sit amet. Sed erat felis, 
            commodo non laoreet in, aliquet a lacus. Curabitur condimentum enim elit. 
            Curabitur tincidunt ligula ut quam rutrum fermentum. Donec laoreet mattis porttitor.             
        ";

        b.iter(|| damerau_levenshtein(text1, text2))
    }
}
