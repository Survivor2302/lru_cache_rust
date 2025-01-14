# Cache LRU avec API Alphabet 🔤

## 📝 Description

Un cache LRU (Least Recently Used) performant qui s'interface avec une API de service alphabet. Cette solution offre une gestion efficace du cache avec persistance optionnelle des données.

## ✨ Fonctionnalités principales

- 🚀 Cache LRU avec capacité configurable
- 🌐 Simulation d'API avec latence réseau paramétrable
- 💾 Persistance des données sur disque (optionnelle)
- 🔄 Gestion intelligente de la casse
- ✅ Suite complète de tests unitaires

## 🗂️ Structure du projet

```
.
├── src/
│ ├── lib.rs # Point d'entrée et structure principale
│ ├── api_service.rs # Service API simulé
│ └── cache.rs # Implémentation du cache LRU
└── tests/
├── TU_cache.rs # Tests du cache en mémoire
└── TU_persistent_cache.rs # Tests du cache persistant
```

## 🚀 Utilisation

### Cache simple

```
let api = AlphabetService::new(100); // délai de 100ms
let mut service = CachedAlphabetService::new(api, 3); // capacité de 3
let position = service.get_letter_position('A'); // Retourne Some(1)
```

AVEC PERSISTANCE:

```
let mut service = CachedAlphabetService::new_persistent(api, 3, "cache.txt").unwrap();
```

## 🧪 Tests

Pour lancer tous les tests:

```
cargo test
```

Pour un test spécifique:

avec cache en mémoire :

```
cargo test test_cache -- --nocapture
```

avec cache persistant :

```
cargo test test_persistent_cache -- --nocapture
```

## ⚙️ Comportement du cache

- ✨ Maintient les N dernières entrées utilisées
- 🔄 Politique LRU : supprime l'entrée la moins récemment utilisée
- ⚡ Accès rapide (~0ms) vs appels API (100ms par défaut)
- 💾 Sauvegarde automatique en mode persistant

## 📊 Performance

| Opération   | Temps de réponse     |
| ----------- | -------------------- |
| Accès cache | ~0ms                 |
| Appel API   | 100ms (configurable) |
| Persistance | Légère latence       |

## ⚠️ Limitations actuelles

- Supporte uniquement les lettres A-Z (majuscules/minuscules)
- Persistance synchrone uniquement
- Gestion d'erreurs basique
