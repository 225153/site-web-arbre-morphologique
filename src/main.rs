mod arbre;
mod hashing;
mod morpho_analyzer;

use arbre::Tree;
use hashing::Scheme;
use hashing::SchemeTable;
use hashing::init_schemes;
use morpho_analyzer::afficher_derives_stockes;
use morpho_analyzer::generer_et_stocker;
use morpho_analyzer::valider_et_stocker;
use std::io;

// Corriger l'affichage de l'arabe dans les terminaux qui n'ont pas de support RTL
// Inverse l'ordre des caractères pour compenser l'affichage incorrect
fn afficher_arabe(texte: &str) -> String {
    texte.chars().rev().collect()
}

// Lire une ligne de texte depuis le clavier
fn lire_ligne() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Lire une racine de 3 caractères arabes depuis le clavier
// L'utilisateur tape : ك ت ب (séparés par des espaces)
fn lire_racine() -> Option<[char; 3]> {
    println!("Entrez la racine (3 lettres séparées par des espaces, ex: ك ت ب) :");
    let ligne = lire_ligne();
    let chars: Vec<char> = ligne.chars().filter(|c| !c.is_whitespace()).collect();

    if chars.len() == 3 {
        Some([chars[0], chars[1], chars[2]])
    } else {
        println!("Erreur: il faut exactement 3 caractères.");
        None
    }
}

// Valider qu'un schème contient au moins une lettre morphologique (ف, ع, ل)
// Un schème valide doit pouvoir générer des dérivés en remplaçant ces lettres
fn valider_scheme(nom: &str) -> bool {
    nom.contains('ف') || nom.contains('ع') || nom.contains('ل')
}

// Afficher le menu principal
fn afficher_menu() {
    println!();
    println!("╔══════════════════════════════════════════╗");
    println!("║   Moteur Morphologique Arabe             ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║  1. Charger racines depuis fichier       ║");
    println!("║  2. Ajouter une racine                   ║");
    println!("║  3. Chercher une racine                  ║");
    println!("║  4. Générer les dérivés d'une racine     ║");
    println!("║  5. Valider un mot                       ║");
    println!("║  6. Afficher les dérivés d'une racine    ║");
    println!("║  7. Afficher l'arbre des racines         ║");
    println!("║  8. Afficher les schèmes                 ║");
    println!("║  9. Ajouter un schème                    ║");
    println!("║ 10. Modifier un schème                   ║");
    println!("║ 11. Supprimer un schème                  ║");
    println!("║ 12. Quitter                              ║");
    println!("╚══════════════════════════════════════════╝");
    print!("Choix > ");
    // Forcer l'affichage immédiat du "Choix > "
    use std::io::Write;
    io::stdout().flush().unwrap();
}

