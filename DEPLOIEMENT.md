# Guide de deploiement rapide

## 1. Verifier que tout fonctionne localement

Le serveur de dev tourne actuellement sur http://localhost:4200

## 2. Preparer pour Git

```bash
cd "C:\Users\omar\Desktop\arbre morphologique\morpho-web"
git init
git add .
git commit -m "Initial commit - Arabic Morphological Analyzer"
```

## 3. Creer un repo sur GitHub

1. Aller sur https://github.com/new
2. Nom: morpho-web (ou autre nom)
3. Ne PAS initialiser avec README/gitignore (deja presents)
4. Creer le repo

## 4. Pousser le code

```bash
git remote add origin https://github.com/VOTRE-USERNAME/morpho-web.git
git branch -M main
git push -u origin main
```

## 5. Deployer sur Netlify

### Option A: Depuis GitHub (recommande)

1. Aller sur https://app.netlify.com
2. "Add new site" > "Import an existing project"
3. Connecter GitHub et selectionner le repo morpho-web
4. Configuration detectee automatiquement depuis netlify.toml
5. Cliquer "Deploy"
6. Votre site sera en ligne en 2-3 minutes!

### Option B: Drag & Drop

1. Builder localement: `npm run build`
2. Aller sur https://app.netlify.com/drop
3. Glisser le dossier `dist/morpho-web/browser/`
4. Site en ligne immediatement!

## 6. Mobile Capacitor (optionnel)

### Pour Android:

```bash
npm install @capacitor/core @capacitor/cli @capacitor/android
npx cap init
# App name: Arabic Morphological Analyzer
# Package ID: com.morpho.analyzer
npx cap add android
npm run build
npx cap sync android
npx cap open android
```

Dans Android Studio: Build > Build APK

### Pour iOS (macOS uniquement):

```bash
npm install @capacitor/ios
npx cap add ios
npm run build
npx cap sync ios
npx cap open ios
```

Dans Xcode: Product > Build

## Fichiers importants

- `netlify.toml` - Configuration Netlify (MIME type WASM, redirections SPA)
- `.gitignore` - Ignore node_modules, dist, android, ios, .netlify
- `README.md` - Documentation complete

## Notes

- Le WASM (126 KB) est bundle automatiquement
- Les donnees sont en memoire (localStorage optionnel)
- Le site est RTL par defaut avec police Amiri
- Tous les navigateurs modernes sont supportes

Bon deploiement!
