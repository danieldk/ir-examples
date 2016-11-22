use std::collections::hash_map::HashMap;
use std::fmt;
use std::io;
use std::io::BufRead;

/// Errors for reading documents.
#[derive(Debug)]
pub enum DocIdError {
    Io(io::Error),
    NoId,
    NoTitle,
    Parse,
}

impl From<io::Error> for DocIdError {
    fn from(err: io::Error) -> DocIdError {
        DocIdError::Io(err)
    }
}

impl fmt::Display for DocIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &DocIdError::Io(ref err) => write!(f, "{}", err),
            &DocIdError::Parse => write!(f, "Could not parse document identifier"),
            &DocIdError::NoId => write!(f, "No identifier found"),
            &DocIdError::NoTitle => write!(f, "No title found"),
        }
    }
}

/// Document identifier to title mapping.
pub struct DocIdentifiers {
    doc_ids: HashMap<usize, String>,
}

impl DocIdentifiers {
    /// Read document identifiers from a buffered reader. The expected format
    /// is: one (document id, title) pair per line, separated by a tab.
    pub fn from_buf_read<R>(reader: R) -> Result<Self, DocIdError>
        where R: BufRead
    {
        let mut doc_ids = HashMap::new();

        for line in reader.lines() {
            let line = try!(line);
            let mut iter = line.split('\t');

            let id = match iter.next() {
                Some(str_id) => try!(str_id.parse().map_err(|_| DocIdError::Parse)),
                None => return Err(DocIdError::NoId),
            };

            let title = match iter.next() {
                Some(title) => title,
                None => return Err(DocIdError::NoTitle),
            };

            doc_ids.insert(id, title.to_owned());
        }

        Ok(DocIdentifiers { doc_ids: doc_ids })
    }

    /// Get the title of a document.
    pub fn get(&self, id: usize) -> Option<&str> {
        self.doc_ids.get(&id).map(String::as_str)
    }
}
