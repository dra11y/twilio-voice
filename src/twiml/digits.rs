use std::{
    fmt,
    ops::{Deref, Index},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{Result, errors::DigitsError};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Digits(Vec<Digit>);

impl fmt::Debug for Digits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Digits")
            .field(&self.0.iter().map(AsRef::as_ref).collect::<String>())
            .finish()
    }
}

impl Deref for Digits {
    type Target = [Digit];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Digits {
    // pub fn iter(&self) -> std::slice::Iter<'_, Digit> {
    //     self.0.iter()
    // }

    // pub fn len(&self) -> usize {
    //     self.0.len()
    // }

    // pub fn is_empty(&self) -> bool {
    //     self.0.is_empty()
    // }

    /// Return the integer value of the leading numeric digits if all non-numeric digits appear after all numeric digits; otherwise return None.
    pub fn to_int(&self) -> Result<u64> {
        if self.is_empty() {
            return Err(DigitsError::Empty.into());
        }

        // First digit must be numeric or a pause.
        if !(self.0[0].to_int().is_some() || self.0[0].is_pause()) {
            return Err(DigitsError::FirstDigitNotNumeric.into());
        }

        // Convert only the numeric part to integer.
        let mut result = 0u64;
        let mut found_digit = false;
        let mut found_non_digit = false;
        for digit in &self.0 {
            // Always treat sequence containing any alphabetic as non-numeric.
            if digit.is_alpha() {
                return Err(DigitsError::ContainsAlphabetic.into());
            }

            // Ignore pauses.
            if digit.is_pause() {
                continue;
            }

            let Some(int) = digit.to_int() else {
                found_non_digit = true;
                continue;
            };

            // Found a digit after a non-digit.
            if found_non_digit {
                return Err(DigitsError::NumericAfterNonNumeric.into());
            }

            // Check for potential overflow.
            if result > u64::MAX / 10 {
                return Err(DigitsError::Overflow.into());
            }

            found_digit = true;
            result = result * 10 + int;
        }

        if !found_digit {
            return Err(DigitsError::NoNumeric.into());
        }

        Ok(result)
    }
}

// TwiML Voice: Voices List:
// https://www.twilio.com/docs/voice/twiml/say/text-speech#available-voices-and-languages

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, strum::AsRefStr, strum::Display, Serialize, Deserialize,
)]
pub enum Digit {
    #[strum(serialize = "0")]
    #[serde(rename = "0")]
    Zero,
    #[strum(serialize = "1")]
    #[serde(rename = "1")]
    One,
    #[strum(serialize = "2")]
    #[serde(rename = "2")]
    Two,
    #[strum(serialize = "3")]
    #[serde(rename = "3")]
    Three,
    #[strum(serialize = "4")]
    #[serde(rename = "4")]
    Four,
    #[strum(serialize = "5")]
    #[serde(rename = "5")]
    Five,
    #[strum(serialize = "6")]
    #[serde(rename = "6")]
    Six,
    #[strum(serialize = "7")]
    #[serde(rename = "7")]
    Seven,
    #[strum(serialize = "8")]
    #[serde(rename = "8")]
    Eight,
    #[strum(serialize = "9")]
    #[serde(rename = "9")]
    Nine,
    #[strum(serialize = "*")]
    #[serde(rename = "*")]
    Star,
    #[strum(serialize = "#")]
    #[serde(rename = "#")]
    Pound,
    A,
    B,
    C,
    D,
    /// 0.5-second pause (lowercase 'w' in API)
    #[strum(serialize = "w")]
    #[serde(rename = "w")]
    W,
    /// 1-second pause (UPPERCASE 'W' in API)
    #[strum(serialize = "W")]
    #[serde(rename = "W")]
    WW,
}

impl Digit {
    pub fn is_alpha(&self) -> bool {
        matches!(self, Digit::A | Digit::B | Digit::C | Digit::D)
    }

    pub fn is_pause(&self) -> bool {
        matches!(self, Digit::W | Digit::WW)
    }

