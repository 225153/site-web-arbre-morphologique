import { Injectable } from '@angular/core';
import init, {
  init_app,
  ajouter_racine,
  chercher_racine,
  supprimer_racine,
  charger_racines_depuis_texte,
  obtenir_toutes_racines,
  generer_derive,
  generer_tous_derives,
  generer_et_stocker_derive,
  generer_et_stocker_tous_derives,
  valider_mot_derive,
  ajouter_derive_a_racine,
  obtenir_derives_stockes,
  supprimer_derive,
  obtenir_tous_schemes,
  ajouter_scheme,
  supprimer_scheme,
  exporter_donnees,
  importer_donnees
} from 'moteur_morphologique';

// Types pour les résultats
export interface DeriveInfo {
  mot: string;
  schema: string;
}

export interface RacineInfo {
  racine: string;
  nombre_derives: number;
  derives: DeriveInfo[];
}

export interface SchemeInfo {
  id: string;
  nom: string;
  modele: string;
}

export interface ValidationResult {
  valide: boolean;
  schema: string;
}

@Injectable({
  providedIn: 'root'
})
export class MorphoService {
  private _initialized = false;

  get initialized(): boolean {
    return this._initialized;
  }

  async initialize(): Promise<void> {
    if (this._initialized) return;

    try {
      await init('/wasm/moteur_morphologique_bg.wasm');
      init_app();
      this._initialized = true;
      console.log('Moteur morphologique initialisé avec succès');
    } catch (error) {
      console.error('Erreur d\'initialisation du moteur WASM:', error);
      throw error;
    }
  }

  // === Utilitaire: extraire les 3 caractères ===
  private extraireCaracteres(racine: string): [string, string, string] {
    const chars = [...racine];
    if (chars.length !== 3) {
      throw new Error('La racine doit contenir exactement 3 caractères arabes');
    }
    return [chars[0], chars[1], chars[2]];
  }

  // === RACINES ===
  ajouterRacine(racine: string): string {
    const [c1, c2, c3] = this.extraireCaracteres(racine);
    return ajouter_racine(c1, c2, c3);
  }

  chercherRacine(racine: string): boolean {
    const [c1, c2, c3] = this.extraireCaracteres(racine);
    return chercher_racine(c1, c2, c3);
  }

  supprimerRacine(racine: string): boolean {
    const [c1, c2, c3] = this.extraireCaracteres(racine);
    return supprimer_racine(c1, c2, c3);
  }

  chargerRacinesDepuisTexte(contenu: string): number {
    return charger_racines_depuis_texte(contenu);
  }

  obtenirToutesRacines(): RacineInfo[] {
    return obtenir_toutes_racines() || [];
  }

  // === DERIVATION (génération seule) ===
  genererDerive(racine: string, schema: string): string {
    const [c1, c2, c3] = this.extraireCaracteres(racine);
    return generer_derive(c1, c2, c3, schema);
  }

  genererTousDerives(racine: string): DeriveInfo[] {
    const [c1, c2, c3] = this.extraireCaracteres(racine);
    return generer_tous_derives(c1, c2, c3) || [];
  }

  // === DERIVATION + STOCKAGE (comme terminal) ===
  genererEtStockerDerive(racine: string, schema: string): boolean {
    const [c1, c2, c3] = this.extraireCaracteres(racine);
    return generer_et_stocker_derive(c1, c2, c3, schema);
  }

  genererEtStockerTousDerives(racine: string): number {
    const [c1, c2, c3] = this.extraireCaracteres(racine);
    return generer_et_stocker_tous_derives(c1, c2, c3);
  }

  // === VALIDATION ===
  validerMotDerive(mot: string, racine: string): ValidationResult {
    const [c1, c2, c3] = this.extraireCaracteres(racine);
    return valider_mot_derive(mot, c1, c2, c3) || { valide: false, schema: '' };
  }

  // === DERIVES STOCKES ===
  ajouterDeriveARacine(racine: string, mot: string, schema: string): boolean {
    const [c1, c2, c3] = this.extraireCaracteres(racine);
    return ajouter_derive_a_racine(c1, c2, c3, mot, schema);
  }

  obtenirDerivesStockes(racine: string): DeriveInfo[] {
    const [c1, c2, c3] = this.extraireCaracteres(racine);
    return obtenir_derives_stockes(c1, c2, c3) || [];
  }

  supprimerDerive(racine: string, mot: string): boolean {
    const [c1, c2, c3] = this.extraireCaracteres(racine);
    return supprimer_derive(c1, c2, c3, mot);
  }

  // === SCHEMES ===
  obtenirTousSchemes(): SchemeInfo[] {
    return obtenir_tous_schemes() || [];
  }

  ajouterScheme(nom: string, pattern: string, description: string): boolean {
    return ajouter_scheme(nom, pattern, description);
  }

  supprimerScheme(nom: string): boolean {
    return supprimer_scheme(nom);
  }

  // === SAUVEGARDE / RESTAURATION ===
  exporterDonnees(): string {
    return exporter_donnees();
  }

  importerDonnees(json: string): boolean {
    return importer_donnees(json);
  }

  sauvegarderDansLocalStorage(): void {
    const data = this.exporterDonnees();
    localStorage.setItem('morpho_data', data);
  }

  restaurerDepuisLocalStorage(): boolean {
    const json = localStorage.getItem('morpho_data');
    if (json) {
      return this.importerDonnees(json);
    }
    return false;
  }
}
