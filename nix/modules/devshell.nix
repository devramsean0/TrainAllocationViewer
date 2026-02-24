{ inputs, ... }:
{
  perSystem = { config, self', pkgs, lib, ... }:
    let
      # Pkgs with unfree and Android license accepted
      androidPkgs = import inputs.nixpkgs {
        inherit (pkgs) system;
        config = {
          allowUnfree = true;
          android_sdk.accept_license = true;
        };
      };

      androidComposition = androidPkgs.androidenv.composeAndroidPackages {
        cmdLineToolsVersion = "11.0";
        platformToolsVersion = "35.0.1";
        buildToolsVersions = [ "34.0.0" ];
        platformVersions = [ "34" ];
        ndkVersions = [ "26.1.10909125" ];
        includeEmulator = true;
        includeSystemImages = true;
        systemImageTypes = [ "google_apis_playstore" ];
        abiVersions = [ "arm64-v8a" "x86_64" ];
        includeNDK = true;
      };
      androidSdk = androidComposition.androidsdk;

      # Wrap Android Studio to use our SDK
      android-studio-with-sdk = androidPkgs.android-studio.override {
        forceWayland = false;
      };
    in
    {
      devShells.default = pkgs.mkShell {
        name = "train-allocation-viewer-shell";
        inputsFrom = [
          self'.devShells.rust
          config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
        ];
        packages = with pkgs; [
          just
          nixd # Nix language server
          bacon
          openssl
          pkg-config
          wrapGAppsHook4

          sqlx-cli
          sqlite
          cargo-tauri

          librsvg
          webkitgtk_4_1
        ];

        shellHook = ''
          export PATH="$HOME/.cargo/bin:$PATH"
        '';
      };

      # Separate devshell for Android development
      devShells.android = pkgs.mkShell {
        name = "train-allocation-viewer-android-shell";
        inputsFrom = [
          self'.devShells.default
        ];
        packages = [
          androidSdk
          androidPkgs.jdk17
          android-studio-with-sdk
        ];

        shellHook = ''
                  export PATH="$HOME/.cargo/bin:$PATH"
        
                  # Create a writable Android SDK directory
                  export ANDROID_SDK_DIR="$HOME/.android-sdk"
                  export ANDROID_USER_HOME="$HOME/.android"
                  mkdir -p "$ANDROID_USER_HOME"
                  mkdir -p "$ANDROID_SDK_DIR"
        
                  # Copy SDK to writable location if not already done
                  NIX_SDK="${androidSdk}/libexec/android-sdk"
                  if [ ! -f "$ANDROID_SDK_DIR/.nix-sdk-installed" ] || [ "$(cat "$ANDROID_SDK_DIR/.nix-sdk-installed" 2>/dev/null)" != "${androidSdk}" ]; then
                    echo "Setting up writable Android SDK at $ANDROID_SDK_DIR..."
                    rm -rf "$ANDROID_SDK_DIR"
                    mkdir -p "$ANDROID_SDK_DIR"
          
                    # Copy the SDK structure
                    cp -r "$NIX_SDK"/* "$ANDROID_SDK_DIR/"
          
                    # Make everything writable
                    chmod -R u+w "$ANDROID_SDK_DIR"
          
                    # Mark as installed with this SDK version
                    echo "${androidSdk}" > "$ANDROID_SDK_DIR/.nix-sdk-installed"
                    echo "Android SDK setup complete!"
                  fi
        
                  # Set up SDK paths to writable location
                  export ANDROID_HOME="$ANDROID_SDK_DIR"
                  export ANDROID_SDK_ROOT="$ANDROID_SDK_DIR"
                  export NDK_HOME="$ANDROID_SDK_DIR/ndk/26.1.10909125"
                  export JAVA_HOME="${androidPkgs.jdk17}"
        
                  # Create local.properties hint for Android Studio
                  if [ ! -f "$PWD/local.properties" ]; then
                    echo "sdk.dir=$ANDROID_SDK_DIR" > "$PWD/local.properties"
                    echo "ndk.dir=$ANDROID_SDK_DIR/ndk/26.1.10909125" >> "$PWD/local.properties"
                  fi
        
                  # Gradle configuration for Android
                  export GRADLE_USER_HOME="$HOME/.gradle"
                  mkdir -p "$GRADLE_USER_HOME"
        
                  # Write gradle.properties to help Android Studio find SDK
                  if [ ! -f "$GRADLE_USER_HOME/gradle.properties" ] || ! grep -q "android.sdk.channel" "$GRADLE_USER_HOME/gradle.properties" 2>/dev/null; then
                    cat >> "$GRADLE_USER_HOME/gradle.properties" << EOF
          android.useAndroidX=true
          android.sdk.channel=0
          EOF
                  fi
        '';
      };
    };
}
