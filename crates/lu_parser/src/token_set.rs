use std::fmt::Display;

use itertools::Itertools;

use crate::SyntaxKind::{self, ParserInternal};

/// A bit-set of `SyntaxKind`s
#[derive(Clone, Copy, Debug)]
pub struct TokenSet(u128, [SyntaxKind; 8]);

#[allow(unused)]
impl TokenSet {
    pub(crate) fn new(kinds: [SyntaxKind; 8]) -> TokenSet {
        let mut res = 0u128;
        let mut i = 0;
        while i < kinds.len() {
            res |= mask(kinds[i]);
            i += 1
        }
        TokenSet(res, kinds)
    }

    // pub(crate) const fn union(self, other: TokenSet) -> TokenSet {
    //     TokenSet(self.0 | other.0)
    // }

    pub(crate) fn contains(&self, kind: SyntaxKind) -> bool {
        let by_kinds = self.1.contains(&kind);
        let by_mask = self.0 & mask(kind) != 0;
        assert_eq!(by_kinds, by_mask);
        by_mask
    }
}

const fn mask(kind: SyntaxKind) -> u128 {
    1u128 << (kind as usize)
}

impl<const N: usize> From<&[SyntaxKind; N]> for TokenSet {
    fn from(arr: &[SyntaxKind; N]) -> Self {
        assert!(N <= 8);
        let mut kinds = [
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
        ];
        for i in 0..N {
            kinds[i] = arr[i];
        }
        TokenSet::new(kinds)
    }
}

impl<const N: usize> From<[SyntaxKind; N]> for TokenSet {
    fn from(arr: [SyntaxKind; N]) -> Self {
        assert!(N <= 8);
        let mut kinds = [
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
        ];
        for i in 0..N {
            kinds[i] = arr[i];
        }
        TokenSet::new(kinds)
    }
}

impl From<SyntaxKind> for TokenSet {
    fn from(elem: SyntaxKind) -> Self {
        TokenSet::new([
            elem,
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
            ParserInternal,
        ])
    }
}

impl Display for TokenSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kinds = &self.1;
        write!(
            f,
            "{}",
            kinds
                .iter()
                .filter(|kind| *kind != &ParserInternal)
                .format(" or ")
        )
    }
}
