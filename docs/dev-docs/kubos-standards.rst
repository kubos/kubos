Kubos Coding and Documentation Standards
========================================


This is a doc to maintain the current naming and coding standards when
working with the Kubos products.

Product Names
#############

The general naming scheme for products is "Kubos {component}". Note that there is a space separating the two words.

The component should be capitalized like a normal proper noun. First
letter capitalized if the component is a word, all letters capitalized
if the component is an initialism. For example: 

- Kubos Linux
- Kubos SDK 
- Kubos CLI 

KubOS (note the capitalization) refers to the entire system, not a specific component. 

Component Names
###############

Components within Kubos breakdown as follows:

- API - a statically linked library
- Service - A persistent process that is used to interact with the system. These typically call APIs.
- Application - Makes decisions and uses the services. 

Components added to the system should be referred to/labeled accordingly. See the :doc:`Architecture Overview <../architecture-overview>` document for more information on each component's roll. 

File Naming
###########

Code (\*.c, \*.h, scripts, etc)
-------------------------------

-  No spaces
-  Use underscores to separate words
-  All lowercase

Docs (\*.md)
------------

-  No spaces
-  Use hyphens to separate words
-  All lowercase

Folders
-------

-  No spaces
-  Use hyphens to separate words
-  All lowercase

Special Files
-------------

The CONTRIUBUTING, LICENSE, and README files should all be uppercased.

'Vagrantfile', 'Makefile', 'CMake', and other similar files should all
be cased to match industry standards.

Them's Fightin' Words
#####################
A few of the more *controversial* rules:

-  Spaces, not tabs
-  No if/for/while statements without brackets
-  All brackets on their own line
-  Use oxford commas
-  Single space after periods and colons

Documentation Standards
#######################

While creating clean and maintainable code is a high priority for our
organization, writing successful documentation can be considered even
more important. Documentation is a vital part of the user experience
and, in most cases, will be a major component of a new customer's first
impression of us.

Each document should be concise and well-written, and should fill some
logical gap missing from the current documentation set.

Our user docs are written using `reStructuredText <http://docutils.sourceforge.net/rst.html>`__.
This gives us a standardized way to construct and present user content.

Consider Your Audience
----------------------

When writing a new document, consider who the expected reader is.

For a doc like "Installing the Kubos SDK", we can assume that the reader
is a brand new user. Therefore, few assumptions should be made about
their current knowledge and, instead, the doc should have a great deal
more handholding. This means more explicit explanations, more links to
external resources, and more screenshots and example commands.

In contrast, with a doc like "Developing Kubos Modules" we can expect
the reader to be a) familiar with our products and b) familiar with
development in general. In this case, while handholding is still
appreciated, the content can be more brief. Essential components are
still fleshed out, but peripheral knowledge can be mentioned by simply
linking to a relevant resource.

Headers
-------

Headers should be considered the same as section titles. As a result,
they should follow the same capitalization rules as titles. When in
doubt, `use this tool <http://titlecapitalization.com/>`__ to check what
should be capitalized.

Content
-------

The start of each document (and preferrably each high-level section)
should have an overview blurb which describes what information the doc
covers.

Some items to remember:

