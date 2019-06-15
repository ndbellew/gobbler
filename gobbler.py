# Gobbler
# Author: Nathan Bellew
# Tool that takes in file/folder names, zips them in one temp folder,
# and sends them to trash.

import sys
import os
from optparse import OptionParser
import zipfile
from shutil import rmtree
import time
VERSION="Gobbler 0.1.0 Alpha"

usage = "Usage: %prog [ -s|--start ][options] [files]"
parser = OptionParser(usage)
parser.add_option('-s','--s',dest='start', help='Select starting location for gobbler to begin gobbling. ex. /home/usr/Documents')
parser.add_option('-F','--folder', action='store_true',dest='folder',help='If any of the files chosen are/are also folders, then they will be included.')
parser.add_option('-n', '--name', dest='name',help='Name Zip folder')
parser.add_option('-v','--version',action='store_true',dest='version',help='Shows current version of gobbler.py', default=False)
(options, args)= parser.parse_args()

if len(args) < 1 or options.start is None:
    parser.print_help()
    sys.exit(0)

def zipdir(path, ziph):
    for root, dirs, files in os.walk(path):
        for file in files:
            ziph.write(os.path.join(root, file))

def CreateFolders(folder):
    try:
        os.mkdir("/tmp/"+folder)
    except OSError:
        print ("Creation of the directory %s failed" % folder)

def LinuxMain(zipf):
    if not os.path.exists("/tmp/TooBeDeleted/"):
        CreateFolders("TooBeDeleted")
    if not os.path.exists("/tmp/TheHold/"):
        CreateFolders("TheHold")
    path = options.start
    os.chdir(path)
    for root, dirs, files in os.walk(path):
        for file in files:
            if file in args:
                zipf.write(file)
                os.remove(file)
        for dir in dirs:
            if dir in args and options.folder:
                zipdir(dir, zipf)
                rmtree(dir)
    #zipdir(path, zipf)

def WindowsMain(zipf):
    pass

def MacMain(zipf):
    pass

def NameGenerator():
    t = int(time.time())
    time.sleep(.3)
    s=int(time.time() %(t%(172800+1)))
    return("TooBeZipped"+str(s)+".zip")


if __name__=="__main__":

    if options.version:
        print(VERSION)
    else:
        zipf = zipfile.ZipFile(NameGenerator(), 'w', zipfile.ZIP_DEFLATED)
        if os.name == "posix":
            LinuxMain(zipf)
        elif os.name == "nt":
            WindowsMain()
        else:
            MacMain()

        zipf.close()

"""
So the gobbler needs to be able to create files that do not share a same name and be added to either too be deleted or delete folder.
The next step is to figure out how to determine the date at which folders were added, and then delete them. also to determine whether
the folders will be in the gobbler package or apart of something else, I would guess tmp, makes the most sense.
"""
