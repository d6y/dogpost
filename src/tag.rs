use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Tag(String);

impl Tag {
    fn new(tag: &str) -> Tag {
        Tag(tag.to_string())
    }
}

pub fn detag(subject: &str) -> (String, Vec<Tag>) {
    let (result, tags) = subject.split_whitespace().fold(
        (String::new(), Vec::new()),
        |(mut result, mut tags), word| {
            if let Some(tag) = word.strip_prefix('#') {
                tags.push(Tag::new(tag));
            } else {
                if !result.is_empty() {
                    result.push(' ');
                }
                result.push_str(word);
            }
            (result, tags)
        },
    );

    (result, tags)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detag_empty() {
        let subject = "";
        let expected_result = "";
        let expected_tags = Vec::new();
        assert_eq!(detag(subject), (expected_result.to_string(), expected_tags));
    }

    #[test]
    fn test_detag_start_tag() {
        let subject = "#happy weekend!";
        let expected_result = "weekend!".to_string();
        let expected_tags = vec![Tag::new("happy")];
        assert_eq!(detag(subject), (expected_result, expected_tags));
    }

    #[test]
    fn test_detag_middle_tag() {
        let subject = "I'm feeling #jolly #happy today.";
        let expected_result = "I'm feeling today.".to_string();
        let expected_tags = vec![Tag::new("jolly"), Tag::new("happy")];
        assert_eq!(detag(subject), (expected_result, expected_tags));
    }

    #[test]
    fn test_detag_end_tag() {
        let subject = "I like my #vet";
        let expected_result = "I like my".to_string();
        let expected_tags = vec![Tag::new("vet")];
        assert_eq!(detag(subject), (expected_result, expected_tags));
    }
}
