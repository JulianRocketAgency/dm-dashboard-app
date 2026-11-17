# DM Dashboard (Mac- en Windows-app)

App (macOS en Windows) die het DM Projects dashboard opent in een eigen venster. De inhoud (dagen, leads) komt live van de website, dus die is altijd actueel. De app zelf controleert bij het starten (en elke 4 uur) op een nieuwe versie van de app en vraagt: **Nu updaten** of **Later**. Handmatig kan via het menu *DM Dashboard > Controleer op updates*.

## Installeren op Mac
1. Ga naar **Releases** en download het `.dmg`-bestand.
2. Open het en sleep **DM Dashboard** naar Programma's.
3. De app is niet ondertekend met een Apple Developer-account. De eerste keer vraagt macOS daarom een bevestiging:
   - Open de app, krijg je een melding dat hij niet geopend kan worden, ga dan naar **Systeeminstellingen > Privacy en beveiliging**, scroll naar beneden en klik **Open toch**.
   - Lukt dat niet (melding "beschadigd"), voer dan eenmalig uit in Terminal: `xattr -cr "/Applications/DM Dashboard.app"`
4. Updates daarna gaan via de app zelf en vragen dit niet opnieuw.

## Installeren op Windows
1. Ga naar **Releases** en download het `.exe`-bestand (`DM.Dashboard_..._x64-setup.exe`).
2. Voer het uit. Het installeert voor jouw gebruiker, er zijn geen beheerdersrechten nodig.
3. De app is niet ondertekend met een betaald certificaat. Windows toont daarom eenmalig *"Windows heeft uw pc beschermd"*. Klik op **Meer informatie** en dan op **Toch uitvoeren**.
4. Updates daarna gaan via de app zelf (*Bestand > Controleer op updates*, of automatisch bij het starten).

## Een nieuwe versie uitbrengen
1. Verhoog het versienummer in `package.json`, `src-tauri/tauri.conf.json` en `src-tauri/Cargo.toml` (alle drie hetzelfde).
2. Commit en maak een tag: `git tag v1.0.1 && git push && git push --tags`
3. GitHub bouwt automatisch de app, ondertekent de update en publiceert de release. Geïnstalleerde apps zien de update binnen enkele uren, of direct via *Controleer op updates*.

Het dashboard zelf (de website) hoeft hiervoor niet opnieuw uitgebracht te worden, die is altijd direct actueel. Een nieuwe app-versie is alleen nodig bij wijzigingen aan de app (adres, icoon, menu).

## Adres wijzigen
Het adres staat in `src-tauri/tauri.conf.json` bij `app.windows[0].url`. Nu: het Railway-adres. Zodra `dashboard.dmprojects.nl` werkt, zet dat daar en breng een nieuwe versie uit.

## Update-sleutel
De updates worden ondertekend met een eigen sleutel (niet Apple). De privésleutel staat als GitHub-secret (`TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`) en als back-up lokaal in `~/.tauri/`. Raak je die kwijt, dan kunnen bestaande installaties geen updates meer ontvangen en moeten ze opnieuw geïnstalleerd worden.
