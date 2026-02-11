pub struct Derive {
    pub mot: String,
    pub schema: String,
}

// Pour lire un fichier
use std::fs;

pub struct RacineNode {
    pub racine: [char; 3],
    pub derives: Vec<Derive>, // liste des mots dérivés validés
    pub frequence: u32,       // nombre de dérivés stockés
    pub left: Option<Box<RacineNode>>,
    pub right: Option<Box<RacineNode>>,
}

pub fn morphologic_cmp(tree_racine: [char; 3], racine: [char; 3]) -> i8 {
    let mut i = 0;
    while i < 3 {
        if tree_racine[i] == racine[i] {
            i = i + 1;
        } else {
            if tree_racine[i] > racine[i] {
                return -1;
            } else {
                return 1;
            }
        }
    }
    return 0;
}

pub struct Tree {
    pub racine: Option<Box<RacineNode>>,
}

impl RacineNode {
    pub fn new(racine: [char; 3]) -> Self {
        RacineNode {
            racine,
            derives: Vec::new(), // liste vide au début
            frequence: 0,        // aucun dérivé au début
            left: None,
            right: None,
        }
    }

    // Ajouter un dérivé validé à ce nœud
    pub fn ajouter_derive(&mut self, mot: String, schema: String) {
        // Vérifier si ce dérivé existe déjà (éviter les doublons)
        for d in &self.derives {
            if d.mot == mot {
                return; // déjà présent, on ne fait rien
            }
        }
        // Ajouter le nouveau dérivé
        self.derives.push(Derive {
            mot: mot,
            schema: schema,
        });
        self.frequence = self.frequence + 1;
    }

    // Afficher tous les dérivés de cette racine
    pub fn afficher_derives(&self) {
        let r: String = self.racine.iter().collect();
        let r_display: String = r.chars().rev().collect();
        println!("Racine: {} ({} dérivés)", r_display, self.frequence);
        for d in &self.derives {
            let mot_display: String = d.mot.chars().rev().collect();
            let schema_display: String = d.schema.chars().rev().collect();
            println!("  - {} (schème: {})", mot_display, schema_display);
        }
    }

    // Parcours in-order : gauche → nœud courant → droite
    // Affiche les racines triées dans l'ordre alphabétique arabe
    pub fn afficher_in_order(&self) {
        // 1) D'abord, afficher tout le sous-arbre gauche
        if let Some(gauche) = &self.left {
            gauche.afficher_in_order();
        }

        // 2) Ensuite, afficher le nœud courant
        let r: String = self.racine.iter().collect();
        let r_display: String = r.chars().rev().collect();
        if self.frequence > 0 {
            println!("  {} ({} dérivés)", r_display, self.frequence);
        } else {
            println!("  {}", r_display);
        }

        // 3) Enfin, afficher tout le sous-arbre droit
        if let Some(droite) = &self.right {
            droite.afficher_in_order();
        }
    }

    pub fn verify_node(&self, ch: [char; 3]) -> bool {
        let cmp = morphologic_cmp(self.racine, ch);
        if cmp == 0 {
            return true;
        } else {
            if cmp == -1 {
                if self.left.is_none() {
                    return false;
                } else {
                    return self.left.as_ref().unwrap().verify_node(ch);
                }
            } else {
                if self.right.is_none() {
                    return false;
                } else {
                    return self.right.as_ref().unwrap().verify_node(ch);
                }
            }
        }
    }
    pub fn insert_node(&mut self, ch: [char; 3]) {
        let cmp = morphologic_cmp(self.racine, ch);
        if cmp == 1 {
            if self.right.is_none() {
                self.right = Some(Box::new(RacineNode::new(ch)));
            } else {
                self.right.as_mut().unwrap().insert_node(ch);
            }
        } else {
            if cmp == 0 {
                return;
            } else {
                if self.left.is_none() {
                    self.left = Some(Box::new(RacineNode::new(ch)));
                } else {
                    self.left.as_mut().unwrap().insert_node(ch);
                }
            }
        }
    }
}
impl Tree {
    pub fn new() -> Self {
        Tree { racine: None }
    }
    pub fn verify(&self, ch: [char; 3]) -> bool {
        if self.racine.is_none() {
            return false;
        }
        let node = self.racine.as_ref().unwrap();
        return node.verify_node(ch);
    }
    pub fn insert(&mut self, ch: [char; 3]) {
        if self.racine.is_none() {
            self.racine = Some(Box::new(RacineNode::new(ch)));
            return;
        }
        self.racine.as_mut().unwrap().insert_node(ch);
    }

