rm -rf ./lib/
mkdir lib
gcc -c -o lib/app.o src/app.c
ar rcs lib/libapp.a lib/app.o