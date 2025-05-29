struct ReaderChars{
    s_exp_begin_char:char,
    s_exp_end_char:char,
    quote_begin_char:char,
    quote_end_char:char,
}

impl ReaderChars{
    pub fn init(&mut self) {
        self.s_exp_begin_char = '(';
        self.s_exp_end_char = ')';
        self.quote_begin_char = '"';
        self.quote_end_char = '"';
    }
    
    // Getter methods
    pub fn get_s_exp_begin_char(&self) -> char {
        self.s_exp_begin_char
    }

    pub fn get_s_exp_end_char(&self) -> char {
        self.s_exp_end_char
    }

    pub fn get_quote_begin_char(&self) -> char {
        self.quote_begin_char
    }

    pub fn get_quote_end_char(&self) -> char {
        self.quote_end_char
    }

    // Setter methods
    pub fn set_s_exp_begin_char(&mut self, c: char) {
        self.s_exp_begin_char = c;
    }

    pub fn set_s_exp_end_char(&mut self, c: char) {
        self.s_exp_end_char = c;
    }

    pub fn set_quote_begin_char(&mut self, c: char) {
        self.quote_begin_char = c;
    }

    pub fn set_quote_end_char(&mut self, c: char) {
        self.quote_end_char = c;
    }
}
