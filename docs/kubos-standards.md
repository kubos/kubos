# Kubos Standards

[TOC]

# Kubos Coding and Documentation Standards {#kubos-standards}

This is a doc to maintain the current naming and coding standards when working with the Kubos products.

# Them's Fightin' Words {#fightin-words}

A few of the more _controversial_ rules:

* Spaces, not tabs
* No if/for/while statements without brackets
* All brackets on their own line
* Use oxford commas
* Single space after periods and colons

# Product Names {#product-names}

The general naming scheme is "Kub[OS|os] {component}". Note that there is a space separating the two words.

If the component is an operating system, then use the capitalized "OS". If not, then use "os". 

At the moment this means specifically:
- KubOS RT
- KubOS Linux 

The component should be capitalized like a normal proper noun. First letter capitalized if the component is
a word, all letters capitalized if the component is an initialism.

For example:
- Kubos SDK
- Kubos CLI
- Kubos Portal
- Kubos Core

# File Naming {#file-naming}

## Code (\*.c, \*.h, scripts, etc) {#code}

- No spaces
- Use underscores to separate words
- All lowercase

## Docs (\*.md) {#docs}

- No spaces
- Use hyphens to separate words
- All lowercase

## Folders {#folders}

- No spaces
- Use hyphens to separate words
- All lowercase

## Special Files {#special-files}

The CONTRIUBUTING, LICENSE, and README files should all be uppercased.

'Vagrantfile', 'Makefile', 'CMake', and other similar files should all be cased to match industry standards.

# Documentation Standards {#doc-standards}

While creating clean and maintainable code is a high priority for our organization, writing successful documentation can be considered even more important. Documentation is a vital part of the user experience and, in most cases, will be a major component of a new customer's first impression of us.

Each document should be concise and well-written, and should fill some logical gap missing from the current documentation set.

## Consider Your Audience {#doc-audience}

When writing a new document, consider who the expected reader is. 

For a doc like "Installing the Kubos SDK", we can assume that the reader is a brand new user. 
Therefore, few assumptions should be made about their current knowledge and, instead, the doc should have a great deal more handholding. 
This means more explicit explanations, more links to external resources, and more screenshots and example commands.

In contrast, with a doc like "Developing Kubos Modules" we can expect the reader to be a) familiar with our products and b) familiar with development in general. In this case, while handholding is still appreciated, the content can be more brief. Essential components are still fleshed out, but peripheral knowledge can be mentioned by simply linking to a relevant resource.

## Headers {#doc-headers}

