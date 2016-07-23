
macro_rules! in_char_ranges_callback {
    ( $([$f:expr, $t: expr]),* ) => {
        |ch| {
            $( (**ch >= $f && **ch <= $t) ||)* false
        }
    }
}

macro_rules! in_chars_callback {
    ( $($c:expr),* ) => {
        |ch| {
            $( (**ch == $c) ||)* false
        }
    }
}

macro_rules! not_in_chars_callback {
    ( $($c:expr),* ) => {
        |ch| {
            $( (**ch != $c) &&)* true
        }
    }
}

macro_rules! is_valid_fn {
    (VARNAME) => (
        in_char_ranges_callback!{['a', 'z'], ['A', 'Z'], ['0', '9'], ['_', '_'], ['$', '$']}
    );
    (NUMBER) => (
        in_char_ranges_callback!{['0', '9'], ['-', '-'], ['.', '.']}
    );
    (OP) => (
        in_chars_callback!{'+', '-', '*', '/', '%', '^', '=', '!', '.'}
    );
    (NOT, $ch:expr) => (
        not_in_chars_callback!($ch)
    );
}

macro_rules! is_valid {
    (VARNAME_START, $c:expr) => ({
        ($c >= 'a' && $c <= 'z') || ($c >= 'A' && $c <= 'Z')
    });
    (NUMBER_START, $c:expr) => ({
        ($c >= '0' && $c <= '9') || $c == '-'
    });
    (DIGIT, $c:expr) => ({
        ($c >= '0' && $c <= '9')
    });
}

macro_rules! add_one_char_token_reader {
    ($tokenizer:expr, $ch:expr, $out:expr) => {{
        $tokenizer.add_token_reader(Box::new(|reader, _| {
            if reader.peek_char() == $ch {
                reader.forward();
                return Some($out);
            }
            None
        }));
    }}
}
