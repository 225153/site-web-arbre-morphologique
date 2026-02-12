// ============================================================================
// API WebAssembly pour Angular
// Ce fichier expose les fonctions Rust au JavaScript/TypeScript
// ============================================================================

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod arbre;
mod hashing;
mod morpho_analyzer;

use arbre::Tree;
use hashing::{SchemeTable, init_schemes};
use morpho_analyzer::{generer_mot, valider_mot};

// État global de l'application (partagé entre les appels JS)
static mut ARBRE: Option<Tree> = None;
static mut SCHEMES: Option<SchemeTable> = None;

// Initialiser l'application (à appeler au démarrage Angular)
#[wasm_bindgen]
pub fn init_app() {
    unsafe {
        ARBRE = Some(Tree::new());
        SCHEMES = Some(init_schemes());
    }
}

// ============================================================================
// GESTION DES RACINES
// ============================================================================

#[wasm_bindgen]
pub fn ajouter_racine(c1: char, c2: char, c3: char) -> String {
    let racine = [c1, c2, c3];
    unsafe {
        if let Some(ref mut arbre) = ARBRE {
            arbre.insert(racine);
            let r: String = racine.iter().collect();
            format!("✓ Racine '{}' ajoutée", r)
        } else {
            "❌ Erreur: Application non initialisée".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn chercher_racine(c1: char, c2: char, c3: char) -> bool {
    let racine = [c1, c2, c3];
    unsafe {
        if let Some(ref arbre) = ARBRE {
            arbre.verify(racine)
        } else {
            false
        }
    }
}

#[wasm_bindgen]
pub fn supprimer_racine(c1: char, c2: char, c3: char) -> bool {
    let racine = [c1, c2, c3];
    unsafe {
        if let Some(ref mut arbre) = ARBRE {
            arbre.delete(racine)
        } else {
            false
        }
    }
}

#[wasm_bindgen]
pub fn charger_racines_depuis_texte(contenu: &str) -> u32 {
    unsafe {
        if let Some(ref mut arbre) = ARBRE {
            let mut compteur = 0;
            for ligne in contenu.lines() {
                let chars: Vec<char> = ligne.chars().filter(|c| !c.is_whitespace()).collect();
                if chars.len() == 3 {
                    let racine = [chars[0], chars[1], chars[2]];
                    arbre.insert(racine);
                    compteur += 1;
                }
            }
            compteur
        } else {
            0
        }
    }
}

// ============================================================================
// GÉNÉRATION DE DÉRIVÉS
// ============================================================================

#[derive(Serialize, Deserialize)]
pub struct Derive {
    pub mot: String,
    pub schema: String,
}

#[wasm_bindgen]
pub fn generer_derive(c1: char, c2: char, c3: char, schema: &str) -> String {
    let racine = [c1, c2, c3];
    generer_mot(racine, schema)
}

#[wasm_bindgen]
pub fn generer_tous_derives(c1: char, c2: char, c3: char) -> JsValue {
    let racine = [c1, c2, c3];
    let mut resultats = Vec::new();

    unsafe {
        if let Some(ref schemes) = SCHEMES {
            for scheme in schemes.get_all_schemes() {
                let mot = generer_mot(racine, &scheme.nom);
                resultats.push(Derive {
                    mot,
                    schema: scheme.nom.clone(),
                });
            }
        }
    }

    serde_wasm_bindgen::to_value(&resultats).unwrap()
}

#[wasm_bindgen]
pub fn ajouter_derive_a_racine(c1: char, c2: char, c3: char, mot: &str, schema: &str) -> bool {
    let racine = [c1, c2, c3];
    unsafe {
        if let Some(ref mut arbre) = ARBRE {
            // Vérifier que la racine existe, sinon refuser
            if !arbre.verify(racine) {
                return false;
            }
            arbre.ajouter_derive(racine, mot.to_string(), schema.to_string())
        } else {
            false
        }
    }
}

// Générer un dérivé ET le stocker automatiquement (comme le terminal)
#[wasm_bindgen]
pub fn generer_et_stocker_derive(c1: char, c2: char, c3: char, schema: &str) -> bool {
    let racine = [c1, c2, c3];

    unsafe {
        if let Some(ref mut arbre) = ARBRE {
            // Vérifier que la racine existe, sinon refuser
            if !arbre.verify(racine) {
                return false;
            }

            // Générer le mot
            let mot = generer_mot(racine, schema);

            // Stocker dans l'arbre
            return arbre.ajouter_derive(racine, mot, schema.to_string());
        }
    }
    false
}

// Générer TOUS les dérivés ET les stocker automatiquement (comme le terminal)
#[wasm_bindgen]
pub fn generer_et_stocker_tous_derives(c1: char, c2: char, c3: char) -> u32 {
    let racine = [c1, c2, c3];
    let mut compteur = 0;

    unsafe {
        if let Some(ref mut arbre) = ARBRE {
            // Vérifier que la racine existe, sinon refuser
            if !arbre.verify(racine) {
                return 0;
            }

            if let Some(ref schemes) = SCHEMES {
                for scheme in schemes.get_all_schemes() {
                    let mot = generer_mot(racine, &scheme.nom);
                    if arbre.ajouter_derive(racine, mot, scheme.nom.clone()) {
                        compteur += 1;
                    }
                }
            }
        }
    }

    compteur
}

// ============================================================================
// VALIDATION DE MOTS
// ============================================================================

#[derive(Serialize, Deserialize)]
pub struct ValidationResult {
    pub valide: bool,
    pub schema: Option<String>,
}

#[wasm_bindgen]
pub fn valider_mot_derive(mot: &str, c1: char, c2: char, c3: char) -> JsValue {
    let racine = [c1, c2, c3];

    unsafe {
        if let Some(ref schemes) = SCHEMES {
            let (valide, schema_opt) = valider_mot(mot, racine, schemes);
            let result = ValidationResult {
                valide,
                schema: schema_opt,
            };
            serde_wasm_bindgen::to_value(&result).unwrap()
        } else {
            serde_wasm_bindgen::to_value(&ValidationResult {
                valide: false,
                schema: None,
            })
            .unwrap()
        }
    }
}

// ============================================================================
// AFFICHAGE DES DÉRIVÉS STOCKÉS
// ============================================================================

#[wasm_bindgen]
pub fn obtenir_derives_stockes(c1: char, c2: char, c3: char) -> JsValue {
    let racine = [c1, c2, c3];

    unsafe {
        if let Some(ref mut arbre) = ARBRE {
            if let Some(noeud) = arbre.chercher_noeud(racine) {
                let derives: Vec<Derive> = noeud
                    .derives
                    .iter()
                    .map(|d| Derive {
                        mot: d.mot.clone(),
                        schema: d.schema.clone(),
                    })
                    .collect();
                return serde_wasm_bindgen::to_value(&derives).unwrap();
            }
        }
    }

    serde_wasm_bindgen::to_value(&Vec::<Derive>::new()).unwrap()
}

#[wasm_bindgen]
pub fn supprimer_derive(c1: char, c2: char, c3: char, mot: &str) -> bool {
    let racine = [c1, c2, c3];

    unsafe {
        if let Some(ref mut arbre) = ARBRE {
            if let Some(noeud) = arbre.chercher_noeud(racine) {
                return noeud.supprimer_derive(mot);
            }
        }
    }
    false
}

// ============================================================================
// GESTION DES SCHÈMES
// ============================================================================

#[derive(Serialize, Deserialize)]
pub struct SchemeInfo {
    pub nom: String,
    pub description: String,
}

#[wasm_bindgen]
pub fn obtenir_tous_schemes() -> JsValue {
    unsafe {
        if let Some(ref schemes) = SCHEMES {
            let infos: Vec<SchemeInfo> = schemes
                .get_all_schemes()
                .iter()
                .map(|s| SchemeInfo {
                    nom: s.nom.clone(),
                    description: s.description.clone(),
                })
                .collect();
            return serde_wasm_bindgen::to_value(&infos).unwrap();
        }
    }
    serde_wasm_bindgen::to_value(&Vec::<SchemeInfo>::new()).unwrap()
}

#[wasm_bindgen]
pub fn ajouter_scheme(nom: &str, pattern: &str, description: &str) -> bool {
    unsafe {
        if let Some(ref mut schemes) = SCHEMES {
            use hashing::Scheme;
            schemes.insert(
                nom.to_string(),
                Scheme {
                    nom: nom.to_string(),
                    pattern: pattern.to_string(),
                    description: description.to_string(),
                },
            );
            return true;
        }
    }
    false
}

#[wasm_bindgen]
pub fn supprimer_scheme(nom: &str) -> bool {
    unsafe {
        if let Some(ref mut schemes) = SCHEMES {
            return schemes.delete(nom);
        }
    }
    false
}

// ============================================================================
// AFFICHAGE DE L'ARBRE
// ============================================================================

#[derive(Serialize, Deserialize)]
pub struct DeriveInfo {
    pub mot: String,
    pub schema: String,
}

#[derive(Serialize, Deserialize)]
pub struct RacineInfo {
    pub racine: String,
    pub nombre_derives: u32,
    pub derives: Vec<DeriveInfo>,
}

#[wasm_bindgen]
pub fn obtenir_toutes_racines() -> JsValue {
    let mut racines = Vec::new();

    unsafe {
        if let Some(ref arbre) = ARBRE {
            arbre.collecter_racines(&mut racines);
        }
    }

    let infos: Vec<RacineInfo> = racines
        .iter()
        .map(|(racine, freq, derives)| {
            let r: String = racine.iter().collect();
            RacineInfo {
                racine: r,
                nombre_derives: *freq,
                derives: derives
                    .iter()
                    .map(|d| DeriveInfo {
                        mot: d.mot.clone(),
                        schema: d.schema.clone(),
                    })
                    .collect(),
            }
        })
        .collect();

    serde_wasm_bindgen::to_value(&infos).unwrap()
}

// ============================================================================
// SAUVEGARDE ET RESTAURATION (pour éviter la perte lors du rafraîchissement)
// ============================================================================

// Structure pour exporter l'état complet de l'arbre
#[derive(Serialize, Deserialize)]
pub struct ExportData {
    pub racines: Vec<([char; 3], Vec<(String, String)>)>, // (racine, [(mot, schema)])
}

// Exporter toutes les données en JSON (pour localStorage)
#[wasm_bindgen]
pub fn exporter_donnees() -> String {
    let mut data = ExportData {
        racines: Vec::new(),
    };

    unsafe {
        if let Some(ref mut arbre) = ARBRE {
            let mut racines_list = Vec::new();
            arbre.collecter_racines(&mut racines_list);

            for (racine, _freq, derives_vec) in racines_list {
                let derives: Vec<(String, String)> = derives_vec
                    .iter()
                    .map(|d| (d.mot.clone(), d.schema.clone()))
                    .collect();

                if !derives.is_empty() {
                    data.racines.push((racine, derives));
                }
            }
        }
    }

    serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string())
}

// Importer les données depuis JSON (depuis localStorage)
#[wasm_bindgen]
pub fn importer_donnees(json: &str) -> bool {
    let data: ExportData = match serde_json::from_str(json) {
        Ok(d) => d,
        Err(_) => return false,
    };

    unsafe {
        if let Some(ref mut arbre) = ARBRE {
            // Réinitialiser l'arbre
            *arbre = Tree::new();

            // Restaurer toutes les racines et leurs dérivés
            for (racine, derives) in data.racines {
                arbre.insert(racine);

                for (mot, schema) in derives {
                    arbre.ajouter_derive(racine, mot, schema);
                }
            }

            return true;
        }
    }

    false
}
