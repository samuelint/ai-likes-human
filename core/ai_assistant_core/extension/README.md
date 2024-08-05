# Extensions

## State

Extension have 3 states

0. Uninstalled
   The extension files are not in the extension directory.
1. Installed
   The extension file exist in the directory. It may or may not be loaded.
2. Loaded
   The extension code is running.
   For example, PEX extensions are a local HTTP Server. When loaded, the servier is running and ready to recieve requests

## PEX Extensions

PEX are (Python EXecutable) https://docs.pex-tool.org

They are standalone python files packaged with their dependencies.
Main differences with pyinstaller package are:

- They are runnable on multiple platform if their dependencies are cross-plaform or contains a distribuable for each supported platform.
- They require python to be installed on the host machine. In the case of this app, the python distribuable used is the one packaged with the main app.
