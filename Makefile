dmg:
	./dmg

macospy:
	pyinstaller -Fi logo.icns python/updater.py
	pyinstaller -Fi logo.icns python/files.py

windowspy:
	pyinstaller -Fi logo.ico python/installer_windows.py
	pyinstaller -Fi logo.ico python/updater.py
	pyinstaller -Fi logo.ico python/files.py

game:
	cd game;cargo build --release

windowssync:
	mv game/target/release/game.exe files/windows.exe
	mv dist/updater.exe files
	mv dist/files.exe files
	zip files/*
	mv files/0.zip Cursed-Pong/files/windows.zip
	rm files/*.exe

macossync:
	mkdir files/Cursed\ Pong.app
	mv game/target/release/game files/Cursed\ Pong.app/Cursed\ Pong
	mkdir files/Updater.app
	mv dist/updater files/Updater.app/Updater
	mkdir files/Files.app
	mv dist/files files/Files.app/Files
	zip files/*
	mv files/0.zip Cursed-Pong/files/macos.zip
	rm files/*.app

macos:
	cd game;cargo b -r
	pyinstaller -Fi logo.icns python/updater.py
	pyinstaller -Fi logo.icns python/files.py
	mkdir files/Cursed\ Pong.app
	mv game/target/release/game files/Cursed\ Pong.app/Cursed\ Pong
	mkdir files/Updater.app
	mv dist/updater files/Updater.app/Updater
	mkdir files/Files.app
	mv dist/files files/Files.app/Files
	zip files/*
	mv files/0.zip Cursed-Pong/files/macos.zip
	rm -r files/*.app
	cd game;cargo b -r
	mv game/target/release/game Cursed\ Pong.app/Cursed\ Pong
	./dmg
	mv Cursed\ Pong-Installer.dmg Cursed-Pong/files/Cursed-Pong.dmg
	cd Cursed-Pong;git add files/macos.zip;git add files/Cursed-Pong.dmg;git commit -m "Update";git push -u origin main

windows:
	cd game;cargo b -r
	pyinstaller -Fi logo.ico python/installer_windows.py
	pyinstaller -Fi logo.ico python/updater.py
	pyinstaller -Fi logo.ico python/files.py
	mv game/target/release/game.exe files/windows.exe
	mv dist/updater.exe files
	mv dist/files.exe files
	zip files/*
	mv files/0.zip Cursed-Pong/files/windows.zip
	rm files/*.exe
	mv dist/installer_windows.exe Cursed-Pong/files/Cursed-Pong.exe
	cd Cursed-Pong;git add files/windows.zip;git add files/Cursed-Pong.exe;git commit -m "Update"; git push -u origin main

clean:
	rm updater.spec files.spec 
	rm -r dist build files/Cursed\ Pong.app files/Updater.app files/Files.app
