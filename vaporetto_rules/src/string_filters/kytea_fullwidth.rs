use crate::StringFilter;

/// Half-width to full-width filter. This filter works like KyTea's preprocessor.
#[derive(Clone, Default)]
pub struct KyteaFullwidthFilter;

impl StringFilter for KyteaFullwidthFilter {
    fn filter(&self, string: &str) -> String {
        let mut chars: Vec<_> = string.chars().collect();
        for c in &mut chars {
            *c = match *c {
                'a' => 'ａ',
                'b' => 'ｂ',
                'c' => 'ｃ',
                'd' => 'ｄ',
                'e' => 'ｅ',
                'f' => 'ｆ',
                'g' => 'ｇ',
                'h' => 'ｈ',
                'i' => 'ｉ',
                'j' => 'ｊ',
                'k' => 'ｋ',
                'l' => 'ｌ',
                'm' => 'ｍ',
                'n' => 'ｎ',
                'o' => 'ｏ',
                'p' => 'ｐ',
                'q' => 'ｑ',
                'r' => 'ｒ',
                's' => 'ｓ',
                't' => 'ｔ',
                'u' => 'ｕ',
                'v' => 'ｖ',
                'w' => 'ｗ',
                'x' => 'ｘ',
                'y' => 'ｙ',
                'z' => 'ｚ',
                'A' => 'Ａ',
                'B' => 'Ｂ',
                'C' => 'Ｃ',
                'D' => 'Ｄ',
                'E' => 'Ｅ',
                'F' => 'Ｆ',
                'G' => 'Ｇ',
                'H' => 'Ｈ',
                'I' => 'Ｉ',
                'J' => 'Ｊ',
                'K' => 'Ｋ',
                'L' => 'Ｌ',
                'M' => 'Ｍ',
                'N' => 'Ｎ',
                'O' => 'Ｏ',
                'P' => 'Ｐ',
                'Q' => 'Ｑ',
                'R' => 'Ｒ',
                'S' => 'Ｓ',
                'T' => 'Ｔ',
                'U' => 'Ｕ',
                'V' => 'Ｖ',
                'W' => 'Ｗ',
                'X' => 'Ｘ',
                'Y' => 'Ｙ',
                'Z' => 'Ｚ',
                '0' => '０',
                '1' => '１',
                '2' => '２',
                '3' => '３',
                '4' => '４',
                '5' => '５',
                '6' => '６',
                '7' => '７',
                '8' => '８',
                '9' => '９',
                '(' => '（',
                ')' => '）',
                '{' => '｛',
                '}' => '｝',
                '<' => '＜',
                '>' => '＞',
                '｢' => '「',
                '｣' => '」',
                '[' => '［',
                ']' => '］',
                '-' => '−',
                '～' => '〜',
                '.' => '。',
                '－' => 'ー',
                '/' => '／',
                '_' => '＿',
                ',' => '，',
                '%' => '％',
                '?' => '？',
                '､' => '、',
                '―' => 'ー',
                '"' => '”',
                '\'' => '’',
                '･' => '・',
                '─' => 'ー',
                '+' => '＋',
                ':' => '：',
                '–' => 'ー',
                '!' => '！',
                '｡' => '。',
                '&' => '＆',
                '*' => '＊',
                '@' => '＠',
                '=' => '＝',
                c => c,
            };
        }
        chars.into_iter().collect()
    }
}
