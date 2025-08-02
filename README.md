# tc-emulator

The emulator is an attempt to emulate a target provider locally.


To emulate a function

```sh
cd function-dir
tc-emulator
```

The following command spins up a docker container with the defined layers in function.yml, sets up the paths, environment variables, AWS access, local code and runtime parameters (mem, handlers etc)

To run it as a shell (bash)

```
tc-emulator --shell
```

You can now invoke a payload locally with this emulator

```
tc invoke --local [--payload <payload.json | json-str>]
```
