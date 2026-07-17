pub(crate) struct Indent {
    _0: String,
    _1: String,
    _2: String,
    _3: String,
    _4: String,
    _5: String,
    _6: String,
    _7: String,
    _8: String,
    indent: Vec<String>
}

impl Indent {
    pub(crate) fn new() -> Self {
        let mut obj = Self {
            _0: String::new(),
            _1: " ".repeat(4),
            _2: " ".repeat(8),
            _3: " ".repeat(12),
            _4: " ".repeat(16),
            _5: " ".repeat(20),
            _6: " ".repeat(24),
            _7: " ".repeat(28),
            _8: " ".repeat(32),
            indent: vec![],
        };
        obj.indent = vec![
            obj._0.clone(), obj._1.clone(), obj._2.clone(), obj._3.clone(), obj._4.clone(),
            obj._5.clone(), obj._6.clone(), obj._7.clone(), obj._8.clone(),
        ];
        obj
    }
}
