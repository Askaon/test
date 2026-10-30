# Kalkulator (Rust + egui) na Androida

Aplikacja kalkulatora z interfejsem graficznym, napisana w całości w Ruście
(biblioteka GUI: egui/eframe, uruchamiana jako NativeActivity — bez Javy/Kotlina).

## Budowanie APK przez GitHub (najprościej)

1. Załóż nowe repozytorium na github.com.
2. Wgraj do niego całą zawartość tego folderu (razem z ukrytym folderem `.github`).
3. Wejdź w zakładkę **Actions** → workflow **Zbuduj APK** uruchomi się sam
   (albo kliknij *Run workflow*).
4. Po kilku minutach otwórz zakończony przebieg i pobierz artefakt
   **kalkulator-apk** (plik .zip z `app-debug.apk` w środku).
5. Przenieś APK na telefon i zainstaluj (trzeba zezwolić na instalację
   z nieznanych źródeł).

## Budowanie lokalnie

Wymagane: Rust, Android SDK + NDK, JDK 17, Gradle 8.9+.

```
rustup target add aarch64-linux-android armv7-linux-androideabi
cargo install cargo-ndk
cargo ndk -t arm64-v8a -t armeabi-v7a -o android/app/src/main/jniLibs build --release
cd android && gradle assembleDebug
```

APK: `android/app/build/outputs/apk/debug/app-debug.apk`

## Struktura

- `src/calc.rs` — silnik obliczeń (parser wyrażeń, testy: `cargo test` nie jest potrzebne do budowy)
- `src/lib.rs` — interfejs graficzny
- `android/` — minimalny projekt Gradle pakujący bibliotekę Rusta w APK
- `.github/workflows/build-apk.yml` — automatyczne budowanie APK