fn main() {
    // Créer l'arbre (vide au départ)
    let mut arbre = Tree::new();

    // Créer la table de hachage avec les schèmes pré-chargés
    let mut table_schemes: SchemeTable = init_schemes();

    println!("Bienvenue dans le Moteur Morphologique Arabe !");

    // Boucle principale du menu
    loop {
        afficher_menu();
        let choix = lire_ligne();

        match choix.as_str() {
            // === 1. Charger racines depuis fichier ===
            "1" => {
                println!("Entrez le chemin du fichier (ex: racines.txt) :");
                let chemin = lire_ligne();
                arbre.charger_depuis_fichier(&chemin);
            }

            // === 2. Ajouter une racine manuellement ===
            "2" => {
                if let Some(racine) = lire_racine() {
                    arbre.insert(racine);
                    let r: String = racine.iter().collect();
                    println!("Racine '{}' ajoutée.", afficher_arabe(&r));
                }
            }

            // === 3. Chercher une racine dans l'arbre ===
            "3" => {
                if let Some(racine) = lire_racine() {
                    let r: String = racine.iter().collect();
                    if arbre.verify(racine) {
                        println!("✓ La racine '{}' existe dans l'arbre.", afficher_arabe(&r));
                    } else {
                        println!(
                            "✗ La racine '{}' n'existe pas dans l'arbre.",
                            afficher_arabe(&r)
                        );
                    }
                }
            }

            // === 4. Générer les dérivés d'une racine ===
            "4" => {
                if let Some(racine) = lire_racine() {
                    // Vérifier que la racine existe
                    if !arbre.verify(racine) {
                        let r: String = racine.iter().collect();
                        println!(
                            "Racine '{}' non trouvée. Ajout automatique...",
                            afficher_arabe(&r)
                        );
                        arbre.insert(racine);
                    }
                    generer_et_stocker(&mut arbre, racine, &table_schemes);
                    // Afficher les dérivés stockés
                    afficher_derives_stockes(&mut arbre, racine);
                }
            }

            // === 5. Valider un mot ===
            "5" => {
                println!("Entrez le mot à valider :");
                let mot = lire_ligne();

                if let Some(racine) = lire_racine() {
                    // Vérifier la racine dans l'arbre d'abord
                    if !arbre.verify(racine) {
                        let r: String = racine.iter().collect();
                        println!(
                            "Racine '{}' non trouvée. Ajout automatique...",
                            afficher_arabe(&r)
                        );
                        arbre.insert(racine);
                    }
                    let (trouve, scheme) =
                        valider_et_stocker(&mut arbre, &mot, racine, &table_schemes);
                    let r: String = racine.iter().collect();
                    if trouve {
                        println!(
                            "✓ OUI : '{}' appartient à la racine '{}'",
                            afficher_arabe(&mot),
                            afficher_arabe(&r)
                        );
                        println!("  Schème : {}", afficher_arabe(&scheme.unwrap()));
                    } else {
                        println!(
                            "✗ NON : '{}' n'appartient pas à la racine '{}'",
                            afficher_arabe(&mot),
                            afficher_arabe(&r)
                        );
                    }
                }
            }

            // === 6. Afficher les dérivés d'une racine ===
            "6" => {
                if let Some(racine) = lire_racine() {
                    afficher_derives_stockes(&mut arbre, racine);
                }
            }

            // === 7. Afficher l'arbre complet ===
            "7" => {
                arbre.afficher();
            }

            // === 8. Afficher les schèmes ===
            "8" => {
                table_schemes.display();
            }

            // === 9. Ajouter un schème ===
            "9" => {
                println!("Entrez le nom du schème (ex: فاعل) :");
                let nom = lire_ligne();
                if nom.is_empty() {
                    println!("Erreur: le nom ne peut pas être vide.");
                } else if table_schemes.contains(&nom) {
                    println!(
                        "Le schème '{}' existe déjà. Utilisez l'option 10 pour le modifier.",
                        nom
                    );
                } else if !valider_scheme(&nom) {
                    println!("✗ Erreur: le schème doit contenir au moins un ف, ع, ou ل");
                    println!("   (nécessaire pour générer des dérivés morphologiques)");
                } else {
                    println!("Entrez le pattern (ex: ف-ا-ع-ل) :");
                    let pattern = lire_ligne();
                    println!("Entrez la description (ex: participe actif) :");
                    let description = lire_ligne();
                    table_schemes.insert(
                        nom.clone(),
                        Scheme {
                            nom: nom.clone(),
                            pattern,
                            description,
                        },
                    );
                    println!("✓ Schème '{}' ajouté.", afficher_arabe(&nom));
                }
            }

            // === 10. Modifier un schème ===
            "10" => {
                println!("Entrez le nom du schème à modifier :");
                let nom = lire_ligne();
                if !table_schemes.contains(&nom) {
                    println!("✗ Schème '{}' non trouvé.", afficher_arabe(&nom));
                } else {
                    println!("Nouveau nom du schème (ou Entrée pour garder '{}') :", nom);
                    let nouveau_nom = lire_ligne();
                    let nom_final = if nouveau_nom.is_empty() {
                        nom.clone()
                    } else {
                        nouveau_nom
                    };

                    if !valider_scheme(&nom_final) {
                        println!("✗ Erreur: le schème doit contenir au moins un ف, ع, ou ل");
                    } else {
                        println!("Nouveau pattern (ex: ف-ا-ع-ل) :");
                        let pattern = lire_ligne();
                        println!("Nouvelle description :");
                        let description = lire_ligne();

                        // Si le nom change, supprimer l'ancien
                        if nom_final != nom {
                            table_schemes.delete(&nom);
                        }

                        table_schemes.insert(
                            nom_final.clone(),
                            Scheme {
                                nom: nom_final.clone(),
                                pattern,
                                description,
                            },
                        );
                        println!("✓ Schème '{}' modifié.", afficher_arabe(&nom_final));
                    }
                }
            }

            // === 11. Supprimer un schème ===
            "11" => {
                println!("Entrez le nom du schème à supprimer :");
                let nom = lire_ligne();
                if table_schemes.delete(&nom) {
                    println!("✓ Schème '{}' supprimé.", afficher_arabe(&nom));
                } else {
                    println!("✗ Schème '{}' non trouvé.", afficher_arabe(&nom));
                }
            }

            // === 12. Quitter ===
            "12" => {
                println!("Au revoir !");
                break;
            }

            // Choix invalide
            _ => {
                println!("Choix invalide. Tapez un nombre entre 1 et 12.");
            }
        }
    }
}
