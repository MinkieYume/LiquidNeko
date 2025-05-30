struct Symbols {
    s_exp_begin_char:char,
    s_exp_end_char:char,
    quote_begin_char:char,
    quote_end_char:char,
}

impl Symbols {
    pub fn init(&mut self) {
        self.s_exp_begin_char = '(';
        self.s_exp_end_char = ')';
        self.quote_begin_char = '"';
        self.quote_end_char = '"';
    }
    
    pub fn pair_char(&mut self,s:&str) {
        
    }
}
