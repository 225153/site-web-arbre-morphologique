# Guide d'intégration Angular

## 📦 Package WebAssembly généré

- **Fichier WASM** : `pkg/moteur_morphologique_bg.wasm` (126 KB)
- **Module JavaScript** : `pkg/moteur_morphologique.js`
- **Définitions TypeScript** : `pkg/moteur_morphologique.d.ts`

## 🚀 Installation dans Angular

### 1. Copier le dossier pkg dans votre projet Angular

```bash
cp -r pkg/* src/assets/wasm/
```

### 2. Configuration TypeScript (tsconfig.json)

```json
{
  "compilerOptions": {
    "allowSyntheticDefaultImports": true,
    "esModuleInterop": true
  }
}
```

### 3. Service Angular (morpho.service.ts)

```typescript
import { Injectable } from "@angular/core";

@Injectable({
  providedIn: "root",
})
export class MorphoService {
  private wasm: any;
  private initialized = false;

  async initialize() {
    if (this.initialized) return;

    // Charger le module WASM
    const module = await import("../assets/wasm/moteur_morphologique.js");
    await module.default();
    this.wasm = module;

    // Initialiser l'application
    this.wasm.init_app();
    this.initialized = true;
    console.log("Moteur morphologique initialisé");
  }

  // Opérations sur les racines
  ajouterRacine(racine: string): boolean {
    return this.wasm.ajouter_racine(racine);
  }

  chercherRacine(racine: string): boolean {
    return this.wasm.chercher_racine(racine);
  }

  supprimerRacine(racine: string): boolean {
    return this.wasm.supprimer_racine(racine);
  }

  chargerRacines(texte: string): number {
    return this.wasm.charger_racines_depuis_texte(texte);
  }

  obtenirToutesRacines(): any[] {
    return this.wasm.obtenir_toutes_racines();
  }

  // Génération de dérivés (sans stockage)
  genererDerive(racine: string, schemeId: string): string | null {
    return this.wasm.generer_derive(racine, schemeId);
  }

  genererTousDerives(racine: string): any[] {
    return this.wasm.generer_tous_derives(racine);
  }

  // Génération de dérivés AVEC stockage (comme terminal)
  genererEtStockerDerive(racine: string, schemeId: string): string | null {
    return this.wasm.generer_et_stocker_derive(racine, schemeId);
  }

  genererEtStockerTousDerives(racine: string): any[] {
    return this.wasm.generer_et_stocker_tous_derives(racine);
  }

  // Validation
  validerMotDerive(racine: string, motDerive: string): any {
    return this.wasm.valider_mot_derive(racine, motDerive);
  }

  // Gestion des dérivés stockés
  ajouterDeriveARacine(
    racine: string,
    motDerive: string,
    schemeId: string,
  ): boolean {
    return this.wasm.ajouter_derive_a_racine(racine, motDerive, schemeId);
  }

  obtenirDerivesStockes(racine: string): any[] {
    return this.wasm.obtenir_derives_stockes(racine);
  }

  supprimerDerive(
    racine: string,
    motDerive: string,
    schemeId: string,
  ): boolean {
    return this.wasm.supprimer_derive(racine, motDerive, schemeId);
  }

  // Gestion des schémas
  obtenirTousSchemes(): any[] {
    return this.wasm.obtenir_tous_schemes();
  }

  ajouterScheme(id: string, nom: string, modele: string): boolean {
    return this.wasm.ajouter_scheme(id, nom, modele);
  }

  supprimerScheme(id: string): boolean {
    return this.wasm.supprimer_scheme(id);
  }

  // Persistance (optionnel - localStorage)
  exporterDonnees(): string {
    return this.wasm.exporter_donnees();
  }

  importerDonnees(json: string): boolean {
    return this.wasm.importer_donnees(json);
  }
}
```

### 4. Composant Angular (morpho.component.ts)

