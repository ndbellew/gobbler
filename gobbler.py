# Gobbler
# Author: Nathan Bellew
# Tool that takes in file/folder names, zips them in one temp folder,
# and sends them to trash.

import sys
import os
from optparse import OptionParser
import zipfile
from shutil import rmtree


usage = "Usage: %prog [ -s|--start ][options] [files]"
parser = OptionParser(usage)
parser.add_option('-s','--s',dest='start', help='Select starting location for gobbler to begin gobbling. ex. /home/usr/Documents')
parser.add_option('-F','--folder', action='store_true',dest='folder',help='If any of the files chosen are/are also folders, then they will be included.')
parser.add_option('-n', '--name', dest='name',help='Name Zip folder')
(options, args)= parser.parse_args()

if len(args) < 1 or options.start is None:
    parser.print_help()
    sys.exit(0)

def zipdir(path, ziph):
    for root, dirs, files in os.walk(path):
        for file in files:
            ziph.write(os.path.join(root, file))

def LinuxMain(zipf):
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

if __name__=="__main__":
    zipf = zipfile.ZipFile('TooBeGobbled.zip', 'w', zipfile.ZIP_DEFLATED)

    if os.name == "posix":
        LinuxMain(zipf)
    elif os.name == "nt":
        WindowsMain()
    else:
        MacMain()

    zipf.close()
