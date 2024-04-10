# s3po 

rust s3 object file storage client with files encryption/decryption

## project state

testing, unstable

## usage

help                               - for see this help

ls                                 - list buckets

ls <bucket name>                   - list files/objects in specified <bucket name>

cd <bucket name>                   - change current bucket to specified <bucket name>

cd (cd ..)                         - return too root server folder

mkdir <bucket name>                - creates new bucket

rm < bucket name / filename >      - delete bucket if exists or file if current bucket is not root

rm <bucket name> <file name>       - delete file/objects in specified bucket

put <bucket name> <file name>      - encrypt and upload <file name> to specified <bucket name>

put <file name>                    - encrypt and upload <file name> to current bucket

get <bucket name> <file name>      - decrypt and download <file name> from specified <bucket name>

get <file name>                    - decrypt and download <file name> from current bucket

upload <bucket name> <file name>   - upload <file name> to specified <bucket name> without encryption

download <bucket name> <file name> - download <file name> from specified <bucket name> without decryption

config (config print/cat)          - prints used/current/loaded config

config list                        - lists all created configs

config folder                      - prints path to configs folder

config create (add/new)            - creates new config

config delete (del/rm) <name>      - delete the config with name

config use <name>                  - loads new config and use it to all commands

keys                               - generates new crypto keys !danger! - rewrites existing keys

q (exit/quit)                      - to exit this app
