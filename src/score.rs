use serde::{Deserialize, Serialize};

pub const REQUIRED_QUALIFICATIONS_MAX: u8 = 40;
pub const SKILLS_AND_RESPONSIBILITIES_MAX: u8 = 30;
pub const RELEVANT_ACHIEVEMENTS_MAX: u8 = 20;
pub const KEYWORDS_AND_CLARITY_MAX: u8 = 10;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CategoryScores {
    pub required_qualifications: u8,
    pub skills_and_responsibilities: u8,
    pub relevant_achievements: u8,
    pub keywords_and_clarity: u8,
}

pub fn total(scores: &CategoryScores) -> Result<u8, String> {
    check(
        "required_qualifications",
        scores.required_qualifications,
        REQUIRED_QUALIFICATIONS_MAX,
    )?;
    check(
        "skills_and_responsibilities",
        scores.skills_and_responsibilities,
        SKILLS_AND_RESPONSIBILITIES_MAX,
    )?;
    check(
        "relevant_achievements",
        scores.relevant_achievements,
        RELEVANT_ACHIEVEMENTS_MAX,
    )?;
    check(
        "keywords_and_clarity",
        scores.keywords_and_clarity,
        KEYWORDS_AND_CLARITY_MAX,
    )?;

    Ok(scores.required_qualifications
        + scores.skills_and_responsibilities
        + scores.relevant_achievements
        + scores.keywords_and_clarity)
}

fn check(name: &str, value: u8, maximum: u8) -> Result<(), String> {
    if value > maximum {
        return Err(format!("{name} must be between 0 and {maximum}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_every_category_at_its_bounds() {
        assert_eq!(
            total(&CategoryScores {
                required_qualifications: REQUIRED_QUALIFICATIONS_MAX,
                skills_and_responsibilities: SKILLS_AND_RESPONSIBILITIES_MAX,
                relevant_achievements: RELEVANT_ACHIEVEMENTS_MAX,
                keywords_and_clarity: KEYWORDS_AND_CLARITY_MAX,
            }),
            Ok(100)
        );
        assert_eq!(
            total(&CategoryScores {
                required_qualifications: 0,
                skills_and_responsibilities: 0,
                relevant_achievements: 0,
                keywords_and_clarity: 0,
            }),
            Ok(0)
        );
    }

    #[test]
    fn rejects_each_category_above_its_maximum() {
        for scores in [
            CategoryScores {
                required_qualifications: 41,
                skills_and_responsibilities: 0,
                relevant_achievements: 0,
                keywords_and_clarity: 0,
            },
            CategoryScores {
                required_qualifications: 0,
                skills_and_responsibilities: 31,
                relevant_achievements: 0,
                keywords_and_clarity: 0,
            },
            CategoryScores {
                required_qualifications: 0,
                skills_and_responsibilities: 0,
                relevant_achievements: 21,
                keywords_and_clarity: 0,
            },
            CategoryScores {
                required_qualifications: 0,
                skills_and_responsibilities: 0,
                relevant_achievements: 0,
                keywords_and_clarity: 11,
            },
        ] {
            assert!(total(&scores).is_err());
        }
    }
}
