#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub struct PostingList {
    docs: Vec<u32>,
}

impl PostingList {
    pub fn difference(&self, other: &PostingList) -> PostingList {
        let mut result = Vec::with_capacity(self.docs.len() + other.docs.len());

        let mut p1i = 0;
        let mut p2i = 0;

        while p1i != self.docs.len() && p2i != other.docs.len() {
            let doc1 = self.docs[p1i];
            let doc2 = other.docs[p2i];

            if doc1 == doc2 {
                p1i += 1;
                p2i += 1;
            } else if doc1 < doc2 {
                result.push(doc1);
                p1i += 1;
            } else {
                p2i += 1;
            }
        }

        if p1i != self.docs.len() {
            result.extend_from_slice(&self.docs[p1i..]);
        }

        PostingList { docs: result }
    }

    pub fn difference_binsearch(&self, other: &PostingList) -> PostingList {
        let mut diff = Vec::new();

        let mut offset = 0;
        for doc in &self.docs {
            offset = match other.docs[offset..].binary_search(doc) {
                Ok(idx) => idx,
                Err(idx) => {
                    diff.push(*doc);
                    idx
                }
            }
        }

        PostingList { docs: diff }
    }


    pub fn intersect_naive(&self, other: &PostingList) -> PostingList {
        let mut inter = Vec::new();

        for doc1 in &self.docs {
            for doc2 in &other.docs {
                if doc1 == doc2 {
                    inter.push(*doc1);
                    break;
                }
            }
        }

        PostingList { docs: inter }
    }

    pub fn intersect(&self, other: &PostingList) -> PostingList {
        let mut inter = Vec::new();

        let mut p1i = 0;
        let mut p2i = 0;

        while p1i != self.docs.len() && p2i != other.docs.len() {
            let doc1 = self.docs[p1i];
            let doc2 = other.docs[p2i];

            if doc1 == doc2 {
                inter.push(doc1);
                p1i += 1;
                p2i += 1;
            } else if doc1 < doc2 {
                p1i += 1;
            } else {
                p2i += 1;
            }
        }

        PostingList { docs: inter }
    }

    pub fn intersect_binsearch(&self, other: &PostingList) -> PostingList {
        let mut inter = Vec::new();

        let (smaller, larger) = min_max_posting(self, other);

        let mut offset = 0;
        for doc in &smaller.docs {
            offset = match larger.docs[offset..].binary_search(doc) {
                Ok(idx) => {
                    inter.push(*doc);
                    idx
                }
                Err(idx) => idx,
            }
        }

        PostingList { docs: inter }
    }

    pub fn union(&self, other: &PostingList) -> PostingList {
        let mut result = Vec::with_capacity(self.docs.len() + other.docs.len());

        let mut p1i = 0;
        let mut p2i = 0;

        while p1i != self.docs.len() && p2i != other.docs.len() {
            let doc1 = self.docs[p1i];
            let doc2 = other.docs[p2i];

            if doc1 == doc2 {
                result.push(doc1);
                p1i += 1;
                p2i += 1;
            } else if doc1 < doc2 {
                result.push(doc1);
                p1i += 1;
            } else {
                result.push(doc2);
                p2i += 1;
            }
        }

        if p1i != self.docs.len() {
            result.extend_from_slice(&self.docs[p1i..]);
        }

        if p2i != other.docs.len() {
            result.extend_from_slice(&other.docs[p2i..]);
        }

        PostingList { docs: result }
    }
}

fn min_max_posting<'a>(a: &'a PostingList, b: &'a PostingList) -> (&'a PostingList, &'a PostingList) {
    if a.docs.len() < b.docs.len() {
        (a, b)
    } else {
        (b, a)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    quickcheck! {
        fn prop_difference(xs: BTreeSet<u32>, ys: BTreeSet<u32>) -> bool {
            let p1 = PostingList{
                docs: xs.iter().map(ToOwned::to_owned).collect()
            };
            
            let p2 = PostingList{
                docs: ys.iter().map(ToOwned::to_owned).collect()
            };

            let posting_diff = p1.difference(&p2);

            let set_diff: Vec<_> = xs.difference(&ys).map(ToOwned::to_owned).collect();

            posting_diff.docs == set_diff
        }
    }

    quickcheck! {
        fn prop_difference_binsearch(xs: BTreeSet<u32>, ys: BTreeSet<u32>) -> bool {
            let p1 = PostingList{
                docs: xs.iter().map(ToOwned::to_owned).collect()
            };
            
            let p2 = PostingList{
                docs: ys.iter().map(ToOwned::to_owned).collect()
            };

            let posting_diff = p1.difference_binsearch(&p2);

            let set_diff: Vec<_> = xs.difference(&ys).map(ToOwned::to_owned).collect();

            posting_diff.docs == set_diff
        }
    }

    quickcheck! {
        fn prop_intersect(xs: BTreeSet<u32>, ys: BTreeSet<u32>) -> bool {
            let p1 = PostingList{
                docs: xs.iter().map(ToOwned::to_owned).collect()
            };
            
            let p2 = PostingList{
                docs: ys.iter().map(ToOwned::to_owned).collect()
            };

            let posting_isect = p1.intersect(&p2);

            let set_isect: Vec<_> = xs.intersection(&ys).map(ToOwned::to_owned).collect();

            posting_isect.docs == set_isect
        }
    }

    quickcheck! {
        fn prop_intersect_binsearch(xs: BTreeSet<u32>, ys: BTreeSet<u32>) -> bool {
            let p1 = PostingList{
                docs: xs.iter().map(ToOwned::to_owned).collect()
            };
            
            let p2 = PostingList{
                docs: ys.iter().map(ToOwned::to_owned).collect()
            };

            let posting_isect = p1.intersect_binsearch(&p2);

            let set_isect: Vec<_> = xs.intersection(&ys).map(ToOwned::to_owned).collect();

            posting_isect.docs == set_isect
        }
    }


    quickcheck! {
        fn prop_union(xs: BTreeSet<u32>, ys: BTreeSet<u32>) -> bool {
            let p1 = PostingList{
                docs: xs.iter().map(ToOwned::to_owned).collect()
            };
            
            let p2 = PostingList{
                docs: ys.iter().map(ToOwned::to_owned).collect()
            };

            let posting_union = p1.union(&p2);

            let set_union: Vec<_> = xs.union(&ys).map(ToOwned::to_owned).collect();

            posting_union.docs == set_union
        }
    }

}
