use crate::entries::{EntryType, FileEntry};

pub struct ResultList {
    pub state: tui::widgets::ListState,
    pub entries: Vec<FileEntry>,
    file_names_indices: Vec<usize>,
}

impl ResultList {
    pub fn new() -> ResultList {
        ResultList {
            state: tui::widgets::ListState::default(),
            entries: Vec::new(),
            file_names_indices: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: FileEntry) {
        match self.entries.last() {
            Some(e) => {
                let last_header_index = *self.file_names_indices.last().unwrap();
                self.file_names_indices
                    .push(last_header_index + e.matches.len() + 1);
            }
            None => self.file_names_indices.push(0),
        }

        self.entries.push(entry);

        if self.entries.len() == 1 {
            self.state.select(Some(1));
        }
    }

    pub fn next(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        let index = match self.state.selected() {
            Some(i) => {
                let next_index = if self.file_names_indices.contains(&(i + 1)) {
                    i + 2
                } else {
                    i + 1
                };

                if next_index
                    >= self.entries.iter().map(|e| e.matches.len()).sum::<usize>()
                        + self.file_names_indices.len()
                {
                    i
                } else {
                    next_index
                }
            }
            None => 1,
        };

        self.state.select(Some(index));
    }

    pub fn previous(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        let index = match self.state.selected() {
            Some(i) => {
                if i == 1 {
                    1
                } else {
                    if self.file_names_indices.contains(&(i - 1)) {
                        i - 2
                    } else {
                        i - 1
                    }
                }
            }
            None => 1,
        };
        self.state.select(Some(index));
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    // todo: return Option<(&str, u64)>
    pub fn get_selected_entry(&self) -> Option<(String, u64)> {
        match self.state.selected() {
            Some(i) => {
                let file_position = self
                    .file_names_indices
                    .iter()
                    .rposition(|&hi| hi < i)
                    .unwrap();
                if let Some(EntryType::Match(n, _)) =
                    self.entries.iter().map(|item| item.list()).flatten().nth(i)
                {
                    return Some((self.entries[file_position].name.clone(), n));
                }
                None
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entries::Match;
    #[test]
    fn test_empty_list() {
        let mut list = ResultList::new();
        assert_eq!(list.state.selected(), None);
        list.next();
        assert_eq!(list.state.selected(), None);
        list.previous();
        assert_eq!(list.state.selected(), None);
    }

    #[test]
    fn test_add_entry() {
        let mut list = ResultList::new();
        list.add_entry(FileEntry::new("entry1", vec![Match::new(0, "e1m1")]));
        assert_eq!(list.entries.len(), 1);
        assert_eq!(list.file_names_indices.len(), 1);
        assert_eq!(list.state.selected(), Some(1));

        list.add_entry(FileEntry::new(
            "entry2",
            vec![Match::new(0, "e1m2"), Match::new(0, "e2m2")],
        ));
        assert_eq!(list.entries.len(), 2);
        assert_eq!(list.file_names_indices.len(), 2);
        assert_eq!(list.state.selected(), Some(1));
    }
}
