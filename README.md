# JVenv

This project unites the efforts to combine https://github.com/jenv/jenv and https://github.com/FelixSelter/JEnv-for-Windows/ to a cross platform solution. The readme of this project which is in a very early stage of development should temporarily function as a design document. Please contribute by expanding it or implementing ideas from it. Everything in this document is temporary and may be changed by you. Once sections of this document grow to large they will be moved into the wiki submodule.

The main problem creating an application like JEnv is that changing the environment variables of the current shell session can only be done by a script of that specific shell. Therefore development of these tools lead naturally to no cross platform solution.

The idea of this tool is to have a main codebase that handles common tasks as jdk installation, discovery and keeping track of the global java version. Additionally there will be a collection of small shell specific scripts that query the main cross platform code and change the necessary environment variables. These script should be kept as minimal as possible. Creating as many scripts for all the different shells out there is a key feature. Please expand the list of shells that need to be supported below.

The repository currently contains very basic rust code that allows for a cross platform config file and type safe command line argument parsing. The language for this tool is still up to debate. Basic requirements for a language to write this tool in are static type safety, cross platform support and that it does not require an additional runtime to be installed (e.g. Python)

### List of supported shells

### List of shells that need to be supported

- bash
- powershell
- fish

### List of cli commands and their function

##### java-home

- Returns the java home for a specific directory so the shell script can update the environment variables when it is activated

##### auto-scan

- Tries to find installed jvms on the system to add them to the config file

##### list

- lists all the jvms with their alias name from the config. Shows the current active global version

##### install

- install a specific jdk (disco api?)
- sdkman support?

##### uninstall

- uninstall a specific jdk

##### integrate

- automatically configure a shell to work with the jvenv script

##### Needs name

- Change the global java version

##### Needs name

- Change the java version of the current shell

##### Needs name

- Create a project by creating a .java-version file with the given java version

##### add

- Add a jdk to the config and create an alias

##### remove

- Remove a jdk from the config

##### something to uninstall this tool
