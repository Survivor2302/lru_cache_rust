use lru_cache::{AlphabetService, CachedAlphabetService};
use std::time::Instant;

#[test]
fn test_persistent_cache() {
    let cache_path = "test_cache.txt";
    // Supprimer le fichier de cache s'il existe
    if std::path::Path::new(cache_path).exists() {
        println!("Suppression du fichier de cache existant...");
        std::fs::remove_file(cache_path).unwrap();
    }

    let api = AlphabetService::new(100);
    let mut service = CachedAlphabetService::new_persistent(api, 3, cache_path).unwrap();

    println!("\n=== Test de performance du cache LRU avec persistance ===");

    // Séquence de test qui vérifie le comportement LRU
    let test_sequence = [
        'A', 'B', 'C', // Remplir le cache
        'D', // Devrait évincer A
        'B', // Utiliser B (devient le plus récent)
        'E', // Devrait évincer C (pas B car récemment utilisé)
        'A', // Cache miss (A a été évincé)
        'B', // Cache hit
    ];

    for (i, &letter) in test_sequence.iter().enumerate() {
        let start = Instant::now();
        let result = service.get_letter_position(letter);
        let duration = start.elapsed();

        println!("\nÉtape {} - Lettre '{}' :", i + 1, letter);
        println!("Durée : {:?} -> position {:?}", duration, result);

        // Vérifier si c'était probablement un cache hit (durée très courte) ou miss (environ 100ms)
        let was_cache_hit = duration.as_millis() < 50;
        println!("Cache {} !", if was_cache_hit { "HIT" } else { "MISS" });

        // Vérifications spécifiques selon la séquence
        match (i, letter) {
            (3, 'D') => assert!(!was_cache_hit, "D devrait être un cache miss"),
            (4, 'B') => assert!(was_cache_hit, "B devrait être un cache hit"),
            (6, 'A') => assert!(!was_cache_hit, "A devrait être un cache miss car évincé"),
            (7, 'B') => assert!(was_cache_hit, "B devrait toujours être dans le cache"),
            _ => {}
        }
    }

    // Test avec un caractère invalide
    println!("\n=== Test avec caractère invalide ===");
    let start = Instant::now();
    let result = service.get_letter_position('1');
    let duration = start.elapsed();
    println!(
        "Résultat avec '1' : {:?} (durée: {:?} - {})",
        result,
        duration,
        if duration.as_millis() < 50 {
            "depuis le cache"
        } else {
            "depuis l'API"
        }
    );
    assert_eq!(
        result, None,
        "Les caractères non-alphabétiques devraient retourner None"
    );

    // Test de la casse (majuscules et minuscules)
    println!("\n=== Test de la casse ===");
    let start = Instant::now();
    let uppercase = service.get_letter_position('Z');
    let duration = start.elapsed();
    println!(
        "Résultat avec 'Z' : {:?} (durée: {:?} - {})",
        uppercase,
        duration,
        if duration.as_millis() < 50 {
            "depuis le cache"
        } else {
            "depuis l'API"
        }
    );

    let start = Instant::now();
    let lowercase = service.get_letter_position('z');
    let duration = start.elapsed();
    println!(
        "Résultat avec 'z' : {:?} (durée: {:?} - {})",
        lowercase,
        duration,
        if duration.as_millis() < 50 {
            "depuis le cache"
        } else {
            "depuis l'API"
        }
    );
    assert_eq!(
        uppercase, lowercase,
        "La casse ne devrait pas affecter le résultat"
    );

    // Test des limites du cache
    println!("\n=== Test des limites du cache ===");
    for letter in ['X', 'Y', 'Z', 'X'] {
        let start = Instant::now();
        let result = service.get_letter_position(letter);
        let duration = start.elapsed();
        println!(
            "Position de '{}' : {:?} (durée: {:?} - {})",
            letter,
            result,
            duration,
            if duration.as_millis() < 50 {
                "depuis le cache"
            } else {
                "depuis l'API"
            }
        );
    }
    let start = Instant::now();
    let result = service.get_letter_position('X');
    let duration = start.elapsed();
    println!("Durée pour 'X' : {:?} -> position {:?}", duration, result);
    assert!(
        duration.as_millis() < 50,
        "X devrait être dans le cache après une séquence qui ne dépasse pas la taille du cache"
    );

    // Test de surcharge du cache
    println!("\n=== Test de surcharge du cache ===");
    for letter in ['P', 'Q', 'R', 'S'] {
        let start = Instant::now();
        let result = service.get_letter_position(letter);
        let duration = start.elapsed();
        println!(
            "Position de '{}' : {:?} (durée: {:?} - {})",
            letter,
            result,
            duration,
            if duration.as_millis() < 50 {
                "depuis le cache"
            } else {
                "depuis l'API"
            }
        );
    }
    let start = Instant::now();
    let result = service.get_letter_position('P');
    let duration = start.elapsed();
    println!("Durée pour 'P' : {:?} -> position {:?}", duration, result);
    assert!(
        duration.as_millis() >= 50,
        "P devrait avoir été évincé du cache après une surcharge"
    );
}
