# Gobbler
# Author: Nathan Bellew
# Tool that takes in file/folder names, zips them in one temp folder,
# and sends them to trash.

import os
import optparse import OptionParser

usage = "Usage: %prog [options] [files]"
parser = OptionParser(usage)
parser.add_option('-F','--folder', dest='folder',help='If any of the files chosen are/are also folders, then they will be included.')
parser.add_option('-n', '--name', dest='name',help='Name Zip folder')
def LinuxMain():
    pass

def WindowsMain():
    pass

def MacMain():
    pass

if __name__=="__main__":

    if os.name == "posix":
        LinuxMain()
    elif os.name == "nt":
        WindowsMain()
    else:
        MacMain()
