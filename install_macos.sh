#!/bin/bash
cd ~/Library/'Application Support'
mkdir Cursed-Pong
cd Cursed-Pong
echo "Downloading application files..."
wget https://narwhal-studios.github.io/Cursed-Pong/files/files.zip
unzip files.zip
rm files.zip
cd ~/Applications
echo "Downloading application..."
wget https://cursedpongdevs.github.io/Cursed-Pong/files/Cursed-Pong.zip
unzip Cursed-Pong.zip
rm Cursed-Pong.zip
echo "Done!"