```typescript
import { Component, OnInit } from "@angular/core";
import { MorphoService } from "./services/morpho.service";

@Component({
  selector: "app-morpho",
  templateUrl: "./morpho.component.html",
  styleUrls: ["./morpho.component.css"],
})
export class MorphoComponent implements OnInit {
  racine: string = "";
  resultat: string = "";
  racines: any[] = [];
  schemes: any[] = [];

  constructor(private morphoService: MorphoService) {}

  async ngOnInit() {
    await this.morphoService.initialize();
    this.chargerSchemes();
  }

  ajouterRacine() {
    if (this.morphoService.ajouterRacine(this.racine)) {
      this.resultat = `Racine "${this.racine}" ajoutée avec succès`;
      this.afficherRacines();
    } else {
      this.resultat = `Erreur : racine invalide ou déjà existante`;
    }
  }

  chercherRacine() {
    const existe = this.morphoService.chercherRacine(this.racine);
    this.resultat = existe
      ? `✓ Racine "${this.racine}" existe`
      : `✗ Racine "${this.racine}" introuvable`;
  }

  genererEtStockerTous() {
    const derives = this.morphoService.genererEtStockerTousDerives(this.racine);
    this.resultat = `${derives.length} dérivés générés et stockés:\n`;
    derives.forEach((d) => {
      this.resultat += `\n• ${d.mot} (${d.schema})`;
    });
  }

  afficherDerivesStockes() {
    const derives = this.morphoService.obtenirDerivesStockes(this.racine);
    if (derives.length === 0) {
      this.resultat = "Aucun dérivé stocké pour cette racine";
    } else {
      this.resultat = `Dérivés stockés (${derives.length}):\n`;
      derives.forEach((d) => {
        this.resultat += `\n• ${d.mot} (${d.schema})`;
      });
    }
  }

  afficherRacines() {
    this.racines = this.morphoService.obtenirToutesRacines();
    this.resultat = `${this.racines.length} racines dans l'arbre`;
  }

  chargerSchemes() {
    this.schemes = this.morphoService.obtenirTousSchemes();
  }

  validerMot() {
    const validation = this.morphoService.validerMotDerive(
      this.racine,
      this.resultat,
    );
    if (validation.valide) {
      this.resultat = `✓ Mot valide avec schéma: ${validation.schema}`;
    } else {
      this.resultat = `✗ Mot invalide pour cette racine`;
    }
  }

  sauvegarder() {
    const json = this.morphoService.exporterDonnees();
    localStorage.setItem("morpho_data", json);
    this.resultat = "Données sauvegardées dans localStorage";
  }

  restaurer() {
    const json = localStorage.getItem("morpho_data");
    if (json && this.morphoService.importerDonnees(json)) {
      this.resultat = "Données restaurées avec succès";
      this.afficherRacines();
    } else {
      this.resultat = "Aucune donnée à restaurer";
    }
  }
}
```

### 5. Template HTML (morpho.component.html)

```html
<div class="morpho-container" dir="rtl">
  <h1>محلل الصرف العربي</h1>

  <div class="input-section">
    <input
      [(ngModel)]="racine"
      placeholder="أدخل الجذر (3 أحرف)"
      maxlength="3"
      class="arabic-input"
    />

    <div class="button-group">
      <button (click)="ajouterRacine()">إضافة جذر</button>
      <button (click)="chercherRacine()">بحث</button>
      <button (click)="genererEtStockerTous()">توليد جميع المشتقات</button>
      <button (click)="afficherDerivesStockes()">عرض المشتقات المخزنة</button>
      <button (click)="afficherRacines()">عرض كل الجذور</button>
    </div>

    <div class="persistence-buttons">
      <button (click)="sauvegarder()">💾 حفظ</button>
      <button (click)="restaurer()">📂 استرجاع</button>
    </div>
  </div>

  <div class="result-section">
    <pre>{{ resultat }}</pre>
  </div>

  <div class="racines-list">
    <h3>الجذور المخزنة ({{ racines.length }})</h3>
    <ul>
      <li *ngFor="let r of racines">
        {{ r.racine }} - {{ r.nombre_derives }} مشتقات
      </li>
    </ul>
  </div>

  <div class="schemes-list">
    <h3>الأوزان المتاحة ({{ schemes.length }})</h3>
    <ul>
      <li *ngFor="let s of schemes">{{ s.nom }} ({{ s.id }})</li>
    </ul>
  </div>
</div>
```

### 6. Styles CSS (morpho.component.css)

```css
.morpho-container {
  max-width: 900px;
  margin: 0 auto;
  padding: 20px;
  font-family: "Amiri", "Arial", sans-serif;
  direction: rtl;
}

