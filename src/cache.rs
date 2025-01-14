use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::hash::Hash;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

/// Définit les méthodes de sérialisation pour la persistance
pub trait Persistent {
    fn serialize(&self) -> String;
    fn deserialize(s: &str) -> Option<Self>
    where
        Self: Sized;
}

/// Cache LRU (Least Recently Used) avec capacité fixe
pub struct Cache<K, V> {
    capacity: usize,           // Nombre maximum d'entrées
    storage: HashMap<K, V>,    // Stockage des paires clé-valeur
    access_order: Vec<K>,      // Ordre d'utilisation (plus récent en dernier)
    file_path: Option<String>, // Chemin du fichier pour la persistance
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone + Persistent,
    V: Persistent,
{
    /// Crée un nouveau cache en mémoire
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            storage: HashMap::with_capacity(capacity),
            access_order: Vec::with_capacity(capacity),
            file_path: None,
        }
    }

    /// Crée un nouveau cache persistant sur disque
    pub fn new_persistent(capacity: usize, file_path: &str) -> io::Result<Self> {
        let mut cache = Cache {
            capacity,
            storage: HashMap::with_capacity(capacity),
            access_order: Vec::with_capacity(capacity),
            file_path: Some(file_path.to_string()),
        };

        if Path::new(file_path).exists() {
            cache.load_from_file()?;
        }

        Ok(cache)
    }

    /// Sauvegarde l'état du cache dans le fichier
    /// Format: nombre d'entrées puis "clé|valeur" par ligne
    fn save_to_file(&self) -> io::Result<()> {
        if let Some(path) = &self.file_path {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)?;
            let mut writer = BufWriter::new(file);

            // Format: nombre d'entrées
            // puis pour chaque entrée: clé|valeur
            writeln!(writer, "{}", self.storage.len())?;

            // Écrire l'ordre d'accès
            for key in &self.access_order {
                if let Some(value) = self.storage.get(key) {
                    writeln!(writer, "{}|{}", key.serialize(), value.serialize())?;
                }
            }
        }
        Ok(())
    }

    /// Charge l'état du cache depuis le fichier
    fn load_from_file(&mut self) -> io::Result<()> {
        if let Some(path) = &self.file_path {
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);
            let mut line = String::new();

            // Lire le nombre d'entrées
            reader.read_line(&mut line)?;
            let count: usize = line.trim().parse().unwrap_or(0);

            self.storage.clear();
            self.access_order.clear();

            // Lire chaque entrée
            for _ in 0..count {
                let mut entry = String::new();
                reader.read_line(&mut entry)?;

                if let Some((key_str, value_str)) = entry.trim().split_once('|') {
                    if let (Some(key), Some(value)) =
                        (K::deserialize(key_str), V::deserialize(value_str))
                    {
                        self.storage.insert(key.clone(), value);
                        self.access_order.push(key);
                    }
                }
            }
        }
        Ok(())
    }

    /// Ajoute ou met à jour une entrée dans le cache
    /// Supprime l'entrée la plus ancienne si le cache est plein
    pub fn put(&mut self, key: K, value: V) {
        if self.storage.contains_key(&key) {
            // Mise à jour d'une entrée existante
            self.storage.insert(key.clone(), value);
            if let Some(pos) = self.access_order.iter().position(|x| x == &key) {
                self.access_order.remove(pos);
                self.access_order.push(key);
            }
            return;
        }

        // Suppression de l'entrée la plus ancienne si nécessaire
        if self.storage.len() >= self.capacity {
            if let Some(lru_key) = self.access_order.first().cloned() {
                self.storage.remove(&lru_key);
                self.access_order.remove(0);
            }
        }

        // Ajout de la nouvelle entrée
        self.storage.insert(key.clone(), value);
        self.access_order.push(key);

        // Persistance si activée
        if self.file_path.is_some() {
            let _ = self.save_to_file();
        }
    }

    /// Récupère une valeur du cache
    /// La marque comme récemment utilisée
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(value) = self.storage.get(key) {
            // Déplacer en fin de liste (plus récemment utilisé)
            if let Some(pos) = self.access_order.iter().position(|x| x == key) {
                let k = self.access_order.remove(pos);
                self.access_order.push(k);
            }

            // Persistance si activée
            if self.file_path.is_some() {
                let _ = self.save_to_file();
            }

            Some(value)
        } else {
            None
        }
    }
}
