#Kubos Standards

This is a doc to maintain the current naming and coding standards when working with the Kubos project.

##Product Names

The general naming scheme is "Kub[OS|os] {component}".  Note that there is a space separating the two words.

If the component is an OS, then use the capitalized "OS".  If not, then use "os".

The component should be capitalized like a normal proper noun.  First letter capitalized if the component is
a word, all letters capitalized if the component is an initialism.

- Kubos SDK
- Kubos CLI
- KubOS RT
- KubOS Linux
- Kubos Portal
- Kubos Core

##File Naming

###Code (\*.c, \*.h, scripts, etc)

- No spaces
- Use underscores to separate words
- All lowercase

###Docs (\*.md)

- No spaces
- Use hyphens to separate words
- All lowercase

###Folders

- No spaces
- Use hyphens to separate words
- All lowercase

###Special Files

The contributing, license, and readme files should all be uppercased.

'Vagrantfile', 'Makefile', 'CMake', and other similar files should all be cased to match industry standards.

##Coding Standards

This section should be updated as coding standards are decided.

###C

- Four spaces, not tabs (for consistency between OS's)
- All brackets should be on their own line
- All `if` statements should use brackets