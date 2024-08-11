use std::collections::HashMap;

struct LetterCount {
    counts: [usize; 26],
}

impl LetterCount {
    pub fn new() -> Self {
        LetterCount { counts: [0; 26] }
    }

    fn increment_count(&mut self, c: char) {
        self.counts[(c as u32 - 'a' as u32) as usize] += 1;
    }

    pub fn from_title(title: String) -> Self {
        let mut lc = LetterCount::new();
        for c in title.chars() {
            lc.increment_count(c)
        }
        return lc
    }
}

impl ToString for LetterCount {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..26 {
            s.push_str(&format!("#{}", self.counts[i]))
        }
        s
    }
}

pub fn group_similar_titles(titles: Vec<String>) -> Vec<Vec<String>> {
    let mut hm = HashMap::<String, Vec<String>>::new();

    for title in &titles {
        let lc = LetterCount::from_title(title.to_string()).to_string();
        if let Some(v) = hm.get_mut(&lc) {
            v.push(title.to_string());
        } else {
            hm.insert(lc, vec![title.to_string()]);
        }
    }
    hm.values().map(|v| v.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_similar_titles_0() {
        let expected = vec![vec!["duel", "dule", "deul"], vec!["speed", "spede"], vec!["cars"]];
        let actual = group_similar_titles(vec![
            "duel".to_string(),
            "dule".into(),
            "speed".into(),
            "spede".into(),
            "deul".into(),
            "cars".into(),
        ]);
        assert_eq!(expected, actual);
    }
}
