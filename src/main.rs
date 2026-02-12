// ============================================================================
// IMPORTANT : MODULE TERMINAL_ADAPTER
// ============================================================================
// Le module terminal_adapter contient TOUTES les fonctions spécifiques au
// terminal Windows. Lors de la migration vers web/mobile :
// 1. Supprimer : mod terminal_adapter;
// 2. Supprimer : les imports terminal_adapter::*
// 3. Remplacer lire_racine_terminal() par réception données API
// 4. Remplacer afficher_arabe() par affichage HTML direct avec dir="rtl"
// ============================================================================

mod arbre;
mod hashing;
mod morpho_analyzer;
mod terminal_adapter; // ← À SUPPRIMER pour le web

use arbre::Tree;
use hashing::Scheme;
use hashing::SchemeTable;
use hashing::init_schemes;
use morpho_analyzer::afficher_derives_stockes;
use morpho_analyzer::afficher_famille;
use morpho_analyzer::generer_et_stocker;
use morpho_analyzer::generer_mot;
use morpho_analyzer::valider_et_stocker;

// ← À SUPPRIMER pour le web
use terminal_adapter::{afficher_arabe, lire_ligne_simple, lire_racine_terminal, lire_texte_arabe};

use std::io; // Pour io::stdout()

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
    println!("║  4. Supprimer une racine                 ║");
    println!("║  5. Prévisualiser famille morphologique  ║");
    println!("║  6. Générer un dérivé avec un schème     ║");
    println!("║  7. Générer tous les dérivés (stockage)  ║");
    println!("║  8. Valider un mot                       ║");
    println!("║  9. Afficher les dérivés stockés         ║");
    println!("║ 10. Supprimer un dérivé d'une racine     ║");
    println!("║ 11. Afficher l'arbre des racines         ║");
    println!("║ 12. Afficher les schèmes                 ║");
    println!("║ 13. Ajouter un schème                    ║");
    println!("║ 14. Modifier un schème                   ║");
    println!("║ 15. Supprimer un schème                  ║");
    println!("║ 16. Quitter                              ║");
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
        let choix = lire_ligne_simple();

        match choix.as_str() {
            // === 1. Charger racines depuis fichier ===
            "1" => {
                println!("Entrez le chemin du fichier (ex: racines.txt) :");
                let chemin = lire_ligne_simple();
                arbre.charger_depuis_fichier(&chemin);
            }

            // === 2. Ajouter une racine manuellement ===
            "2" => {
                if let Some(racine) = lire_racine_terminal() {
                    arbre.insert(racine);
                    let r: String = racine.iter().collect();
                    println!("Racine '{}' ajoutée.", afficher_arabe(&r));
                }
            }

            // === 3. Chercher une racine dans l'arbre ===
            "3" => {
                if let Some(racine) = lire_racine_terminal() {
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

            // === 4. Supprimer une racine ===
            "4" => {
                if let Some(racine) = lire_racine_terminal() {
                    let r: String = racine.iter().collect();
                    if arbre.delete(racine) {
                        println!("✓ Racine '{}' supprimée avec succès.", afficher_arabe(&r));
                    } else {
                        println!(
                            "✗ La racine '{}' n'existe pas dans l'arbre.",
                            afficher_arabe(&r)
                        );
                    }
                }
            }

            // === 5. Prévisualiser famille morphologique ===
            "5" => {
                if let Some(racine) = lire_racine_terminal() {
                    if !arbre.verify(racine) {
                        let r: String = racine.iter().collect();
                        println!(
                            "✗ La racine '{}' n'existe pas dans l'arbre. Ajoutez-la d'abord.",
                            afficher_arabe(&r)
                        );
                    } else {
                        afficher_famille(racine, &table_schemes);
                    }
                }
            }

            // === 6. Générer un dérivé avec un schème spécifique ===
            "6" => {
                if let Some(racine) = lire_racine_terminal() {
                    // Vérifier que la racine existe
                    if !arbre.verify(racine) {
                        let r: String = racine.iter().collect();
                        println!(
                            "✗ La racine '{}' n'existe pas dans l'arbre. Ajoutez-la d'abord.",
                            afficher_arabe(&r)
                        );
                    } else {
                        println!("Entrez le nom du schème (ex: فاعل) :");
                        let nom_scheme = lire_texte_arabe();

                        // Vérifier que le schème existe
                        if !table_schemes.contains(&nom_scheme) {
                            println!("✗ Schème '{}' non trouvé.", afficher_arabe(&nom_scheme));
                        } else {
                            // Générer le mot
                            let mot = generer_mot(racine, &nom_scheme);

                            // Stocker dans l'arbre
                            let ok = arbre.ajouter_derive(racine, mot.clone(), nom_scheme.clone());

                            if ok {
                                println!(
                                    "✓ Dérivé '{}' généré et stocké (schème: {})",
                                    afficher_arabe(&mot),
                                    afficher_arabe(&nom_scheme)
                                );
                            } else {
                                println!("✗ Erreur lors du stockage du dérivé.");
                            }
                        }
                    }
                }
            }

            // === 7. Générer tous les dérivés d'une racine ===
            "7" => {
                if let Some(racine) = lire_racine_terminal() {
                    // Vérifier que la racine existe
                    if !arbre.verify(racine) {
                        let r: String = racine.iter().collect();
                        println!(
                            "✗ La racine '{}' n'existe pas dans l'arbre. Ajoutez-la d'abord.",
                            afficher_arabe(&r)
                        );
                    } else {
                        generer_et_stocker(&mut arbre, racine, &table_schemes);
                        // Afficher les dérivés stockés
                        afficher_derives_stockes(&mut arbre, racine);
                    }
                }
            }

            // === 8. Valider un mot ===
            "8" => {
                println!("Entrez le mot à valider (ex: كاتب) :");
                let mot = lire_texte_arabe();

                if let Some(racine) = lire_racine_terminal() {
                    // Vérifier la racine dans l'arbre d'abord
                    if !arbre.verify(racine) {
                        let r: String = racine.iter().collect();
                        println!(
                            "✗ La racine '{}' n'existe pas dans l'arbre. Ajoutez-la d'abord.",
                            afficher_arabe(&r)
                        );
                    } else {
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
            }

            // === 9. Afficher les dérivés stockés d'une racine ===
            "9" => {
                if let Some(racine) = lire_racine_terminal() {
                    afficher_derives_stockes(&mut arbre, racine);
                }
            }

            // === 10. Supprimer un dérivé d'une racine ===
            "10" => {
                if let Some(racine) = lire_racine_terminal() {
                    // Vérifier que la racine existe
                    if !arbre.verify(racine) {
                        let r: String = racine.iter().collect();
                        println!(
                            "✗ La racine '{}' n'existe pas dans l'arbre.",
                            afficher_arabe(&r)
                        );
                    } else {
                        // Afficher d'abord les dérivés existants
                        afficher_derives_stockes(&mut arbre, racine);

                        println!("\nEntrez le mot dérivé à supprimer (ex: كاتب) :");
                        let mot = lire_texte_arabe();

                        // Chercher le nœud et supprimer le dérivé
                        if let Some(noeud) = arbre.chercher_noeud(racine) {
                            if noeud.supprimer_derive(&mot) {
                                println!(
                                    "✓ Dérivé '{}' supprimé avec succès.",
                                    afficher_arabe(&mot)
                                );
                            } else {
                                println!(
                                    "✗ Dérivé '{}' non trouvé dans cette racine.",
                                    afficher_arabe(&mot)
                                );
                            }
                        }
                    }
                }
            }

            // === 11. Afficher l'arbre complet ===
            "11" => {
                arbre.afficher();
            }

            // === 12. Afficher les schèmes ===
            "12" => {
                table_schemes.display();
            }

            // === 13. Ajouter un schème ===
            "13" => {
                println!("Entrez le nom du schème (ex: فاعل) :");
                let nom = lire_texte_arabe();
                if nom.is_empty() {
                    println!("Erreur: le nom ne peut pas être vide.");
                } else if table_schemes.contains(&nom) {
                    println!(
                        "Le schème '{}' existe déjà. Utilisez l'option 10 pour le modifier.",
                        afficher_arabe(&nom)
                    );
                } else if !valider_scheme(&nom) {
                    println!("✗ Erreur: le schème doit contenir au moins un ف, ع, ou ل");
                    println!("   (nécessaire pour générer des dérivés morphologiques)");
                } else {
                    println!("Entrez le pattern (ex: ف-ا-ع-ل) :");
                    let pattern = lire_texte_arabe();
                    println!("Entrez la description :");
                    let description = lire_ligne_simple();
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

            // === 14. Modifier un schème ===
            "14" => {
                println!("Entrez le nom du schème à modifier (ex: فاعل) :");
                let nom = lire_texte_arabe();
                if !table_schemes.contains(&nom) {
                    println!("✗ Schème '{}' non trouvé.", afficher_arabe(&nom));
                } else {
                    println!(
                        "Nouveau nom du schème (ou Entrée pour garder '{}') :",
                        afficher_arabe(&nom)
                    );
                    let nouveau_nom_input = lire_ligne_simple();
                    let nom_final = if nouveau_nom_input.is_empty() {
                        nom.clone()
                    } else {
                        nouveau_nom_input
                            .chars()
                            .filter(|c| !c.is_whitespace())
                            .collect()
                    };

                    if !valider_scheme(&nom_final) {
                        println!("✗ Erreur: le schème doit contenir au moins un ف, ع, ou ل");
                    } else {
                        println!("Nouveau pattern (ex: ف-ا-ع-ل) :");
                        let pattern = lire_texte_arabe();
                        println!("Nouvelle description :");
                        let description = lire_ligne_simple();

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

            // === 15. Supprimer un schème ===
            "15" => {
                println!("Entrez le nom du schème à supprimer (ex: فاعل) :");
                let nom = lire_texte_arabe();
                if table_schemes.delete(&nom) {
                    println!("✓ Schème '{}' supprimé.", afficher_arabe(&nom));
                } else {
                    println!("✗ Schème '{}' non trouvé.", afficher_arabe(&nom));
                }
            }

            // === 16. Quitter ===
            "16" => {
                println!("Au revoir !");
                break;
            }

            // Choix invalide
            _ => {
                println!("Choix invalide. Tapez un nombre entre 1 et 16.");
            }
        }
    }
}