-  Single space after periods and colons
-  All commands which are not in a code block should be encased in
   backticks (\`command\`), rather than single or double quotes.
-  Use things like bullet points, bold or italicized text, and images to
   break up and highlight your content (no one likes a giant blob of
   text)
-  Oxford commas should **always** be used

Coding Standards
################

This section covers the styling and standards for the various languages
and tools that we use.

C Standards
-----------

`ClangFormat <https://clang.llvm.org/docs/ClangFormat.html>`__ is a
series of tools that can be used to automatically correct any C coding
inconsistencies. A stand-alone tool is available, which can then be used
with a variety of IDEs. We have created a '.clang-format' file in the
`Kubos
repo <https://github.com/kubos/kubos/blob/master/.clang-format>`__
which can be used to automatically correct C code files to conform with
our styling.

-  `Clang-format with Eclipse <https://github.com/wangzw/CppStyle>`__
-  `Clang-format with Atom <https://atom.io/packages/clang-format>`__
-  `Clang-format with Visual
   Studio <https://marketplace.visualstudio.com/items?itemName=xaver.clang-format>`__

*The following subsections are based on a doc generated 2017-04-18 by
Coding Standard Generator version 1.13.*

Files
~~~~~

Each file must start with a copyright notice.

Header files must have a ``#pragma once`` statement. This causes the
file to be included only once. If for some reason you encounter a
scenario where the pragma statements are not supported, use include
guards instead. The name used in the include guard should be the same
name as the file (excluding the extension) followed by the suffix "\_H".

Example:

::

    #pragma once

    OR

    #ifndef FILE_H
    #define FILE_H
    ...
    #endif

System header files should be included with <> and project headers with
"".

Put all #include directives at the top of files. Having all #include
directives in one place makes it easy to find them. Do not use absolute
directory names in #include directives.

Put all #define statements immediately after any #include statements.

Put all function prototypes after any #define statements.

Comments
~~~~~~~~

All functions should be fully documented in the header file that they
belong to. Use the '/\*\* ... \*/' comment style so that Doxygen can add
the function to the generated API documenation.

The function comment block should include: - A brief description of the
function - The name, type, and purpose of all input variables - The
name, type, and purpose of the returned value/s

For example:

::

    /**
     * Read data over i2c bus from specified address
     *
     * In order to ensure safe i2c sharing, this function is semaphore locked.
     * There is one semaphore per bus. This function will block indefinitely
     * while waiting for the semaphore.
     *
     * @param i2c i2c bus to read from
     * @param addr address of target i2c device
     * @param ptr pointer to data buffer
     * @param len length of data to read
     * @return int I2C_OK on success, I2C_ERROR on error
     */
    KI2CStatus k_i2c_read(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len);

For large and/or complex functions, it is helpful to also include the
function comment block just above where the function is actually
implemented. This way the developer can quickly review what the function
is and how it's supposed to work. The regular '/\* ... \*/' comment
styling is acceptable in this case.

All code comments should be placed above the line the comment describes,
indented identically, as opposed to allowing in-line comments.

::

    /* comment here */
    call_function(do, stuff) /* instead of here */

Code comments should cover the 'what' and 'why' of the following code,
rather than the 'how'.

Use #ifdef instead of /\* ... \*/ to comment out blocks of code. The
code that is commented out may already contain comments which then
terminate the block comment and causes lots of compile errors or other
harder to find errors.

**However**, code should not be left permanently commented out; "#ifdef
0" is fine when creating and testing code, but has no place in the final
product. Make sure to remove all dead code before merging changes into
the master branch.

**Do not leave comments like 'TODO' or 'FIX ME' in your final code
changes unless absolutely necessary.** Just do whatever it is that
you're trying to procrastinate on. If you must leave a to-do, THERE
BETTER BE A STORY FOR IT IN JIRA AND IT BETTER BE AT THE TOP OF THE
BACKLOG. "Oh, I'll create a story for it later". NO, YOU WON'T. DO IT
NOW.

Names
~~~~~

-  Constants, enumerators, and macros should be all upper case.
-  All other names should be all lower case.
-  Words should be separated by underscore (\_).

Use sensible, descriptive names. Do not use short cryptic names or names
based on internal jokes. It should be easy to type a name without
looking up how it is spelt. Exception: Scratch variables used for
temporary storage or indices are best kept short. A programmer reading
such variables should be able to assume that its value is not used
outside a few lines of code. Common scratch variables for integers are
i, j, k, m, n and for characters c and d.

Use name prefixes for identifiers declared in different modules. For
example, 'csp\_buffer\_free' indicates that the function belongs to the
CSP directory.

Indentation and Spacing
~~~~~~~~~~~~~~~~~~~~~~~

**Do not use tabs. Instead, use 4 spaces.** Kubos developers and
contributors use a variety of operating systems and development
environments. Using spaces ensures that multiple people can contribute
to the same file and all indentions will remain the same width,
improving readability and cohesion.

Braces should follow "Exdented Style".

The Exdented Bracing Style means that the curly brace pair are lined up
with the surrounding statement. Statements and declarations between the
braces are indented relative to the braces. Braces should be indented 4
columns to the right of the starting position of the enclosing statement
or declaration.

Example:

::

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

Loop and conditional statements (``if``, ``for``, ``while``) should
always have brace enclosed sub-statements. The code looks more
consistent if all conditional and loop statements have braces. Even if
there is only a single statement after the condition or loop statement
today, there might be a need for more code in the future.

Braces without any content may be placed on the same line.

::

    while (...) {//do nothing};

Each statement should be placed on its own line. There is no need to
make code compact. Putting several statements on the same line only
makes the code cryptic to read.

Declare each variable in a separate declaration. This makes it easier to
see all variables. It also avoids the problem of knowing which variables
are pointers. int\* p, i; It is easy to forget that the star belongs to
the declared name, not the type, and look at this and say that the type
is "pointer to int" and both p and i are declared to this type.

All binary arithmetic, bitwise and assignment operators and the ternary
conditional operator (?:) should be surrounded by spaces; the comma
operator should be followed by a space but not preceded. **Exception:**
No spaces around pre/postfix increment and decrement operators ('++',
'--').

Loop and conditional statements should have a single space preceding the
condition in parenthesis.

::

    if (condition) /* correct */
    if(condition)  /* wrong */

Lines should not exceed 78 characters. Even if your editor handles long
lines, other people may have set up their editors differently. Long
lines in the code may also cause problems for other programs and
printers.

Declarations
~~~~~~~~~~~~

Provide names of parameters in function declarations. Parameter names
are useful to document what the parameter is used for. The parameter
names should be the same in all declarations and definitions of the
function.

Always provide the return type explicitly.

Use a typedef to define a pointer to a function. Pointers to functions
have a strange syntax. The code becomes much clearer if you use a
typedef for the pointer to function type. This typedef name can then be
used to declare variables etc.

::

    double sin(double arg);
    typedef double (*trig_func)(double arg);

    /* Usage examples */
    trig_func my_func = sin;
    void call_func(trig_func callback);
    trig_func func_table[10];

If not previously defined in a header file, declare variables as close
to the first use as is useful. This is opposed to the old C requirement
where all variables in a function needed to be declared before all
instruction lines.

::

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

Statements
~~~~~~~~~~

Never use gotos.

All switch statements should have a ``default`` label. Even if there is
no action for the default label, it should be included to show that the
programmer has considered values not covered by case labels. It is
normally useful to place an error message in the default label in this
case.

Return Values
~~~~~~~~~~~~~

In most cases, it is preferable to return an error code, rather than a
value. If an output value is desired, a pointer to the desired storage
area should be added to the function's arguments. This allows us to be
consistent in our declarations.

::

    int length;
    int ret;

    /* Don't do this */
    length = get_length(input);
    /* Do this instead */
    ret = get_length(input, length);

Other Typographical Issues
~~~~~~~~~~~~~~~~~~~~~~~~~~

Avoid macros; most macros can be replaced by constants, enumerations or
inline functions. Using macros can lead to decreased readability and
increased chance of bugs.

Do not use literal numbers other than 0 and 1. Use constants instead of
literal numbers to make the code consistent and easy to maintain. The
name of the constant is also used to document the purpose of the number.

Do not rely on implicit conversion to bool in conditions.

::

    if (ptr)         // wrong
    if (ptr != NULL) // ok

Python Standards
----------------

`Python's PEP8 Style
Guide <https://www.python.org/dev/peps/pep-0008/>`__ is our preferred
Python styling.

`PyLint <https://pylint.readthedocs.io/en/latest/>`__ is a great tool
which can be used to check the style and validity of your python files.
It has support for a variety of `editors and
IDEs <https://pylint.readthedocs.io/en/latest/user_guide/ide-integration.html>`__.

-  `PyLint via PyDev for
   Eclipse <http://www.pydev.org/manual_adv_pylint.html>`__
-  `PyLint for Atom <https://atom.io/packages/linter-pylint>`__
-  `PyLint for Visual
   Studio <https://www.mantidproject.org/How_to_run_Pylint>`__

`Autopep8 <https://pypi.python.org/pypi/autopep8>`__ can be used to
automatically format your code to conform with the Python PEP8 standard.

-  `Autopep8 via PyDev for
   Eclipse <https://marketplace.eclipse.org/content/pydev-python-ide-eclipse>`__
-  `Autopep8 for Atom <https://atom.io/packages/python-autopep8>`__
-  `Autopep8 for Visual
   Studio <https://marketplace.visualstudio.com/items?itemName=himanoa.Python-autopep8>`__

Working with External Projects
------------------------------

Some of the Kubos code uses or extends external projects. In this case
where you are adding a new file, use the Kubos standards. If you are
modifying an existing file, try to match the formatting of the
surrounding code.

Linux Kernel
~~~~~~~~~~~~

`Linux kernel coding
style <https://01.org/linuxgraphics/gfx-docs/drm/process/coding-style.html>`__

Notably:

-  8 space indentation
-  Torvalds disagrees with us on basically everything

U-Boot
~~~~~~

`U-Boot coding style <http://www.denx.de/wiki/U-Boot/CodingStyle>`__

Notably:

-  Mostly follows the Linux coding style
-  Tabs, not spaces
-  No C++ style comments (use /\* \*/, not //)

Other Languages
---------------

Bash - Refer to `Google's style
guide <https://google.github.io/styleguide/shell.xml>`__. **Exception:**
Use 4 spaces, since that's what we do in all of our other languages.

`KConfig <https://www.kernel.org/doc/Documentation/kbuild/kconfig-language.txt>`__

CONSISTENCY
###########

BE CONSISTENT. I DON'T CARE IF YOU IGNORE EVERY OTHER RULE IN THIS DOC
(okay, I do care, but I'm trying to make a point), JUST MAKE SURE THAT
WHATEVER YOU DO, IT LOOKS AND SMELLS THE SAME AS EVERYTHING ELSE YOU DO
AND/OR EVERYTHING ELSE AROUND IT.
