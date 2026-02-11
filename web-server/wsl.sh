# /bin/bash -e
# wsl.sh
# Script to install all needed package

# root asking
if [ $(whoami) != "root" ];
then
    echo "Usage: sudo $0"
    exit 1
fi

apt-get install libssl-dev pkg-config clang libclang1 -y  