    // Chercher un noeud par sa racine et retourner une référence mutable
    // On en a besoin pour pouvoir ajouter des dérivés à un noeud
    pub fn chercher_noeud(&mut self, ch: [char; 3]) -> Option<&mut RacineNode> {
        // Commencer à la racine de l'arbre
        let mut courant = self.racine.as_mut();

        while let Some(noeud) = courant {
            let cmp = morphologic_cmp(noeud.racine, ch);
            if cmp == 0 {
                return Some(noeud); // trouvé !
            } else if cmp == -1 {
                courant = noeud.left.as_mut(); // aller à gauche
            } else {
                courant = noeud.right.as_mut(); // aller à droite
            }
        }
        None // pas trouvé
    }

    // Ajouter un dérivé à une racine donnée (cherche le noeud puis ajoute)
    pub fn ajouter_derive(&mut self, ch: [char; 3], mot: String, schema: String) -> bool {
        // D'abord on cherche le noeud de cette racine
        let noeud = self.chercher_noeud(ch);
        match noeud {
            Some(n) => {
                n.ajouter_derive(mot, schema);
                true // succès
            }
            None => false, // racine non trouvée dans l'arbre
        }
    }

    // Charger des racines depuis un fichier texte
    // Le fichier contient une racine par ligne, format : "ك ت ب"
    // Retourne le nombre de racines chargées
    pub fn charger_depuis_fichier(&mut self, chemin: &str) -> u32 {
        // Étape 1 : Lire tout le contenu du fichier
        let contenu = fs::read_to_string(chemin);

        // Étape 2 : Vérifier si la lecture a réussi
        let texte = match contenu {
            Ok(t) => t, // lecture OK, on récupère le texte
            Err(e) => {
                println!("Erreur lecture fichier '{}': {}", chemin, e);
                return 0; // on retourne 0 racines chargées
            }
        };

        let mut compteur: u32 = 0;

        // Étape 3 : Parcourir le fichier ligne par ligne
        for ligne in texte.lines() {
            // Ignorer les lignes vides
            let ligne = ligne.trim();
            if ligne.is_empty() {
                continue;
            }

            // Étape 4 : Extraire les 3 caractères arabes de la ligne
            // Format attendu : "ك ت ب" (3 caractères séparés par des espaces)
            let chars: Vec<char> = ligne
                .chars() // itérer sur chaque caractère
                .filter(|c| !c.is_whitespace()) // enlever les espaces
                .collect(); // collecter dans un vecteur

            // Vérifier qu'on a bien 3 caractères
            if chars.len() == 3 {
                let racine: [char; 3] = [chars[0], chars[1], chars[2]];
                self.insert(racine); // insérer dans l'arbre (les doublons sont ignorés)
                compteur = compteur + 1;
            } else {
                let ligne_display: String = ligne.chars().rev().collect();
                println!("Ligne ignorée (pas 3 caractères): '{}'", ligne_display);
            }
        }

        println!("{} racines chargées depuis '{}'", compteur, chemin);
        compteur
    }

    // Afficher toutes les racines de l'arbre (parcours in-order)
    pub fn afficher(&self) {
        if self.racine.is_none() {
            println!("L'arbre est vide.");
            return;
        }
        println!("=== Racines stockées (ordre trié) ===");
        self.racine.as_ref().unwrap().afficher_in_order();
    }
}
