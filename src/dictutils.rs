use jmdict::{Entry, SenseInfo};

pub fn find_entry_by_word(word: &str) -> Option<Entry> {
    jmdict::entries().find(|e| {
        e.reading_elements().any(|r| r.text == word) || e.kanji_elements().any(|k| k.text == word)
    })
}
pub fn try_find_entry_by_number(number: u32) -> Option<Entry> {
    jmdict::entries().find(|e| e.number == number)
}
pub fn find_entry_by_number(number: u32) -> Entry {
    self::try_find_entry_by_number(number).expect("Entry not found")
}

pub trait EntryExt {
    fn common_text_form(&self) -> String;
    fn usually_written_using_kana(&self) -> bool;
    fn word_in_kana(&self) -> String;
}
impl EntryExt for Entry {
    fn usually_written_using_kana(&self) -> bool {
        self.senses().any(|s| {
            s.infos()
                .any(|s| s == SenseInfo::UsuallyWrittenUsingKanaAlone)
        })
    }
    fn word_in_kana(&self) -> String {
        self.reading_elements()
            .find(|x| x.priority.is_common())
            .or_else(|| {
                self.reading_elements()
                    .min_by_key(|x| x.priority.frequency_bucket)
            })
            .unwrap()
            .text
            .to_string()
    }
    fn common_text_form(&self) -> String {
        if self.usually_written_using_kana() {
            self.word_in_kana()
        } else {
            self.kanji_elements()
                .find(|x| x.priority.is_common())
                .or_else(|| {
                    self.kanji_elements()
                        .min_by_key(|x| x.priority.frequency_bucket)
                })
                .unwrap()
                .text
                .to_string()
        }
    }
}
