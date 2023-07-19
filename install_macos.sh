#!/bin/bash
cd ~/Library/'Application Support'
mkdir cursed_pong
cd cursed_pong
echo "Downloading application files..."
wget https://github.com/cursedpongdevs/cursed_pong/files/files.zip
unzip files.zip
rm files.zip
cd ~/Applications
echo "Downloading application..."
wget https://github.com/cursedpongdevs/cursed_pong/files/cursed_pong.app
echo "Done!"