.arabic-input {
  font-size: 24px;
  padding: 10px;
  width: 100%;
  text-align: center;
  border: 2px solid #4caf50;
  border-radius: 5px;
  direction: rtl;
}

.button-group {
  display: flex;
  gap: 10px;
  margin: 15px 0;
  flex-wrap: wrap;
}

button {
  padding: 10px 20px;
  background-color: #4caf50;
  color: white;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  font-size: 16px;
}

button:hover {
  background-color: #45a049;
}

.result-section {
  background-color: #f5f5f5;
  padding: 15px;
  border-radius: 5px;
  margin: 15px 0;
  min-height: 100px;
}

.result-section pre {
  white-space: pre-wrap;
  font-size: 18px;
  direction: rtl;
}

.racines-list,
.schemes-list {
  margin-top: 20px;
  background-color: #fff;
  padding: 15px;
  border: 1px solid #ddd;
  border-radius: 5px;
}

.racines-list ul,
.schemes-list ul {
  list-style: none;
  padding: 0;
}

.racines-list li,
.schemes-list li {
  padding: 8px;
  border-bottom: 1px solid #eee;
}

.persistence-buttons {
  margin-top: 10px;
  display: flex;
  gap: 10px;
}
```

## 📋 Checklist d'intégration

- [ ] Copier le dossier `pkg/` dans `src/assets/wasm/`
- [ ] Créer le service `morpho.service.ts`
- [ ] Appeler `await this.morphoService.initialize()` dans `ngOnInit()`
- [ ] Configurer `dir="rtl"` sur les éléments arabes
- [ ] Ajouter police arabe (Amiri, Scheherazade, etc.)
- [ ] Tester toutes les opérations (ajouter, chercher, générer, stocker)
- [ ] Implémenter la sauvegarde localStorage (optionnel)

## 🌐 Déploiement sur Netlify

### netlify.toml

```toml
[build]
  command = "npm run build"
  publish = "dist/nom-projet"

[[headers]]
  for = "*.wasm"
  [headers.values]
    Content-Type = "application/wasm"
```

### Notes importantes

1. **Différence clés** :
   - `generer_derive()` : génère SANS stocker
   - `generer_et_stocker_derive()` : génère ET stocke (comme terminal)

2. **Persistance** : Les données sont en mémoire. Pour sauvegarder :

   ```typescript
   // Sauvegarder
   localStorage.setItem("data", this.morphoService.exporterDonnees());

   // Restaurer au démarrage
   const data = localStorage.getItem("data");
   if (data) this.morphoService.importerDonnees(data);
   ```

3. **Rafraîchissement page** : Toutes les données sont perdues sauf si sauvegardées dans localStorage

## ✅ Toutes les opérations disponibles

| Opération        | Fonction                                  | Description                 |
| ---------------- | ----------------------------------------- | --------------------------- |
| Ajouter racine   | `ajouter_racine(racine)`                  | Ajoute une racine trilatère |
| Chercher racine  | `chercher_racine(racine)`                 | Vérifie existence           |
| Supprimer racine | `supprimer_racine(racine)`                | Supprime racine + dérivés   |
| Charger racines  | `charger_racines_depuis_texte(texte)`     | Charge multiples racines    |
| Générer dérivé   | `generer_et_stocker_derive(racine, id)`   | Génère + stocke             |
| Générer tous     | `generer_et_stocker_tous_derives(racine)` | Tous dérivés + stocke       |
| Valider mot      | `valider_mot_derive(racine, mot)`         | Valide dérivé               |
| Afficher dérivés | `obtenir_derives_stockes(racine)`         | Liste stockés               |
| Supprimer dérivé | `supprimer_derive(racine, mot, id)`       | Supprime un dérivé          |
| Afficher arbre   | `obtenir_toutes_racines()`                | Toutes racines              |
| Gérer schémas    | `ajouter_scheme()`, `supprimer_scheme()`  | CRUD schémas                |
| Sauvegarder      | `exporter_donnees()`                      | JSON export                 |
| Restaurer        | `importer_donnees(json)`                  | JSON import                 |

🎉 **Votre moteur morphologique est prêt pour Angular !**
