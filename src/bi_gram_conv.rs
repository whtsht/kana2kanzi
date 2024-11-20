use std::collections::HashSet;

use crate::{bigram_db::BigramDB, dict_db::DictDB};

struct SubstringIterator<'a> {
    string: &'a str,
    end: usize,
}

impl<'a> SubstringIterator<'a> {
    fn new(string: &'a str) -> Self {
        Self { string, end: 0 }
    }
}

impl<'a> Iterator for SubstringIterator<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.end >= self.string.len() {
            None
        } else {
            self.end += self.string[self.end..].chars().next()?.len_utf8();
            Some((&self.string[..self.end], &self.string[self.end..]))
        }
    }
}

#[derive(Debug)]
pub struct Kana2kanziConverter {
    pub dict: DictDB,
    pub bigram: BigramDB,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Candidate {
    pub word: String,
    pub start: usize,
    pub end: usize,
}

impl Kana2kanziConverter {
    pub fn new() -> Self {
        let dict = DictDB::new();
        let bigram = BigramDB::new().unwrap();
        Kana2kanziConverter { dict, bigram }
    }
}

impl Default for Kana2kanziConverter {
    fn default() -> Self {
        Self::new()
    }
}

pub fn find_candidate(conv: &Kana2kanziConverter, kana: &str, start: usize) -> Vec<Candidate> {
    if kana.is_empty() {
        return vec![];
    }

    let mut candidates = HashSet::new();
    for (sub, rest) in SubstringIterator::new(kana) {
        let kanzis = conv.dict.get_kanzis(sub);
        if !kanzis.is_empty() {
            let end = start + sub.chars().count();
            let mut new_candidates = kanzis
                .into_iter()
                .map(|kanzi| Candidate {
                    word: kanzi,
                    start,
                    end,
                })
                .collect::<Vec<_>>();
            new_candidates.extend(find_candidate(conv, rest, end));

            for c in new_candidates {
                candidates.insert(c);
            }
        }
    }
    candidates.into_iter().collect()
}

pub fn kana2kanzi(conv: &Kana2kanziConverter, kana: &str) -> (String, f64) {
    let candidates = find_candidate(conv, kana, 0);
    let n = kana.chars().count();
    let mut dp = vec![0.0; n + 1];
    let mut traces: Vec<Vec<String>> = vec![vec![]; n + 1];

    for end in 0..=n {
        for start in 0..end {
            if start == 0 {
                for c in candidates
                    .iter()
                    .filter(|c| c.start == start && c.end == end)
                {
                    let prob = conv.bigram.get_probability(BigramDB::BOS, &c.word).unwrap();
                    if prob >= dp[end] {
                        dp[end] = prob;
                        traces[end] = vec![c.word.clone()];
                    }
                }
            } else {
                for c in candidates
                    .iter()
                    .filter(|c| c.start == start && c.end == end)
                {
                    if let Some(last_word) = traces[start].last() {
                        let prob = conv.bigram.get_probability(last_word, &c.word).unwrap();
                        if dp[start] * prob >= dp[end] {
                            dp[end] = dp[start] * prob;
                            let mut new_words = traces[start].clone();
                            new_words.push(c.word.clone());
                            traces[end] = new_words;
                        }
                    }
                }
            }
        }
    }

    (traces[n].join(""), dp[n])
}

fn generate_replacements(input: &str) -> Vec<String> {
    let hiragana = "あいうえお\
                    かきくけこがぎぐげご\
                    さしすせそざじずぜぞ\
                    たちつてとだぢづでど\
                    なにぬねの\
                    はひふへほばびぶべぼぱぴぷぺぽ\
                    まみむめも\
                    やゆよ\
                    らりるれろ\
                    わをん";

    let mut results = Vec::new();

    for i in 0..input.chars().count() {
        for ch in hiragana.chars() {
            let mut chars: Vec<char> = input.chars().collect();
            chars[i] = ch;
            results.push(chars.iter().collect());
        }
    }

    results
}

fn fix_typo(input: &str, conv: &Kana2kanziConverter, word_len: usize) -> (String, f64) {
    let input_vec = input.chars().collect::<Vec<_>>();

    let mut max_score = 0.0;
    let mut max_kanzi = String::new();

    for (idx, i) in input_vec
        .windows(word_len)
        .map(|x| x.iter().collect::<String>())
        .enumerate()
    {
        let replacements = generate_replacements(&i);

        for replacement in replacements {
            if !conv.dict.get_kanzis(&replacement).is_empty() {
                let kana = input.replace(
                    &input.chars().skip(idx).take(3).collect::<String>(),
                    &replacement,
                );
                let (kanzi, score) = kana2kanzi(conv, &kana);
                if score > max_score {
                    max_score = score;
                    max_kanzi = kanzi;
                }
            }
        }
    }

    (max_kanzi, max_score)
}

pub fn kana2kanzi_with_typo(conv: &Kana2kanziConverter, input: &str) -> String {
    let (kanzi, score) = kana2kanzi(conv, input);
    if score > 0.0 {
        return kanzi;
    }

    let mut max_score = 0.0;
    let mut max_kanzi = String::new();
    // 3文字以上の文字列に対して、誤字を訂正する
    for i in 3..=input.chars().count() {
        let (kanzi, score) = fix_typo(input, conv, i);
        if score > max_score {
            max_score = score;
            max_kanzi = kanzi;
        }
    }

    max_kanzi
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_substring_iterator() {
        let mut iter = super::SubstringIterator::new("かれがくるまでまつ");
        assert_eq!(iter.next(), Some(("か", "れがくるまでまつ")));
        assert_eq!(iter.next(), Some(("かれ", "がくるまでまつ")));
        assert_eq!(iter.next(), Some(("かれが", "くるまでまつ")));
        assert_eq!(iter.next(), Some(("かれがく", "るまでまつ")));
        assert_eq!(iter.next(), Some(("かれがくる", "までまつ")));
        assert_eq!(iter.next(), Some(("かれがくるま", "でまつ")));
        assert_eq!(iter.next(), Some(("かれがくるまで", "まつ")));
        assert_eq!(iter.next(), Some(("かれがくるまでま", "つ")));
        assert_eq!(iter.next(), Some(("かれがくるまでまつ", "")));
    }

    #[test]
    fn test_find_candidate() {
        let conv = super::Kana2kanziConverter::new();
        let candidates = super::find_candidate(&conv, "かれがくるまでまつ", 0);

        assert!(candidates.contains(&super::Candidate {
            word: "彼".to_string(),
            start: 0,
            end: 2
        }));
        assert!(candidates.contains(&super::Candidate {
            word: "が".to_string(),
            start: 2,
            end: 3
        }));
        assert!(candidates.contains(&super::Candidate {
            word: "来る".to_string(),
            start: 3,
            end: 5
        }));
        assert!(candidates.contains(&super::Candidate {
            word: "まで".to_string(),
            start: 5,
            end: 7
        }));
        assert!(candidates.contains(&super::Candidate {
            word: "待つ".to_string(),
            start: 7,
            end: 9
        }));
    }

    #[test]
    fn test_kana2kanzi() {
        let conv = super::Kana2kanziConverter::new();
        let kanzi = super::kana2kanzi(&conv, "かれがくるまでまつ");
        assert_eq!(kanzi.0, "彼が来るまで待つ");
    }
}
