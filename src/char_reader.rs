
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
    /// 
    /// ### Example:
    /// ```
    /// use telemark::char_reader::CharReader;
    /// 
    /// let mut char_reader = CharReader::new("12");
    /// assert_eq!(char_reader.next_char(), Some(&'1'));
    /// assert_eq!(char_reader.next_char(), Some(&'2'));
    /// assert_eq!(char_reader.next_char(), None);
    /// ```
    pub fn next_char(&mut self) -> Option<&char> {
        let nc = self.chars.get(self.current_position);
        self.current_position += 1;
        return nc;
    }

    /// Go to the previous char, if the reader in the position 0, it will remain on 0.
    /// 
    /// ### Example:
    /// ```
    /// use telemark::char_reader::CharReader;
    /// 
    /// let mut char_reader = CharReader::new("hi");
    /// char_reader.go_back();
    /// assert_eq!(char_reader.next_char(), Some(&'h')); // next is 'i'
    /// char_reader.go_back(); // back to 'h'
    /// assert_eq!(char_reader.next_char(), Some(&'h')); // next is 'i'
    /// assert_eq!(char_reader.next_char(), Some(&'i'));
    /// assert_eq!(char_reader.next_char(), None);
    /// ```
    pub fn go_back(&mut self) {
        if self.current_position > 0 {
            self.current_position -= 1;
        }
    }

    /// Get a string slice from the current position to.
    /// 
    /// ### Example:
    /// ```
    /// use telemark::char_reader::CharReader;
    /// 
    /// let mut char_reader = CharReader::new("hello world!");
    /// if let Some(chars) = char_reader.get_string(5) {
    ///     assert_eq!(chars.iter().collect::<String>(), "hello");
    /// } else {
    ///     assert!(false, "unexpected None!");
    /// }
    /// ```
    pub fn get_string(&self, to: usize) -> Option<&[char]> {
        if self.current_position > to {
            return None;
        }
        return self.chars.get(self.current_position..to);
    }

    /// Get the current position.
    /// 
    /// ### Example:
    /// ```
    /// use telemark::char_reader::CharReader;
    /// 
    /// let mut char_reader = CharReader::new("hello world!");
    /// assert_eq!(char_reader.pos(), 0);
    /// char_reader.next_char();
    /// assert_eq!(char_reader.pos(), 1);
    /// ```
    pub fn pos(&self) -> usize {
        return self.current_position;
    }
}