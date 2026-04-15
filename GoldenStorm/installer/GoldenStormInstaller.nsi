!addincludedir "C:\Program Files (x86)\NSIS\Include"

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

; (No header image for now – avoids PNG/BMP issues)

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

    ; These exist in dist/ when build.ps1 runs NSIS from dist
    File "GoldenStorm.exe"
    File "GoldenStormAgent.exe"

    ; ICONS
    SetOutPath "$INSTDIR\assets\icons"
    File "assets\icons\*.*"

    ; UI ASSETS
    SetOutPath "$INSTDIR\assets"
    File "assets\index.html"
    File "assets\app.js"
    File "assets\style.css"

    ; STATE DIR
    SetOutPath "$INSTDIR\assets\state"

    ; SHORTCUTS
    CreateDirectory "$SMPROGRAMS\${APP_NAME}"
    CreateShortCut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "$INSTDIR\${EXE_NAME}" "" "$INSTDIR\assets\icons\app.ico"
    CreateShortCut "$DESKTOP\${APP_NAME}.lnk" "$INSTDIR\${EXE_NAME}" "" "$INSTDIR\assets\icons\app.ico"

    ; REGISTER AGENT
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "${APP_NAME}Agent" '"$INSTDIR\${AGENT_EXE_NAME}"'
SectionEnd

; --------------------------------------------
; UNINSTALL SECTION
; --------------------------------------------
Section "Uninstall"
    Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
    RMDir "$SMPROGRAMS\${APP_NAME}"
    Delete "$DESKTOP\${APP_NAME}.lnk"

    DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "${APP_NAME}Agent"

    Delete "$LOCALAPPDATA\GoldenStorm\*.*"
    Delete "$APPDATA\GoldenStorm\*.*"
    RMDir "$LOCALAPPDATA\GoldenStorm"
    RMDir "$APPDATA\GoldenStorm"

    RMDir /r "$INSTDIR"
SectionEnd
