/// A `StopwordFilter` filters words based on their lemma and/or tag.
pub trait StopwordFilter {
    fn is_stopword(&self, tag: &str, lemma: &str) -> bool;
}

/// A filter for stopwords with PTB-style tags from TreeTagger.
pub struct PTBStopwordFilter;

impl StopwordFilter for PTBStopwordFilter {
    fn is_stopword(&self, tag: &str, lemma: &str) -> bool {
        if !ptb_open_class(tag) {
            return true;
        }

        if lemma == "be" || lemma == "have" {
            return true;
        }

        false
    }
}

fn ptb_open_class(tag: &str) -> bool {
    tag == "CD" || tag == "FW" || tag.starts_with("JJ") || tag.starts_with("NN") ||
    tag.starts_with("NP") || tag.starts_with("RB") || tag.starts_with("VB")
}
