# Arabic Morphological Analyzer

Application web d'analyse morphologique arabe avec **Rust WebAssembly** + **Angular 21**.

## Fonctionnalites

- Gestion des racines trilaterales (ajouter, chercher, supprimer)
- Generation automatique des derives
- Validation des mots
- CRUD sur les 15 schemas morphologiques arabes
- Visualisation arborescente
- Sauvegarde localStorage

## Installation

\\\ash
npm install
\\\

## Developpement

\\\ash
npm start
\\\

Ouvrir http://localhost:4200

## Build

\\\ash
npm run build
\\\

## Deploiement Netlify

### Option 1: Git

\\\ash
git init
git add .
git commit -m 'Initial commit'
git push
\\\

Sur Netlify: Import project (netlify.toml configure tout)

### Option 2: Drag & Drop

Glisser dist/morpho-web/browser/ sur https://app.netlify.com/drop

## Mobile Capacitor

### Installation
\\\ash
npm install @capacitor/core @capacitor/cli
npm install @capacitor/android @capacitor/ios
\\\

### Init
\\\ash
npx cap init
\\\

### Android
\\\ash
npx cap add android
npm run build
npx cap sync android
npx cap open android
\\\

### iOS
\\\ash
npx cap add ios
npm run build
npx cap sync ios
npx cap open ios
\\\

### Config capacitor.config.ts

\\\	ypescript
import { CapacitorConfig } from '@capacitor/cli';

const config: CapacitorConfig = {
  appId: 'com.morpho.analyzer',
  appName: 'Arabic Morpho Analyzer',
  webDir: 'dist/morpho-web/browser',
  server: { androidScheme: 'https' }
};

export default config;
\\\

Pret pour Netlify et mobile!
