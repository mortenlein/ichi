; Inno Setup Script for Ichi
#define MyAppName "Ichi"
#define MyAppVersion "0.1.3"
#define MyAppPublisher "Morten Lein"
#define MyAppURL "https://github.com/mortenlein/ichi"
#define MyAppExeName "ichi.exe"

[Setup]
AppId={{D3B3A6B1-7A3E-4B7E-B3A6-B1D3B3A6B1D3}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DisableProgramGroupPage=yes
OutputBaseFilename=IchiSetup
Compression=lzma
SolidCompression=yes
WizardStyle=modern
SetupIconFile=ichi.ico
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "startup"; Description: "Launch Ichi on Windows startup"; GroupDescription: "Additional options:"; Flags: checkedonce

[Files]
Source: "target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Registry]
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "{#MyAppName}"; ValueData: """{app}\{#MyAppExeName}"""; Tasks: startup

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent
