# Test Server

A simple Go server and shell script to test web builds.

Running the associated script should copy your output files to the
static folder and host the application at
[http://localhost:8081/](http://localhost:8081/).

Testing debug build:
```console
./debug_server.sh
```
Testing release build:
```console
./release_server.sh
```