    pub fn to_int(&self) -> Option<u64> {
        match self {
            Digit::Zero => Some(0),
            Digit::One => Some(1),
            Digit::Two => Some(2),
            Digit::Three => Some(3),
            Digit::Four => Some(4),
            Digit::Five => Some(5),
            Digit::Six => Some(6),
            Digit::Seven => Some(7),
            Digit::Eight => Some(8),
            Digit::Nine => Some(9),
            _ => None,
        }
    }
}

impl TryFrom<char> for Digit {
    type Error = ();

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        match c {
            '0' => Ok(Digit::Zero),
            '1' => Ok(Digit::One),
            '2' => Ok(Digit::Two),
            '3' => Ok(Digit::Three),
            '4' => Ok(Digit::Four),
            '5' => Ok(Digit::Five),
            '6' => Ok(Digit::Six),
            '7' => Ok(Digit::Seven),
            '8' => Ok(Digit::Eight),
            '9' => Ok(Digit::Nine),
            '*' => Ok(Digit::Star),
            '#' => Ok(Digit::Pound),
            'A' => Ok(Digit::A),
            'B' => Ok(Digit::B),
            'C' => Ok(Digit::C),
            'D' => Ok(Digit::D),
            'w' => Ok(Digit::W),
            'W' => Ok(Digit::WW),
            _ => Err(()),
        }
    }
}

impl Index<usize> for Digits {
    type Output = Digit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IntoIterator for Digits {
    type Item = Digit;
    type IntoIter = std::vec::IntoIter<Digit>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Digits {
    type Item = &'a Digit;
    type IntoIter = std::slice::Iter<'a, Digit>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl fmt::Display for Digits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(AsRef::as_ref).collect::<String>()
        )
    }
}

impl FromStr for Digits {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut digits = Vec::with_capacity(s.len());
        for c in s.chars() {
            digits.push(Digit::try_from(c)?);
        }
        Ok(Digits(digits))
    }
}

impl FromIterator<Digit> for Digits {
    fn from_iter<I: IntoIterator<Item = Digit>>(iter: I) -> Self {
        Digits(iter.into_iter().collect())
    }
}

impl From<Vec<Digit>> for Digits {
    fn from(vec: Vec<Digit>) -> Self {
        Digits(vec)
    }
}

impl From<Digits> for Vec<Digit> {
    fn from(digits: Digits) -> Self {
        digits.0
    }
}

impl Serialize for Digits {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Digits {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        let vec = string
            .chars()
            .filter_map(|c| Digit::try_from(c).ok())
            .collect();
        Ok(Digits(vec))
    }
}

#[cfg(test)]
mod tests {
    use crate::twiml::Digit;

    use super::*;

    #[test]
    fn test_digits_from_str() {
        let digits = Digits::from_str("123wABWCD").unwrap();
        assert_eq!(digits.to_string(), "123wABWCD");
        assert!(digits.to_int().is_err());
        assert_eq!(digits[3], Digit::W);
        assert_eq!(digits[5], Digit::B);
        assert_eq!(digits[6], Digit::WW);
    }

    #[test]
    fn test_digits() {
        let digits = Digits::from_str("*31#").unwrap();
        assert_eq!(digits.to_string(), "*31#");
        assert!(digits.to_int().is_err());
        assert_eq!(digits.iter().next().unwrap(), &Digit::Star);

        let digits = Digits::from_str("31").unwrap();
        assert_eq!(digits.to_string(), "31");
        assert_eq!(digits.to_int().unwrap(), 31);
        assert_eq!(digits.iter().next().unwrap(), &Digit::Three);

        let digits = Digits::from_str("31#*").unwrap();
        assert_eq!(digits.to_string(), "31#*");
        assert_eq!(digits.to_int().unwrap(), 31);
        assert_eq!(digits.iter().next().unwrap(), &Digit::Three);

        let digits = Digits::from_str("31*31").unwrap();
        assert_eq!(digits.to_string(), "31*31");
        assert!(digits.to_int().is_err());
        assert_eq!(digits.iter().next().unwrap(), &Digit::Three);
    }
}
