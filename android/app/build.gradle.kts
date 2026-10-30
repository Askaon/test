plugins {
    id("com.android.application")
}

android {
    namespace = "pl.kalkulator.rust"
    compileSdk = 34

    defaultConfig {
        applicationId = "pl.kalkulator.rust"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }
}