Headers should be considered the same as section titles. As a result, they should follow the same capitalization rules as titles. When in doubt, [use this tool](http://titlecapitalization.com/) to check what should be capitalized.

If you would like to include a table of contents, or would like to be able to link to a specific section, each header should also have a section label.

To include a table of contents, add "[TOC]" after your first header.

For more information, see this the 'Header Id Attributes' section of this [Doxygen doc](https://www.stack.nl/~dimitri/doxygen/manual/markdown.html#md_links).

**Note:** For...reasons...doxygen requires that you have two level one headers for each document in order to display the table of contents and headers correctly. The first header is used as the page title and the second is used as an actual level one header.

## Content {#doc-content}

The start of each document (and preferrably each high-level section) should have an overview blurb which describes what information the doc covers.

Most of our non-code docs are [Markdown](https://www.stack.nl/~dimitri/doxygen/manual/markdown.html) files. As a result, most standard Markdown formatting features are available. 

Some items to remember:

* Single space after periods and colons 
* All commands which are not in a code block should be encased in backticks (\`command\`), rather than single or double quotes.
* Use things like bullet points, bold or italicized text, and images to breakup and highlight your content (no one likes a giant blob of text)
* Oxford commas should **always** be used

# Coding Standards {#coding-standards}

This section covers the styling and standards for the various languages and tools that we use.

## C {#c-standards}

[ClangFormat](https://clang.llvm.org/docs/ClangFormat.html) is a series of tools that can be used to automatically correct any C coding inconsistencies. You can find an example which we've used in the '.clang-format' file in the [Kubos repo](https://github.com/kubostech/kubos/blob/master/.clang-format)

*The following subsections are based on a doc generated 2017-04-18 by Coding Standard Generator version 1.13.*

### Names {#c-names}

- Constants, enumerators, and macros should be all upper case.
- All other names should be all lower case.
- Words should be separated by underscore (_).

Use sensible, descriptive names.
Do not use short cryptic names or names based on internal jokes. It should be easy to type a name without looking up how it is spelt.
Exception: Scratch variables used for temporary storage or indices are best kept short. A programmer reading such variables should be able to assume that its value is not used outside a few lines of code. Common scratch variables for integers are i, j, k, m, n and for characters c and d.

Use name prefixes for identifiers declared in different modules.
For example, 'csp\_buffer\_free' indicates that the function belongs to the CSP directory.

### Indentation and Spacing {#c-spacing}

**Do not use tabs. Instead, use 4 spaces.** Kubos developers and contributors use a variety of operating systems and development environments. Using spaces ensures that multiple people can contribute to the same file and all indentions will remain the same width, improving readability and cohesion.

Braces should follow "Exdented Style".

The Exdented Bracing Style means that the curly brace pair are lined up with the surrounding statement. Statements and declarations between the braces are indented relative to the braces.
Braces should be indented 4 columns to the right of the starting position of the enclosing statement or declaration.

Example:

    void f(int a)
    {
        int i;
        if (a > 0)
        {
            i = a;
        }
        else
        {
            i = a;
        }
    }
    
Loop and conditional statements (`if`, `for`, `while`) should always have brace enclosed sub-statements. The code looks more consistent if all conditional and loop statements have braces. Even if there is only a single statement after the condition or loop statement today, there might be a need for more code in the future.

Braces without any content may be placed on the same line.

    while (...) {//do nothing};

Each statement should be placed on its own line. There is no need to make code compact. Putting several statements on the same line only makes the code cryptic to read.

Declare each variable in a separate declaration.
This makes it easier to see all variables. It also avoids the problem of knowing which variables are pointers.
int* p, i;
It is easy to forget that the star belongs to the declared name, not the type, and look at this and say that the type is "pointer to int" and both p and i are declared to this type.

All binary arithmetic, bitwise and assignment operators and the ternary conditional operator (?:) should be surrounded by spaces; the comma operator should be followed by a space but not preceded. **Exception:** No spaces around pre/postfix increment and decrement operators ('++', '--').

Loop and conditional statements should have a single space preceding the condition in parenthesis.

    if (condition) /* correct */
    if(condition)  /* wrong */

Lines should not exceed 78 characters.
Even if your editor handles long lines, other people may have set up their editors differently. Long lines in the code may also cause problems for other programs and printers.

### Comments {#c-comments}

All functions should be fully documented in the header file that they belong to. Use the '/** ... */' comment style so that Doxygen can add the function to the generated API documenation. 

The function comment block should include:
- The function name
- A brief description of the function
- The name, type, and purpose of all input variables
- The name, type, and purpose of the returned value/s

All code comments should be placed above the line the comment describes, indented identically, as opposed to allowing in-line comments.

    /* comment here */
    call_function(do, stuff) /* instead of here */
    
Code comments should cover the 'what' and 'why' of the following code, rather than the 'how'.
    
Use #ifdef instead of /* ... */ to comment out blocks of code.
The code that is commented out may already contain comments which then terminate the block comment and causes lots of compile errors or other harder to find errors. 

**However**, code should not be left permanently commented out; "#ifdef 0" is fine when creating and testing code, but has no place in the final product. Make sure to remove all dead code before merging changes into the master branch.

**Do not leave comments like 'TODO' or 'FIX ME' in your final code changes unless absolutely necessary.** Just do whatever it is that you're trying to procrastinate on. If you must leave a to-do, THERE BETTER BE A STORY FOR IT IN JIRA AND IT BETTER BE AT THE TOP OF THE BACKLOG. "Oh, I'll create a story for it later". NO, YOU WON'T. DO IT NOW.

### Files {#c-files}

Each file must start with a copyright notice.

Header files must have a `#pragma once` statement. This causes the file to be included only once. 
If for some reason you encounter a scenario where the pragma statements are not supported, use include guards instead.
The name used in the include guard should be the same name as the file (excluding the extension) followed by the suffix "_H".

Example:

    #pragma once
    
    OR

    #ifndef FILE_H
    #define FILE_H
    ...
    #endif
    
System header files should be included with <> and project headers with "".

Put all #include directives at the top of files. Having all #include directives in one place makes it easy to find them.
Do not use absolute directory names in #include directives.

Put all #define statements immediately after any #include statements.

Put all function prototypes after any #define statements.

### Declarations {#c-declarations}

Provide names of parameters in function declarations.
Parameter names are useful to document what the parameter is used for.
The parameter names should be the same in all declarations and definitions of the function.

Always provide the return type explicitly.

Use a typedef to define a pointer to a function. Pointers to functions have a strange syntax. The code becomes much clearer if you use a typedef for the pointer to function type. This typedef name can then be used to declare variables etc.

    double sin(double arg);
    typedef double (*trig_func)(double arg);
    
    /* Usage examples */
    trig_func my_func = sin;
    void call_func(trig_func callback);
    trig_func func_table[10];
    
If not previously defined in a header file, declare variables as close to the first use as is useful. This is opposed to the old C requirement where all variables in a function needed to be declared before all instruction lines.

    int doing_stuff(int parameter)
    {
    
        /* declaring 'ret' here since it's needed for both cases */
        int ret = ALL_OK;
    
        if (condition)
        {
            /* declaring 'val' here, since it's only used in this one case */
            int val = doing_things();
            ret = doing_things_with_val(val);
        }
        else
        {
            ret = ERROR_CODE;
        }
        
        return ret;
    }

### Statements {#c-statements}

Never use gotos.

All switch statements should have a `default` label. Even if there is no action for the default label, it should be included to show that the programmer has considered values not covered by case labels. It is normally useful to place an error message in the default label in this case.

### Return Values {#c-return-values}

In most cases, it is preferable to return an error code, rather than a value. If an output value is desired, a pointer to the desired storage area should be added to the function's arguments. This allows us to be consistent in our declarations.

    int length;
    int ret;
    
    /* Don't do this */
    length = get_length(input);
    /* Do this instead */
    ret = get_length(input, length);

### Other Typographical Issues {#c-other}

Avoid macros; most macros can be replaced by constants, enumerations or inline functions. Using macros can lead to decreased readability and increased chance of bugs.

Do not use literal numbers other than 0 and 1. Use constants instead of literal numbers to make the code consistent and easy to maintain. The name of the constant is also used to document the purpose of the number.

Do not rely on implicit conversion to bool in conditions.

    if (ptr)         // wrong
    if (ptr != NULL) // ok

## Python {#python-standards}

[Pylint](https://pylint.readthedocs.io/en/latest/) is a great tool which can be used to check the style and validity of your python files.
It has support for a variety of [editors and IDEs](https://pylint.readthedocs.io/en/latest/user_guide/ide-integration.html).

Refer to [Python's Python Style Guide](https://www.python.org/dev/peps/pep-0008/) for our preferred Python styling.


## Working with External Projects {#external-projects}

Some of the Kubos code uses or extends external projects. In this case where you are adding a new file, use the Kubos standards. If you are modifying an existing file, try to match the formatting of the surrounding code. 

### Linux Kernel {#linux-kernel}

[Linux kernel coding style](https://01.org/linuxgraphics/gfx-docs/drm/process/coding-style.html)

Notably:

- 8 space indentation
- Torvalds disagrees with us on basically everything

### U-Boot {#uboot}

[U-Boot coding style](http://www.denx.de/wiki/U-Boot/CodingStyle)

Notably:

- Mostly follows the Linux coding style
- Tabs, not spaces
- No C++ style comments (use /* */, not //)

## Other Languages {#other-languages}

Bash - Refer to [Google's style guide](https://google.github.io/styleguide/shell.xml). **Exception:** Use 4 spaces, since that's what we do in all of our other languages.

[KConfig](https://www.kernel.org/doc/Documentation/kbuild/kconfig-language.txt)

# CONSISTENCY {#consistency}

BE CONSISTENT. I DON'T CARE IF YOU IGNORE EVERY OTHER RULE IN THIS DOC (okay, I do care, but I'm trying to make a point), JUST MAKE SURE THAT WHATEVER YOU DO, IT LOOKS AND SMELLS THE SAME AS EVERYTHING ELSE YOU DO AND/OR EVERYTHING ELSE AROUND IT.