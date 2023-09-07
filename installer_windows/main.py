from tkinter import *
from zipfile import ZipFile
from pathlib import Path
import pymongo
import os
import sys
import requests
import subprocess

client = pymongo.MongoClient("mongodb+srv://cursedpong:noddycallum@cursed-pong.4rpcoc2.mongodb.net/")
db = client["db"]
col = db["codes"]

root = Tk()
root.title("Cursed Pong Installer")
root.geometry('960x540')

lbl = Label(root, text="Enter a code: ")
lbl.grid()

lbl2 = Label(root, text="")
lbl2.grid(column=0,row=1)

lbl3 = Label(root, text="")
lbl3.grid(column=0,row=2)

lbl4 = Label(root, text="")
lbl4.grid(column=0,row=3)

code = Entry(root, width=10)
code.grid(column=1,row=0)

def run(cmd):
    completed = subprocess.run(["powershell","-Command", cmd], capture_output=True)
    return completed

def clicked():
    result = col.find_one({"code":code.get()})
    if code.get() == "cpongcode2" or result != None:
        lbl2.configure(text="Correct code")
        lbl3.configure(text="Are you sure you would like to install Cursed Pong?")
        btn2 = Button(root, text="Yes", command=yes)
        btn2.grid(column=1,row=2)
        btn3 = Button(root, text="No", command=no)
        btn3.grid(column=2, row=2)
    else:
        lbl2.configure(text="Invalid code")

def yes():
    if code.get() != "cpongcode2":
        col.delete_one({"code":code.get()})
    path = Path(os.path.expanduser("~")+"\\AppData\\Roaming\\Cursed-Pong")
    print(path)
    command = "mkdir "+os.path.expanduser("~")+"\\AppData\\Roaming\\Cursed-Pong"
    commd_info = run(command)
    if commd_info.returncode != 0:
        print("An error occured: %s", commd_info.stderr)
        sys.exit()
    os.chdir(os.path.expanduser('~')+"\\AppData\\Roaming\\Cursed-Pong")
    url = 'https://narwhal-studios.github.io/Cursed-Pong/files/files.zip'
    command = "Invoke-WebRequest -Uri "+url+" -OutFile "+os.path.expanduser("~")+"\\AppData\\Roaming\\Cursed-Pong\\files.zip"
    commd_info = run(command)
    if commd_info.returncode != 0:
        print("An error occured: %s", commd_info.stderr)
        sys.exit()
    command = "cd "+os.path.expanduser("~")+"\\AppData\\Roaming\\Cursed-Pong;tar -xf files.zip"
    commd_info = run(command)
    if commd_info.returncode != 0:
        print("An error occured: %s", commd_info.stderr)
        sys.exit()
    os.remove('files.zip')
    #os.remove("macos.dmg")
    command = "$WshShell = New-Object -comObject WScript.Shell;$Shortcut = $WshShell.CreateShortcut(\"$Home\\Desktop\\Cursed Pong.lnk\");$Shortcut.TargetPath = \"$Home\\AppData\\Roaming\\Cursed-Pong\\windows.exe\";$Shortcut.Save()"
    commd_info = run(command)
    if commd_info.returncode != 0:
        print("An error occured: %s", commd_info.stderr)
        sys.exit()
    command = "$WshShell = New-Object -comObject WScript.Shell;$Shortcut = $WshShell.CreateShortcut(\"$Home\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Cursed Pong.lnk\");$Shortcut.TargetPath = \"$Home\\AppData\\Roaming\\Cursed-Pong\\windows.exe\";$Shortcut.Save()"
    commd_info = run(command)
    if commd_info.returncode != 0:
        print("An error occured: %s", commd_info.stderr)
        sys.exit()
    lbl4.configure(text="Cursed Pong has been installed")
    print("Done")
    btn4 = Button(root, text="Quit", command=no)
    btn4.grid(column=1,row=3)
    global installed
    installed = True

def no():
    sys.exit()

btn = Button(root, text="Go",command=clicked)
btn.grid(column=2,row=0)

root.mainloop()
