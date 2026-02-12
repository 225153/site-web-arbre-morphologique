# Moteur Morphologique Arabe

Moteur d'analyse morphologique pour l'arabe avec interface web Angular + WebAssembly.

## 📁 Structure

```
moteur_morphologique/
├── src/              # Code Rust (moteur morphologique)
├── morpho-web/       # Application Angular
└── pkg/              # WebAssembly généré (ignoré par git)
```

## 🚀 Déploiement sur Netlify

### 1. Pousser sur GitHub

```bash
cd "C:\Users\omar\Desktop\arbre morphologique\moteur_morphologique"
git init
git add .
git commit -m "Initial commit - Moteur morphologique arabe"
git remote add origin <URL_DE_TON_REPO_GITHUB>
git push -u origin main
```

### 2. Compiler le WebAssembly

```bash
wasm-pack build --target web
Copy-Item -Path pkg/* -Destination morpho-web/public/wasm/ -Force
```

### 3. Builder le projet Angular

```bash
cd morpho-web
npm install
npm run build
```

### 4. Déployer sur Netlify

#### Option A : Via l'interface Netlify (recommandée)

1. Connecte-toi sur [netlify.com](https://netlify.com)
2. Clique sur **"Add new site"** → **"Import an existing project"**
3. Sélectionne **GitHub** et choisis ton repository
4. Configure les paramètres de build :
   - **Build command:** `cd morpho-web && npm install && npm run build`
   - **Publish directory:** `morpho-web/dist/morpho-web/browser`
   - **Base directory:** (laisser vide)
5. Clique sur **"Deploy site"**

#### Option B : Via Netlify CLI

```bash
npm install -g netlify-cli
cd morpho-web
netlify deploy --prod
```

### 5. Configuration automatique

Le fichier `morpho-web/netlify.toml` est déjà configuré avec :

- Build command pour Angular
- Headers CORS pour WebAssembly
- Redirections SPA
- Optimisations de cache

## 🛠️ Développement

### Compiler Rust → WASM

```bash
wasm-pack build --target web
```

### Lancer Angular en développement

```bash
cd morpho-web
npm start
# Ouvre http://localhost:4200
```

### Lancer le moteur en terminal

```bash
cargo run
```

## 📱 Application mobile (Capacitor)

Voir `morpho-web/DEPLOIEMENT.md` pour les instructions de transformation en application mobile.

## 🔧 Technologies

- **Backend:** Rust (BSR tree, hash table)
- **WebAssembly:** wasm-pack, wasm-bindgen
- **Frontend:** Angular 21, TypeScript
- **Déploiement:** Netlify
- **Mobile:** Capacitor (iOS/Android)
