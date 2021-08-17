#!/bin/bash

mkdir "$HOME/.steam/root/steamapps/compatdata/244850/"

export STEAM_COMPAT_CLIENT_INSTALL_PATH=.local/share/steam
export STEAM_COMPAT_DATA_PATH=$HOME/.steam/root/steamapps/compatdata/244850/

cd "$HOME/.steam/root/steamapps/common/SpaceEngineers/"
echo "244850" > ./steam_appid.txt
"$HOME/.steam/root/steamapps/common/Proton $1/proton" run "./Bin64/SpaceEngineers.exe"