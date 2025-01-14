use std::{thread, time::Duration};

/// Convertit une valeur en position dans une séquence
pub trait PositionConverter<T> {
    fn to_position(&self, value: T) -> Option<u32>;
}

/// Service simulant une API qui convertit les lettres en position (1-26)
pub struct AlphabetService {
    delay_ms: u64, // Délai simulant une latence réseau
}

impl AlphabetService {
    /// Crée un nouveau service avec le délai spécifié
    pub fn new(delay_ms: u64) -> Self {
        Self { delay_ms }
    }

    /// Obtient la position d'une lettre dans l'alphabet
    pub fn get_letter_position(&self, letter: char) -> Option<u32> {
        self.to_position(letter)
    }
}

impl PositionConverter<char> for AlphabetService {
    fn to_position(&self, letter: char) -> Option<u32> {
        // Simulation d'une latence réseau
        thread::sleep(Duration::from_millis(self.delay_ms));

        let uppercase = letter.to_ascii_uppercase();
        if uppercase >= 'A' && uppercase <= 'Z' {
            Some((uppercase as u32) - ('A' as u32) + 1)
        } else {
            None
        }
    }
}
