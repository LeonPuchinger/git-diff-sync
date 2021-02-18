# git-diff-sync

sync uncommitted changes between clones of the same repository (for example between your laptop and pc) using a common sync-repository to store all the diff-files

the repository the diffs are created from/applied to is referred to as **local**. the repository used for exchanging the diffs is called **remote**.

the remote repo should be located at `~/.git-diff-sync/git-sync` (for now). You will have to create that one by yourself. Just create a new repository, add a remote and maybe add a first commit with maybe a readme to avoid empty repository issues.

## Usage:

git-diff-sync \<mode\> \<path-to-local-repo\>  
modes:
* `-g` generate diff and commit to remote repository
* `-a` apply diff from remote to local repository

## Assumptions/Notes:

* you are using **ssh key authentication** with git to connect to the remote server. keys are not protected with a passphrase and found under `~/.ssh/id_rsa` and `~/.ssh/id_rsa.pub` respectively
* it is intended that all the clones are on the same commit and you don't have any other changes that will conflict with the diff when it is applied. maybe try to stash all the other changes before applying the diff
* if the remote repository does not have a remote, a warning will be displayed and the local copy will be used (for example if you just want to sync between clones on the same machine)
* if the current branch of local copy of the remote repo can't be found (local copy is an empty git repository for example), the remote branch `main` is assumed
* the remote repo can technically use whatever branch name you want, just make sure that the local copy of remote has at least one commit on that branch (a simple readme for example, this can probably prevent some bugs with an empty git repo too) and that branch is checked-out
* commits in the remote repository are of the form `<local-name>_<date_time>`. Each local-repository gets a folder with its name in the remote repo and each branch gets a diff-file with its name in that folder

i am not responsible if this tool messes up your repository/causes any data loss. if it does though, make sure to check past commits in the remote repo and try to recover from the old patch-files.