import { Component, OnInit, signal, computed } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MorphoService, DeriveInfo, RacineInfo, SchemeInfo } from './services/morpho.service';

type ActiveTab = 'racines' | 'derives' | 'validation' | 'schemes' | 'arbre';

@Component({
  selector: 'app-root',
  imports: [FormsModule],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App implements OnInit {
  // État global
  loading = signal(true);
  erreur = signal('');
  message = signal('');

  // Onglet actif
  activeTab = signal<ActiveTab>('racines');

  // Racines
  racineInput = signal('');
  racinesTexte = signal('');
  racinesList = signal<RacineInfo[]>([]);

  // Dérivés
  deriveRacine = signal('');
  deriveSchema = signal('');
  derivesGeneres = signal<DeriveInfo[]>([]);
  derivesStockes = signal<DeriveInfo[]>([]);

  // Validation
  validationRacine = signal('');
  validationMot = signal('');
  validationResultat = signal<{valide: boolean, schema: string} | null>(null);

  // Schemes
  schemesList = signal<SchemeInfo[]>([]);
  nouveauSchemeNom = signal('');
  nouveauSchemePattern = signal('');
  nouveauSchemeDesc = signal('');

  // Statistiques
  totalRacines = computed(() => this.racinesList().length);
  totalSchemes = computed(() => this.schemesList().length);

  constructor(private morpho: MorphoService) {}

  async ngOnInit() {
    try {
      await this.morpho.initialize();
      this.loading.set(false);
      this.rafraichirSchemes();
      this.rafraichirArbre();
    } catch (e) {
      this.loading.set(false);
      this.erreur.set('خطأ في تهيئة المحلل الصرفي');
      console.error(e);
    }
  }

  // === NAVIGATION ===
  setTab(tab: ActiveTab) {
    this.activeTab.set(tab);
    this.message.set('');
    this.erreur.set('');
    if (tab === 'arbre') {
      this.rafraichirArbre();
    }
  }

  // === RACINES ===
  ajouterRacine() {
    const racine = this.racineInput().trim();
    if (!racine) { this.erreur.set('أدخل الجذر'); return; }
    try {
      const result = this.morpho.ajouterRacine(racine);
      this.message.set(`تمت إضافة الجذر: ${racine} — ${result}`);
      this.erreur.set('');
      this.racineInput.set('');
      this.rafraichirArbre();
    } catch (e: any) {
      this.erreur.set(e.message || 'خطأ في إضافة الجذر');
      this.message.set('');
    }
  }

  chercherRacine() {
    const racine = this.racineInput().trim();
    if (!racine) { this.erreur.set('أدخل الجذر'); return; }
    try {
      const existe = this.morpho.chercherRacine(racine);
      if (existe) {
        this.message.set(`✓ الجذر "${racine}" موجود في الشجرة`);
      } else {
        this.message.set(`✗ الجذر "${racine}" غير موجود`);
      }
      this.erreur.set('');
    } catch (e: any) {
      this.erreur.set(e.message || 'خطأ في البحث');
      this.message.set('');
    }
  }

  supprimerRacine() {
    const racine = this.racineInput().trim();
    if (!racine) { this.erreur.set('أدخل الجذر'); return; }
    try {
      const ok = this.morpho.supprimerRacine(racine);
      if (ok) {
        this.message.set(`✓ تم حذف الجذر "${racine}" ومشتقاته`);
      } else {
        this.message.set(`✗ الجذر "${racine}" غير موجود`);
      }
      this.erreur.set('');
      this.racineInput.set('');
      this.rafraichirArbre();
    } catch (e: any) {
      this.erreur.set(e.message || 'خطأ في الحذف');
      this.message.set('');
    }
  }

  chargerRacines() {
    const texte = this.racinesTexte().trim();
    if (!texte) { this.erreur.set('أدخل نصاً يحتوي على جذور'); return; }
    try {
      const count = this.morpho.chargerRacinesDepuisTexte(texte);
      this.message.set(`✓ تم تحميل ${count} جذر من النص`);
      this.erreur.set('');
      this.racinesTexte.set('');
      this.rafraichirArbre();
    } catch (e: any) {
      this.erreur.set(e.message || 'خطأ في تحميل الجذور');
      this.message.set('');
    }
  }

  // === DÉRIVÉS ===
  genererTousDerives() {
    const racine = this.deriveRacine().trim();
    if (!racine) { this.erreur.set('أدخل الجذر'); return; }
    try {
      if (!this.morpho.chercherRacine(racine)) {
        this.erreur.set('✗ الجذر غير موجود في الشجرة. أضفه أولاً');
        this.message.set('');
        return;
      }
      const derives = this.morpho.genererTousDerives(racine);
      this.derivesGeneres.set(derives);
      this.message.set(`${derives.length} مشتق تم توليده`);
      this.erreur.set('');
    } catch (e: any) {
      this.erreur.set(e.message || 'خطأ في التوليد');
      this.message.set('');
    }
  }

  genererEtStockerTous() {
    const racine = this.deriveRacine().trim();
    if (!racine) { this.erreur.set('أدخل الجذر'); return; }
    try {
      if (!this.morpho.chercherRacine(racine)) {
        this.erreur.set('✗ الجذر غير موجود في الشجرة. أضفه أولاً');
        this.message.set('');
        return;
      }
      const count = this.morpho.genererEtStockerTousDerives(racine);
      if (count === 0) {
        this.erreur.set('✗ الجذر غير موجود في الشجرة');
        this.message.set('');
      } else {
        this.message.set(`✓ ${count} مشتق تم توليده وتخزينه`);
        this.erreur.set('');
        this.afficherDerivesStockes();
        this.rafraichirArbre();
      }
    } catch (e: any) {
      this.erreur.set(e.message || 'خطأ في التوليد والتخزين');
      this.message.set('');
    }
  }

  genererDerive() {
    const racine = this.deriveRacine().trim();
    const schema = this.deriveSchema().trim();
    if (!racine || !schema) { this.erreur.set('أدخل الجذر والوزن'); return; }
    try {
      if (!this.morpho.chercherRacine(racine)) {
        this.erreur.set('✗ الجذر غير موجود في الشجرة. أضفه أولاً');
        this.message.set('');
        return;
      }
      const mot = this.morpho.genererDerive(racine, schema);
      if (mot) {
        this.message.set(`المشتق: ${mot}`);
        this.derivesGeneres.set([{ mot, schema }]);
      } else {
        this.message.set('لم يتم توليد مشتق');
      }
      this.erreur.set('');
    } catch (e: any) {
      this.erreur.set(e.message || 'خطأ في التوليد');
      this.message.set('');
    }
  }

  genererEtStockerDerive() {
    const racine = this.deriveRacine().trim();
    const schema = this.deriveSchema().trim();
    if (!racine || !schema) { this.erreur.set('أدخل الجذر والوزن'); return; }
    try {
      if (!this.morpho.chercherRacine(racine)) {
        this.erreur.set('✗ الجذر غير موجود في الشجرة. أضفه أولاً');
        this.message.set('');
        return;
      }
      const ok = this.morpho.genererEtStockerDerive(racine, schema);
      if (ok) {
        this.message.set(`✓ تم توليد وتخزين المشتق بوزن ${schema}`);
        this.erreur.set('');
        this.afficherDerivesStockes();
        this.rafraichirArbre();
      } else {
        this.erreur.set('✗ الجذر غير موجود في الشجرة');
        this.message.set('');
      }
    } catch (e: any) {
      this.erreur.set(e.message || 'خطأ');
      this.message.set('');
    }
  }

  afficherDerivesStockes() {
    const racine = this.deriveRacine().trim();
    if (!racine) return;
    try {
      if (!this.morpho.chercherRacine(racine)) {
        this.erreur.set('✗ الجذر غير موجود في الشجرة');
        this.derivesStockes.set([]);
        return;
      }
      this.derivesStockes.set(this.morpho.obtenirDerivesStockes(racine));
    } catch {
      this.derivesStockes.set([]);
    }
  }

  supprimerDerive(mot: string) {
    const racine = this.deriveRacine().trim();
    if (!racine) return;
    try {
      if (!this.morpho.chercherRacine(racine)) {
        this.erreur.set('✗ الجذر غير موجود في الشجرة');
        return;
      }
      this.morpho.supprimerDerive(racine, mot);
      this.message.set(`✓ تم حذف المشتق "${mot}"`);
      this.afficherDerivesStockes();
      this.rafraichirArbre();
    } catch (e: any) {
      this.erreur.set(e.message || 'خطأ في الحذف');
    }
  }

  // === VALIDATION ===
  validerMot() {
    const mot = this.validationMot().trim();
    const racine = this.validationRacine().trim();
    if (!mot || !racine) { this.erreur.set('أدخل الكلمة والجذر'); return; }
    try {
      if (!this.morpho.chercherRacine(racine)) {
        this.erreur.set('✗ الجذر غير موجود في الشجرة. أضفه أولاً');
        this.message.set('');
        this.validationResultat.set(null);
        return;
      }
      const result = this.morpho.validerMotDerive(mot, racine);
      this.validationResultat.set(result);
      if (result.valide) {
        this.message.set(`✓ الكلمة "${mot}" صحيحة - الوزن: ${result.schema}`);
      } else {
        this.message.set(`✗ الكلمة "${mot}" غير مطابقة لأي وزن`);
      }
      this.erreur.set('');
    } catch (e: any) {
      this.erreur.set(e.message || 'خطأ في التحقق');
      this.message.set('');
    }
  }

  // === SCHEMES ===
  rafraichirSchemes() {
    try {
      this.schemesList.set(this.morpho.obtenirTousSchemes());
    } catch {
      this.schemesList.set([]);
    }
  }

  ajouterScheme() {
    const nom = this.nouveauSchemeNom().trim();
    const pattern = this.nouveauSchemePattern().trim();
    const desc = this.nouveauSchemeDesc().trim();
    if (!nom || !pattern) { this.erreur.set('أدخل اسم ونمط الوزن'); return; }
    try {
      const ok = this.morpho.ajouterScheme(nom, pattern, desc);
      if (ok) {
        this.message.set(`✓ تم إضافة الوزن "${nom}"`);
        this.nouveauSchemeNom.set('');
        this.nouveauSchemePattern.set('');
        this.nouveauSchemeDesc.set('');
      } else {
        this.message.set('لم تتم الإضافة (ربما الوزن موجود)');
      }
      this.erreur.set('');
      this.rafraichirSchemes();
    } catch (e: any) {
      this.erreur.set(e.message || 'خطأ');
      this.message.set('');
    }
  }

  supprimerScheme(nom: string) {
    try {
      this.morpho.supprimerScheme(nom);
      this.message.set(`✓ تم حذف الوزن "${nom}"`);
      this.rafraichirSchemes();
    } catch (e: any) {
      this.erreur.set(e.message || 'خطأ في الحذف');
    }
  }

  // === ARBRE ===
  rafraichirArbre() {
    try {
      this.racinesList.set(this.morpho.obtenirToutesRacines());
    } catch {
      this.racinesList.set([]);
    }
  }
}
