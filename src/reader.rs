
/// The string chars reader.
pub struct CharReader {
    current_position: usize,
    chars: Vec<char>,
}
impl CharReader {
    /// Create new `CharReader`.
    pub fn new(string: &str) -> Self {
        return Self {
            current_position: 0,
            chars: string.chars().collect()
        };
    }

    /// Get the next char, It will get the current char and then it will go to the next.
    pub fn next_char(&mut self) -> Option<&char> {
        let nc = self.chars.get(self.current_position);
        self.current_position += 1;
        return nc;
    }

    /// Go to the previous char, if the reader in the position 0, it will remain on 0.
    pub fn go_back(&mut self) {
        if self.current_position > 0 {
            self.current_position -= 1;
        }
    }

    /// Get a string slice from the current position to.
    pub fn get_string(&self, to: usize) -> Option<&[char]> {
        if self.current_position > self.current_position+to {
            return None;
        }
        return self.chars.get(self.current_position..self.current_position+to);
    }

    /// Get the current position, if the position is not 0, 
    /// It will decrease it by 1.
    pub fn pos(&self) -> usize {
        if self.current_position == 0 {
            return 0;
        }
        return self.current_position - 1;
    }
}

/// The CharReader tests.
#[cfg(test)]
mod char_reader_tests {
    use super::CharReader;

    #[test]
    fn next_char() {
        let mut char_reader = CharReader::new("12");
        assert_eq!(char_reader.next_char(), Some(&'1'));
        assert_eq!(char_reader.next_char(), Some(&'2'));
        assert_eq!(char_reader.next_char(), None);
    }

    #[test]
    fn go_back() {
        let mut char_reader = CharReader::new("hi");
        char_reader.go_back();
        assert_eq!(char_reader.next_char(), Some(&'h')); // next is 'i'
        char_reader.go_back(); // back to 'h'
        assert_eq!(char_reader.next_char(), Some(&'h')); // next is 'i'
        assert_eq!(char_reader.next_char(), Some(&'i'));
        assert_eq!(char_reader.next_char(), None);
    }

    #[test]
    fn get_string() {
        let char_reader = CharReader::new("hello world!");
        if let Some(chars) = char_reader.get_string(5) {
            assert_eq!(chars.iter().collect::<String>(), "hello");
        } else {
            assert!(false, "unexpected None!");
        }
    }

    #[test]
    fn pos() {
        let mut char_reader = CharReader::new("hello world!");
        assert_eq!(char_reader.pos(), 0);
        char_reader.next_char(); // in the first index 'h'
        assert_eq!(char_reader.pos(), 0); 
    }
}


/// The Vec reader.
pub struct VecReader<T> {
    current_position: usize,
    items: Vec<T>,
}
impl<T> VecReader<T> {
    /// Create new `VecReader`.
    pub fn new(items: Vec<T>) -> Self {
        return Self {
            current_position: 0,
            items
        }
    }

    /// Get the next item, It will get the current iitem and then it will go to the next.
    pub fn next(&mut self) -> Option<&T> {
        let n: Option<&T> = self.items.get(self.current_position);
        if self.current_position < self.items.len() {
            self.current_position += 1;
        }
        return n;
    }

    /// Go to the previous item, if the reader in the position 0, it will remain on 0.
    pub fn go_back(&mut self) {
        if self.current_position > 0 {
            self.current_position -= 1;
        }
    }
}

/// The VecReader tests.
#[cfg(test)]
mod vec_reader_tests {
    use super::VecReader;

    #[test]
    fn next() {
        let mut reader = VecReader::new(vec![1,2,3]);
        assert_eq!(reader.next(), Some(&1));
        assert_eq!(reader.next(), Some(&2));
        assert_eq!(reader.next(), Some(&3));
        assert_eq!(reader.next(), None);
    }

    #[test]
    fn go_back() {
        let mut reader = VecReader::new(vec![1,2,3]);
        reader.go_back();
        assert_eq!(reader.next(), Some(&1));
        reader.go_back();
        assert_eq!(reader.next(), Some(&1));
    }
}