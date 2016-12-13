// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::UnicodeSegmentation;

use std::prelude::v1::*;

#[test]
fn test_graphemes() {
    use testdata::{TEST_SAME, TEST_DIFF};

    let mut i = 0;
    for &(s, g) in TEST_SAME {
        i += 1;
        // test forward iterator
        assert_eq!(UnicodeSegmentation::graphemes(s, true).collect::<Vec<_>>(),
                   g, "in string {}, {:?}", i, s);
        assert_eq!(UnicodeSegmentation::graphemes(s, false).collect::<Vec<_>>(),
                   g, "in string {}, {:?}", i, s);

        // test reverse iterator
        assert_eq!(UnicodeSegmentation::graphemes(s, true).rev().collect::<Vec<_>>(),
                   g.iter().rev().cloned().collect::<Vec<_>>(),
                   "in string {}, {:?} (reverse)", i, s);
        assert_eq!(UnicodeSegmentation::graphemes(s, false).rev().collect::<Vec<_>>(),
                   g.iter().rev().cloned().collect::<Vec<_>>(),
                   "in string {}, {:?} (reverse)", i, s);
    }

    let mut i = 0;
    for &(s, gt, gf) in TEST_DIFF {
        i += 1;
        // test forward iterator
        assert_eq!(UnicodeSegmentation::graphemes(s, true).collect::<Vec<_>>(), gt,
                   "in string {}, {:?} (extended)", i, s);
        assert_eq!(UnicodeSegmentation::graphemes(s, false).collect::<Vec<_>>(), gf,
                   "in string {}, {:?} (legacy)", i, s);

        // test reverse iterator
        assert_eq!(UnicodeSegmentation::graphemes(s, true).rev().collect::<Vec<_>>(),
                   gt.iter().cloned().rev().collect::<Vec<_>>(),
                   "in string {}, {:?} (extended/rev)", i, s);
        assert_eq!(UnicodeSegmentation::graphemes(s, false).rev().collect::<Vec<_>>(),
                   gf.iter().cloned().rev().collect::<Vec<_>>(),
                   "in string {}, {:?} (legacy/rev)", i, s);
    }

    // test the indices iterators
    let s = "a̐éö̲\r\n";
    let gr_inds = UnicodeSegmentation::grapheme_indices(s, true).collect::<Vec<(usize, &str)>>();
    let b: &[_] = &[(0, "a̐"), (3, "é"), (6, "ö̲"), (11, "\r\n")];
    assert_eq!(gr_inds, b);
    let gr_inds = UnicodeSegmentation::grapheme_indices(s, true).rev().collect::<Vec<(usize, &str)>>();
    let b: &[_] = &[(11, "\r\n"), (6, "ö̲"), (3, "é"), (0, "a̐")];
    assert_eq!(gr_inds, b);
    let mut gr_inds_iter = UnicodeSegmentation::grapheme_indices(s, true);
    {
        let gr_inds = gr_inds_iter.by_ref();
        let e1 = gr_inds.size_hint();
        assert_eq!(e1, (1, Some(13)));
        let c = gr_inds.count();
        assert_eq!(c, 4);
    }
    let e2 = gr_inds_iter.size_hint();
    assert_eq!(e2, (0, Some(0)));

    // make sure the reverse iterator does the right thing with "\n" at beginning of string
    let s = "\n\r\n\r";
    let gr = UnicodeSegmentation::graphemes(s, true).rev().collect::<Vec<&str>>();
    let b: &[_] = &["\r", "\r\n", "\n"];
    assert_eq!(gr, b);
}

#[test]
fn test_words() {
    use testdata::TEST_WORD;

    let mut i = 0;
    for &(s, w) in TEST_WORD {
        i += 1;
        // test forward iterator
        assert_eq!(s.split_word_bounds().collect::<Vec<_>>(),
                   w, "in string {}, {:?}", i, s);

        // test reverse iterator
        assert_eq!(s.split_word_bounds().rev().collect::<Vec<_>>(),
                   w.iter().rev().cloned().collect::<Vec<_>>(),
                   "in string {}, {:?} (reverse)", i, s);

        // generate offsets from word string lengths
        let mut indices = vec![0];
        for i in w.iter().cloned().map(|s| s.len()).scan(0, |t, n| { *t += n; Some(*t) }) {
            indices.push(i);
        }
        indices.pop();
        let indices = indices;

        // test forward indices iterator
        assert!(s.split_word_bound_indices()
                 .zip(indices.iter())
                 .all(|((l,_),m)| l == *m));

        // test backward indices iterator
        assert!(s.split_word_bound_indices().rev()
                 .zip(indices.iter().rev())
                 .all(|((l,_),m)| l == *m));
    }
}
