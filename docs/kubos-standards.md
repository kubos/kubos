# Kubos Standards

This is a doc to maintain the current naming and coding standards when working with the Kubos project.

## Them's Fightin' Words

A few of the more _controversial_ rules:

* Spaces, not tabs
* No if/for/while statements without brackets (C-specific)
* Use oxford commas
* Single space after periods and colons

## Product Names

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

## File Naming

### Code (\*.c, \*.h, scripts, etc)

- No spaces
- Use underscores to separate words
- All lowercase

### Docs (\*.md)

- No spaces
- Use hyphens to separate words
- All lowercase

### Folders

- No spaces
- Use hyphens to separate words
- All lowercase

### Special Files

The contributing, license, and readme files should all be uppercased.

'Vagrantfile', 'Makefile', 'CMake', and other similar files should all be cased to match industry standards.

## Documentation Standards

While creating clean and maintainable code is a high priority for our organization, writing successful documentation can be considered even more important. Documentation is a vital part of the user experience and, in most cases, will be a major component of a new customer's first impression of us.

Each document should be concise and well-written, and should fill some logical gap missing from the current documentation set.

### Headers

Headers should be considered the same as section titles. As a result, they should follow the same capitalization rules as titles. When in doubt, [use this tool](http://titlecapitalization.com/) to check what should be capitalized.

If you would like to include a table of contents, or would like to be able to link to a specific section, each header should also have a section label.

To include a table of contents, add "[TOC]" after your first header.

For more information, see this the 'Header Id Attributes' section of this [Doxygen doc](https://www.stack.nl/~dimitri/doxygen/manual/markdown.html#md_links).

**Note:** For...reasons...doxygen requires that you have two level one headers for each document in order to display the table of contents and headers correctly. The first header is used as the page title and the second is used as an actual level one header.

### Content

The start of each document should have an overview blurb which describes what information the doc covers.

Most of our non-code docs are [Markdown](https://www.stack.nl/~dimitri/doxygen/manual/markdown.html) files. As a result, most standard Markdown formatting features are available. 

Some items to remember:

* Single space after periods and colons 
* No one likes a giant blob of text
* Use things like bullet points, bold or italicized text, and images to breakup and highlight your content
* You can pry the oxford comma from my cold, dead hands

## Coding Standards

This section should be updated as coding standards are decided.

### C

[ClangFormat](https://clang.llvm.org/docs/ClangFormat.html) is a series of tools that can be used to automatically correct any C coding inconsistencies. You can find an example which we've used in the '.clang-format' file in the [Kubos repo](https://github.com/kubostech/kubos/blob/master/.clang-format

- Four spaces, not tabs (for consistency between OS's)
- All brackets should be on their own line
- All `if`, `while`, and `for` statements should use brackets



### Python

### Bash

## CONSISTENCY

BE CONSISTENT. I DON'T CARE IF YOU IGNORE EVERY OTHER RULE IN THIS DOC (okay, I do care, but I'm trying to make a point), JUST MAKE SURE THAT WHATEVER YOU DO, IT LOOKS AND SMELLS THE SAME AS EVERYTHING ELSE YOU DO AND/OR EVERYTHING ELSE AROUND IT.