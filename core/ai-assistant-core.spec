# -*- mode: python ; coding: utf-8 -*-
import platform

app_name = "core"


# Follow RUST Target Syntax
# https://doc.rust-lang.org/rustc/platform-support.html
system = platform.system().lower()
system_map = {
    'darwin': 'apple-darwin',
    'windows': 'pc-windows-msvc',
    'linux': 'unknown-linux-gnu',
    
}
system = system_map.get(system, system)

# Follow RUST Target Syntax
# https://doc.rust-lang.org/rustc/platform-support.html
architecture = platform.machine().lower()
architecture_map = {
    'arm64': 'aarch64',
    'amd64': 'x86_64',
}
architecture = architecture_map.get(architecture, architecture)


output_name = f"{app_name}-{architecture}-{system}"


a = Analysis(
    ['ai_assistant_core/main.py'],
    pathex=[],
    binaries=[],
    datas=[],
    hiddenimports=['tiktoken_ext.openai_public', 'tiktoken_ext'],
    hookspath=['./pyinstaller_hooks'],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    noarchive=False,
    optimize=0,
)
pyz = PYZ(a.pure)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.datas,
    [],
    name=output_name,
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=True,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
)
