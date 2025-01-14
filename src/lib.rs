mod api_service;
mod cache;

use cache::Persistent;
use std::io;

pub use api_service::AlphabetService;
pub use cache::Cache;

/// Service combinant une API alphabet avec un cache
/// pour optimiser les requêtes fréquentes
pub struct CachedAlphabetService {
    api: AlphabetService,    // Service API alphabet
    cache: Cache<char, u32>, // Cache LRU des positions des lettres
}

// Implémentations de sérialisation pour le cache persistant
impl Persistent for char {
    fn serialize(&self) -> String {
        self.to_string()
    }

    fn deserialize(s: &str) -> Option<Self> {
        s.chars().next()
    }
}

impl Persistent for u32 {
    fn serialize(&self) -> String {
        self.to_string()
    }

    fn deserialize(s: &str) -> Option<Self> {
        s.parse().ok()
    }
}

impl CachedAlphabetService {
    /// Crée une nouvelle instance avec cache en mémoire
    ///
    /// # Arguments
    /// * `api` - Service API alphabet
    /// * `cache_capacity` - Nombre maximum d'entrées dans le cache
    pub fn new(api: AlphabetService, cache_capacity: usize) -> Self {
        Self {
            api,
            cache: Cache::new(cache_capacity),
        }
    }

    /// Crée une nouvelle instance avec cache persistant sur disque
    ///
    /// # Arguments
    /// * `api` - Service API alphabet
    /// * `cache_capacity` - Nombre maximum d'entrées dans le cache
    /// * `cache_file` - Chemin du fichier de cache
    pub fn new_persistent(
        api: AlphabetService,
        cache_capacity: usize,
        cache_file: &str,
    ) -> io::Result<Self> {
        Ok(Self {
            api,
            cache: Cache::new_persistent(cache_capacity, cache_file)?,
        })
    }
    
    /// Récupère la position (1-26) d'une lettre dans l'alphabet
    ///
    /// Consulte d'abord le cache avant d'appeler l'API.
    /// Retourne None si la lettre n'est pas dans l'alphabet.
    pub fn get_letter_position(&mut self, letter: char) -> Option<u32> {
        if let Some(&position) = self.cache.get(&letter) {
            return Some(position); // Retour rapide si présent dans le cache
        }

        // Appel API si absent du cache
        if let Some(position) = self.api.get_letter_position(letter) {
            self.cache.put(letter, position);
            Some(position)
        } else {
            None
        }
    }
}
