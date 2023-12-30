#### INSTALL.sh to install rustup, cargo, and scc

###### scc stands for 'scorch compiler' even though it's an interpreter only right now

> Note, any dependencies that are already installed will get skipped.

> 'scc' installation just adds an alias to the linux terminal for the cli tool.

``` bash
chmod +x INSTALL.sh
./INSTALL.sh
```

###### run the `scc` command with no args to start the repl. on first launch / after source changes, it will rebuild. 

###### once it's built, we can type `help` to see a list of commands. 

### creating a project

###### to create a new `.scproj` file, we can use the `create command`.

- run the `scc` command to open the repl.
- type `create` and fill in the prompted info. you must add the .scorch on the main file

###### a project file will be created in your current directory, and `scc r` or the `l` / `r` command(s) can be used to load and run the project file & associated code.


### a shortcut

###### to skip the repl and just run the project in the current directory, simply use `scc r`
