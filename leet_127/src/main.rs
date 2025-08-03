fn main() {
    println!("Hello, world!");
}

fn diff(word1: &str, word2: &str) -> usize {
    let mut diff: usize = 0;
    for (cw1, cw2) in word1.chars().zip(word2.chars()) {
        if cw1 != cw2 {
            diff += 1
        }
    }
    diff
}

fn diff_with_all(word: &str, wordlist: &[String]) -> Vec<usize> {
    let mut diffs: Vec<usize> = Vec::new();

    for other_word in wordlist {
        diffs.push(diff(word, other_word))
    }

    diffs
}

pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> usize {
    let mut current_level = vec![begin_word];
    let mut possible_jumps = word_list.clone();
    let mut path: usize = 0;

    while !possible_jumps.is_empty() {
        path += 1;
        let mut next_jumps: Vec<String> = Vec::new();
        for word in current_level.into_iter() {
            let diffs: Vec<usize> = diff_with_all(&word, &possible_jumps);
            for (ind, diff) in diffs.iter().enumerate() {
                if *diff == 1 {
                    next_jumps.push(possible_jumps[ind].clone());
                    if possible_jumps[ind] == end_word {
                        path += 1;
                        return path;
                    }
                }
            }
        }
        if next_jumps.is_empty() {
            return 0;
        }
        let visited = next_jumps.iter().collect::<std::collections::HashSet<_>>();
        possible_jumps.retain(|x| !visited.contains(x));

        current_level = next_jumps;
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn leetcode_example_1() {
        let words = ["hot", "dot", "dog", "lot", "log", "cog"];
        let wordlist: Vec<String> = words.iter().map(|&s| s.to_string()).collect();
        let begin_word = String::from("hit");
        let end_word = String::from("cog");

        let path = ladder_length(begin_word, end_word, wordlist);
        assert_eq!(path, 5);
    }

    #[test]
    fn no_diff() {
        // set up
        let word1 = String::from("ABC");
        let word2 = String::from("ABC");

        assert_eq!(diff(&word1, &word2), 0);
    }
    #[test]
    fn diff_one() {
        //set up
        let word1 = String::from("ABCD");
        let word2 = String::from("BBCD");

        assert_eq!(diff(&word1, &word2), 1);
    }
    #[test]
    fn diff_two() {
        //set up
        let word1 = String::from("ABCD");
        let word2 = String::from("CCCD");

        assert_eq!(diff(&word1, &word2), 2);
    }
}
