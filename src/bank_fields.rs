use crate::ankidb::AnkiNote;

//TODO change to paths?
#[derive(Debug)]
pub struct BankFields {
    pub audio: String,
    pub image: String,
    pub sentence: String,
}

pub fn extract_note(note: &AnkiNote) -> BankFields {
    let mut parts = note.flds.split('\u{1f}');
    BankFields {
        audio: parts.next().unwrap_or_default().to_string(),
        image: parts.next().unwrap_or_default().to_string(),
        sentence: parts.next().unwrap_or_default().to_string(),
    }
}

pub fn extract_slice(fields: &[AnkiNote]) -> Vec<BankFields> {
    fields.iter().map(extract_note).collect()
}
