Kubos Contribution Process
==========================

This is the workflow you should go through in order to create and
complete a task to contribute to the KubOS project

Sign the CLA
------------

All contributors must sign the Kubos Contributor License Agreement
before their changes can be merged into the main codebase.

The Kubos CLA can be found
`here <https://cla-assistant.io/kubos/kubos>`__.

Create or Select an Issue
-------------------------

GitHub issues allow us to track what problems have been found and which
problems are currently being addressed.

If an issue does not already exist for the work you would like to contribute, please open one using
the process layed out in the following docs:

- :doc:`reporting-bugs`
- :doc:`feature-request`

The issue will be automatically labeled with "bug" or "enhancement" depending on the issue template
you use.

Mark the Issue as 'In Progress'
-------------------------------

In order to track what's being worked on and by whom, for every issue you work you should:

- Click the 'Assignees' link and select yourself
- Mark the issue as 'In Progress' by editing the title of the project and adding "[WIP]"

Create a Branch of the Code You Want to Work On
-----------------------------------------------

All code changes should initially be made in a branch of the relavent
Kubos repo either in the main repo, or in a personal copy (if you don't
have access) and then be submitted against the master branch as a pull
request:

To create your own repo:

-  Navigate to the GitHub page of the main code that you want to work
   on. For example, https://github.com/kubos/kubos.
-  Click the 'Fork' button in the upper right-hand corner.
-  If you see a dialog 'Where should we fork this repository?', click
   the icon with your username.
-  Within your development environment, create a link to your new remote
   repository:

   $ git remote add {remote name you create} {personal repo url}

Clone the repo that you want to modify onto your local machine

::

    $ git clone http://github.com/kubos/kubos

Move into the repo folder

::

    $ cd kubos

Create a local branch to make your changes

::

    $ git checkout -b {local branch name you create}

Make Your Changes
-----------------

The code. Write it. Test it.

The :doc:`Kubos Standards <kubos-standards>` doc has some basic
naming and coding standards that should be adhered to. When in doubt,
try to match the styling of the surrounding code.

Update any documentation areas that are affected by your changes.

- If you want to test your documentation changes:

  - Run the command 'doxygen Doxyfile' from the project folder with the new documentation to regenerate the html documentation
  - Open up {project}/html/index.html in a web browser
  - Browse to your doc updates to verify

Add or update any unit tests that are affected by your changes. For
instance, if support for SPI is added for the Kubos Linux HAL,
then a new test folder would be added to `hal/kubos-hal/test` with the
relavent new test cases.

Commit your changes and push to your remote branch (the branch will be
created automatically if it doesn't exist):

::

    $ git add {files you changed}
    $ git commit -m "Descriptive message about the changes you made"
    $ git push {remote name} {local branch name}

If you're committing against a Kubos repo, then the remote name will
likely be "origin". If you're committing against your personal fork,
then the remote name will match what you specified in the
``git remote add`` command.

Commit early, commit often.

Create a Pull Request
---------------------

At some point, you'll want to create a pull request so that your changes
can be merged into the main repo's master branch. You will need to
create a pull request for each repository you are making changes to.

From the GitHub page for the repository that contains the changes you
want to merge:

- Click the 'Branch:' dropdown on the left-hand side and select the local branch containing your changes
- Click the 'New pull request' button
- The title of the pull request should clearly refer to its corresponding issue
- In the description field, add a small summary of the changes you made. The title should have indicated the bulk of the changes you made, but it's also good to mention things like documentation updates and any miscellaneous changes that were made (for example, fixing any bugs that you ran into while working on your code changes).
- Click 'Create pull request'

If you'd like specific people to review your code, you can either
mention them in the description with an ``@{name}`` tag, or by adding them
to the 'Reviewers' list.

You are welcome to create a pull request before your changes are entirely
complete. Creating a pull request early in the code-creation process
allows others to see what changes are being made and answer questions or
offer architectural suggestions. If you do create a pull request before
you are done making changes, add "[WIP]" to the pull request's title.
Remove the "[WIP]" once all code changes have been completed and the PR
is officially ready for review.

Merge in New Changes From Master
--------------------------------

After submitting your pull request, you may find that GitHub has flagged
one or more files as being in conflict with the current version of the
file in the master branch. This means that someone else has committed
code in the same file and similar area as you and your changes can't be
automatically merged.

In order to resolve the conflict, execute the following steps within
your development environment:

Merge the master branch into your local branch

::

    $ git checkout origin/master
    $ git pull origin master
    $ git checkout {local branch where your changes are}
    $ git merge origin/master

Git will edit any files with conflicts. Conflicts will look like this:

::

        >>>Head
            New local changes
        ==========
            New master changes
        <<<kubos
        

Edit the files to resolve the conflicts. Push the resolved changed to
your remote repo

::

    $ git add {fixed files}
    $ git commit
    $ git push {remote name} {local branch name}

If you navigate to your pull request, you should now see that github
says "This branch has no conflicts with the base branch", indicating
that the changes okay to merge (pending pull request approval).

Verify CI Tests Pass
--------------------

When you create a PR, our CI tests will automatically be run against your new code.
The results of these tests are displayed at the bottom of the PR page.

If all tests have passed, you will see a green checkmark icon and "All checks have passed".

If a test failed, then you will see "Some checks were not successful".
The test/s which failed will be marked with a red 'X'.
If you click on the "Details" link for a test, it will take you to the particular CI test that
failed.
From there, you can review the test output and determine what needs to be fixed.

If you're having a difficult time parsing the test output, or if something fails which appears to be
unrelated to your changes, please feel free to contact a Kubos team member via
`Slack <https://slack.kubos.co/>`__ for support.

All tests must pass before your PR can be approved.

Wait for Pull Request Approval
------------------------------

Once your pull request has been submitted, it must be approved by at
least one person before the request can be merged into the master
branch.

In all likelyhood, you'll need to make changes to your code before your
pull request can be merged. Make the changes in your local development
environment and then commit and push them into your remote repo. As long
as you're still using the same local branch, these new changes will be
automatically added to your existing pull request.

Once all changes have been approved, a Kubos engineer will merge the changes
into the master branch.

Close the Issue
---------------

Before you close the issue, verify the following:

- All features listed in the issue have been completed
- All relevant documentation changes have been made
- All relevant unit tests have been created or updated
- All code changes and related code have been tested
- All pull requests related to the issue have been approved and merged
