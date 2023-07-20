#!/bin/bash
cd ~/Library/'Application Support'
mkdir cursed_pong
cd cursed_pong
echo "Downloading application files..."
wget https://cursedpongdevs.github.io/cursed_pong/files/files.zip
unzip files.zip
rm files.zip
cd ~/Applications
echo "Downloading application..."
wget https://cursedpongdevs.github.io/cursed_pong/files/cursed_pong.zip
unzip cursed_pong.zip
rm cursed_pong.zip
echo "Done!"