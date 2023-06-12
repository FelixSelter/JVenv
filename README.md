# JVenv

This project unites the efforts to combine https://github.com/jenv/jenv and https://github.com/FelixSelter/JEnv-for-Windows/ to a cross platform solution. The readme of this project which is in a very early stage of development should temporarily function as a design document. Please contribute by expanding it or implementing ideas from it.

The main problem creating an application like JEnv is that changing the environment variables of the current shell session can only be done by a script of that specific shell. Therefore development of these tools lead naturally to no cross platform solution.

The idea of this tool is to have a main codebase that handles common tasks as jdk installation, discovery and keeping track of the global java version. Additionally there will be a collection of small shell specific scripts that query the main cross platform code and change the necessary environment variables. These script should be kept as minimal as possible. Creating as many scripts for all the different shells out there is a key feature. Please expand the list of shells that need to be supported below. Once sections of this document grow to large they will be moved into the wiki submodule.

The repository currently contains very basic rust code that allows for a cross platform config file and type safe command line argument parsing. The language for this tool is still up to debate. Basic requirements for a language to write this tool in are static type safety, cross platform support and that it does not require an additional runtime to be installed (e.g. Python)

### List of supported shells

### List of shells that need to be supported

- bash
- powershell
- fish

### List of cli commands and their function
