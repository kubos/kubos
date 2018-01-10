Kubos Contribution Process
==========================

This is the workflow you should go through in order to create and
complete a task to contribute to the KubOS project

Sign the CLA
------------

All contributors must sign the Kubos Contributor License Agreement
before their changes can be merged into the main codebase.

The Kubos CLA can be found
`here <https://www.clahub.com/agreements/kubos/KubOS>`__.

Create an Issue
---------------

Everything you work on should have a corresponding JIRA issue. Community
members might not have access to JIRA and should then create a Github
issue instead.

If one doesn't exist, you should create it.

If you want to create an issue but not immediately work on it, the
description field should be an in-depth summary of the problem/feature
and what needs to be changed. Ideally, you, or whoever works the issue,
should be able to read the description and understand what work needs to
be done without talking to whoever created the issue (though talking to
the creator is still recommended in order to make sure that the
requirements are well understood and haven't changed since the issue was
created).

JIRA
~~~~

-  Click the 'Create' button at the top of the JIRA page
-  Project should be 'Kubos'
-  Issue type should be 'Story' or 'Bug'

   -  A story is something to be added or updated within the project
   -  A bug is something that is broken within the project

-  The summary should be a short description of what's being changed
-  The 'Component/s' field should be updated, if possible, to list the
   related area/s affected by this issue.
-  The description should go into more detail about what the
   problem/feature is and what needs to be done in order to complete the
   task.

   -  If you are creating a story, the description should follow the
      `Agile user story
      template <https://www.mountaingoatsoftware.com/agile/user-stories>`__

-  If you're creating a bug, update the 'Affects Version/s' field to
   document the oldest affected version of the project

Github
~~~~~~

-  Navigate to the repo you'd like to open an issue against (most likely
   kubos/kubos)
-  Click the 'Issues' tab
-  Click the 'New Issue' button
-  The title should be a descriptive overview of the problem
-  The description should go into more detail about what the
   problem/feature is and what needs to be done in order to complete the
   task.

   -  If you are creating a story, the description should follow the
      `Agile user story
      template <https://www.mountaingoatsoftware.com/agile/user-stories>`__

-  Click the 'Labels' link and add a tag for 'Bug' or 'Enhancement'
   depending on what kind of work should be done

Mark the Issue as 'In Progress'
-------------------------------

In order to track what's being worked on and by whom, for every issue you work you should:
 
- JIRA: Drag the issue from the backlog into the sprint (if it's not already present) 
- Assign it to yourself 

  -  Click the issue 
  -  JIRA: Under 'People'>'Assignee', click 'Assign to me' 
  -  Github: Click the 'Assignees' link and select yourself 
  
- Mark the issue as 'In Progress'.

  -  JIRA: There are two ways 
  
    - From the full issue description page, click the 'In Progress' button 
    - From the 'Active Sprints' page, drag the issue from the 'To-Do' column into the 'In Progress' column 
     
  -  Github: Edit the title of the project and add "[WIP]"

Create a Branch of the Code You Want to Work On
-----------------------------------------------

All code changes should initially be made in a branch of the relavent
Kubos repo either in the main repo, or in a personal copy (if you don't
have access) and then be submitted against the master branch as a pull
request:

To create your own repo:

-  Navigate to the github page of the main code that you want to work
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

Update any documentation areas that are affected by your changes. For
instance, if you found that a uart configuration option was not
available for a certain board type, you would edit
kubos-hal/kubos-hal/uart.h in the appropriate comment section with a new
note about the unsupported option. 

- If you want to test your documentation changes: 

  - Run the command 'doxygen Doxyfile' from the project folder with the new documentation to regenerate the html documentation 
  - Open up {project}/html/index.html in a web browser 
  - Browse to your doc updates to verify

Add or update any unit tests that are affected by your changes. For
instance, if support for i2c slave mode is added for the STM32F4 board,
then the kubos-hal-stm32f4/test/i2c.c file's test\_i2c\_slave test case
should be updated to test the successful execution of the board in slave
mode.

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

`Commit early, commit
often <http://www.databasically.com/2011/03/14/git-commit-early-commit-often/>`__

Create a Pull Request
---------------------

At some point, you'll want to create a pull request so that your changes
can be merged into the main repo's master branch. You will need to
create a pull request for each repository you are making changes to.

From the github page for the repository that contains the changes you
want to merge: 

- Click the 'Branch:' dropdown on the left-hand side and select the local branch containing your changes 
- Click the 'New pull request' button 
- The title of the pull request should be the JIRA issue number followed by a descriptive title 

  - Ex. KUBOS-111 Adding i2c slave mode for STM32F4 

- In the description field, add a small summary of the changes you made. The title should have indicated the bulk of the changes you made, but it's also good to mention things like documentation updates and any miscellaneous changes that were made (for example, fixing any bugs that you ran into while working on your code changes). 
- Click 'Create pull request'

If you'd like specific people to review your code, you can either
mention them in the description with an ``@{name}`` tag, or by adding them
to the 'Reviewers' list.

You a welcome to create a pull request before your changes are entirely
complete. Creating a pull request early in the code-creation process
allows others to see what changes are being made and answer questions or
offer architectural suggestions. If you do create a pull request before
you are done making changes, add "[WIP]" to the pull request's title.
Remove the "[WIP]" once all code changes have been completed and the PR
is officially ready for review.

Merge in New Changes From Master
--------------------------------

After submitting your pull request, you may find that github has flagged
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

Wait for Pull Request Approval
------------------------------

Move the JIRA issue to 'Reviewing' to indicate that the work is done,
pending approval.

Once your pull request has been submitted, it must be approved by at
least one person before the request can be merged into the master
branch. Once it has been approved, you can go to your pull request page
and then click the 'Merge' button. 

**Note:** If your changes have been
approved, but you don't see a 'Merge' button, you likely don't have
permission to merge for that project. Talk to Ryan Plauche
(ryan@kubos.co).

In all likelyhood, you'll need to make changes to your code before your
pull request can be merged. Make the changes in your local development
environment and then commit and push them into your remote repo. As long
as you're still using the same local branch, these new changes will be
automatically added to your existing pull request.

Mark the Issue as 'Done'
------------------------

Before you mark the issue as done, verify the following: 

- All features listed in the issue have been completed 
- All relevant documentation changes have been made 
- All relevant unit tests have been created or updated 
- All code changes and related code have been tested 
- All pull requests related to the issue have been approved and merged

Update the issue's 'Fix version' field to reflect the version that these
changes are being implemented in.

Once all of the work for the issue has been completed, you can mark the
issue as Done in one of two ways: - From the full issue description
page, click the 'Done' button - From the 'Kanban Board' page, drag the
issue from the 'Reviewing' column into the 'Done' column
