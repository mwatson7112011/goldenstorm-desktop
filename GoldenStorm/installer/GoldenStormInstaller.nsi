!define APP_NAME "GoldenStorm"
!define APP_DIR "GoldenStorm"
!define EXE_NAME "GoldenStorm.exe"
!define AGENT_EXE_NAME "GoldenStormAgent.exe"
!define APP_VERSION "1.0.0"   ; build.ps1 auto-updates this

!include "MUI2.nsh"

; --------------------------------------------
; Branding / UI
; --------------------------------------------
!define MUI_ICON "assets\icons\app.ico"
!define MUI_UNICON "assets\icons\app.ico"

!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_RIGHT
!define MUI_HEADERIMAGE_BITMAP "assets\icons\icon_256x256.png"

!define MUI_WELCOMEPAGE_TITLE "Welcome to ${APP_NAME} Setup"
!define MUI_WELCOMEPAGE_TEXT "This wizard will install ${APP_NAME} on your computer.\r\n\r\nGoldenStorm provides a personality-driven severe weather experience with a dedicated background agent."

!define MUI_FINISHPAGE_TITLE "Setup Complete"
!define MUI_FINISHPAGE_TEXT "GoldenStorm has been installed on your computer.\r\n\r\nYou can launch the app from the Start Menu or desktop shortcut."
!define MUI_FINISHPAGE_RUN "$INSTDIR\${EXE_NAME}"
!define MUI_FINISHPAGE_RUN_TEXT "Launch ${APP_NAME} now"

OutFile "GoldenStormSetup.exe"
InstallDir "$PROGRAMFILES\${APP_DIR}"
RequestExecutionLevel admin

; --------------------------------------------
; Pages
; --------------------------------------------
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

; --------------------------------------------
; INSTALL SECTION
; --------------------------------------------
Section "Install"
    SetOutPath "$INSTDIR"

    ; Main executables (we are running from dist)
    File "GoldenStorm.exe"
    File "GoldenStormAgent.exe"

    ; ----------------------------------------
    ; ICONS
    ; ----------------------------------------
    SetOutPath "$INSTDIR\assets\icons"
    File "assets\icons\app.ico"
    File "assets\icons\alert.ico"
    File "assets\icons\favicon.ico"

    File "assets\icons\icon_16x16.png"
    File "assets\icons\icon_24x24.png"
    File "assets\icons\icon_32x32.png"
    File "assets\icons\icon_48x48.png"
    File "assets\icons\icon_64x64.png"
    File "assets\icons\icon_128x128.png"
    File "assets\icons\icon_256x256.png"
    File "assets\icons\icon_512x512.png"
    File "assets\icons\icon_1024x1024.png"

    File "assets\icons\alert_16x16.png"
    File "assets\icons\alert_24x24.png"
    File "assets\icons\alert_32x32.png"
    File "assets\icons\alert_48x48.png"
    File "assets\icons\alert_64x64.png"
    File "assets\icons\alert_128x128.png"
    File "assets\icons\alert_256x256.png"
    File "assets\icons\alert_512x512.png"
    File "assets\icons\alert_1024x1024.png"

    ; ----------------------------------------
    ; UI ASSETS
    ; ----------------------------------------
    SetOutPath "$INSTDIR\assets"
    File "assets\index.html"
    File "assets\app.js"
    File "assets\style.css"

    ; ----------------------------------------
    ; STATE DIRECTORY
    ; ----------------------------------------
    SetOutPath "$INSTDIR\assets\state"

    ; ----------------------------------------
    ; SHORTCUTS
    ; ----------------------------------------
    CreateDirectory "$SMPROGRAMS\${APP_NAME}"
    CreateShortCut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" \
        "$INSTDIR\${EXE_NAME}" "" "$INSTDIR\assets\icons\app.ico"

    CreateShortCut "$DESKTOP\${APP_NAME}.lnk" \
        "$INSTDIR\${EXE_NAME}" "" "$INSTDIR\assets\icons\app.ico"

    ; ----------------------------------------
    ; REGISTER AGENT AT LOGIN (Current User)
    ; ----------------------------------------
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Run" \
        "${APP_NAME}Agent" '"$INSTDIR\${AGENT_EXE_NAME}"'
SectionEnd

; --------------------------------------------
; UNINSTALL SECTION
; --------------------------------------------
Section "Uninstall"
    ; Shortcuts
    Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
    RMDir "$SMPROGRAMS\${APP_NAME}"
    Delete "$DESKTOP\${APP_NAME}.lnk"

    ; Run key
    DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "${APP_NAME}Agent"

    ; Logs + config (AppData/Config dir)
    ; Matches Rust config/logging behavior
    Delete "$LOCALAPPDATA\GoldenStorm\*.*"
    Delete "$APPDATA\GoldenStorm\*.*"
    RMDir "$LOCALAPPDATA\GoldenStorm"
    RMDir "$APPDATA\GoldenStorm"

    ; Install dir
    RMDir /r "$INSTDIR"
SectionEnd
