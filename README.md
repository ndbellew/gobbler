# Gobbler

The gobbler is a file deletion program, a  single command line program that takes, starting location and files/folders. each file/folder is stored in a zip and either deleted or put into a "Too be deleted" folder. Wherein after 48 hours the folder will delete all of its contents. Another folder will Hold all of the files deletd in a small make-shift logging system. 

## Setup
Currently this is only working for linux users, if you run the Setup bash command it should complete everything that is needed. then just follow the usage. 



## Usage

<p>Usage: gobbler [ -s|--start ][options] [files]<p>

Options:

  -h, --help            show this help message and exit
  
  -s START, --s=START   Select starting location for gobbler to begin gobbling. ex. /home/usr/Documents
  
  -F, --folder          If any of the files chosen are/are also folders, then
                        they will be included.
                        
  -n NAME, --name=NAME  Name Zip folder
