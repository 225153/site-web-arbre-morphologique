# Script de build et préparation pour déploiement Netlify
# Usage: .\build-for-netlify.ps1

Write-Host "🦀 Compilation Rust → WebAssembly..." -ForegroundColor Cyan
wasm-pack build --target web

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Erreur lors de la compilation WASM" -ForegroundColor Red
    exit 1
}

Write-Host "📦 Copie des fichiers WASM vers Angular..." -ForegroundColor Cyan
Copy-Item -Path "pkg\*" -Destination "morpho-web\public\wasm\" -Force

Write-Host "🅰️  Build Angular..." -ForegroundColor Cyan
Set-Location morpho-web
npm install
npm run build

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Erreur lors du build Angular" -ForegroundColor Red
    Set-Location ..
    exit 1
}

Set-Location ..

Write-Host "" 
Write-Host "✅ Build terminé avec succès!" -ForegroundColor Green
Write-Host ""
Write-Host "📂 Le dossier de déploiement est: morpho-web\dist\morpho-web\browser" -ForegroundColor Yellow
Write-Host ""
Write-Host "🚀 Prochaines étapes:" -ForegroundColor Cyan
Write-Host "   1. Push sur GitHub: git add . && git commit -m 'Build WASM' && git push" -ForegroundColor White
Write-Host "   2. Déployer sur Netlify depuis GitHub" -ForegroundColor White
Write-Host ""
