# KubOS Contribution Process

This is the workflow you should go through in order to create a complete a task to contribute to the KubOS project

1. [Create a JIRA Issue](#create-a-jira-issue)
2. [Mark the Issue as 'In Progress'](#mark-the-issue-as-in-progress)
3. [Create a Personal Copy of the Code You Want to Work On](#create-a-personal-copy-of-the-code-you-want-to-work-on)
4. [Make Your Changes](#make-your-changes)
5. [Create a Pull Request](#create-a-pull-request)
6. [Merge in New Changes From Master](#merge-in-new-changes-from-master)
7. [Wait for Pull Request Approval](#wait-for-pull-request-approval)
8. [Mark the Issue as 'Done'](#mark-the-issue-as-done)


## Create a JIRA Issue

Everything you work on should have a corresponding JIRA issue.  

If one doesn't exist, you should create it:
- Click the 'Create' button at the top of the JIRA page
- Project should be 'KubOS'
- Issue type should be 'Story' or 'Bug'
    * A story is something to be added or updated within the project
    * A bug is something that is broken within the project
- The summary should be a short description of what's being changed
- The 'Component/s' field should be updated, if possible, to list the related area/s affected by this issue.
- The description should go into more detail about what the problem/feature is and what needs to be done in order
to complete the task.
- If you're creating a bug, update the 'Affects Version/s' field to document the oldest affected version of the project

If you want to create an issue but not immediately work on it, the description field should be an in-depth summary of the problem/feature
and what needs to be changed.  Ideally, you, or whoever works the issue, should be able to read the description and understand what work 
needs to be done without talking to whoever created the issue (though talking to the creator is still recommended in order to make sure that
the requirements are well understood and haven't changed since the issue was created).

## Mark the Issue as 'In Progress'

In order to track what's being worked on and by whom, for every issue you work you should:
- Drag the issue from the backlog into the sprint (if it's not already present)
- Assign it to yourself
	* Click the issue
	* Under 'People'>'Assignee', click 'Assign to me'
- Mark the issue as 'In Progress'.  There are two ways:
	* From the full issue description page, click the 'In Progress' button
	* From the 'Active Sprints' page, drag the issue from the 'To-Do' column into the 'In Progress' column
	
## Create a Personal Copy of the Code You Want to Work On

All code changes should initially be made in your own personal repo and then be submitted against the main code repo as a pull request.

To create your own repo:
- Navigate to the github page of the main code that you want to work on.  For example, https://github.com/kubostech/kubos-sdk.
- Click the 'Fork' button in the upper right-hand corner.
- If you see a dialog 'Where should we fork this repository?', click the icon with your username.
- Within your development environment, create a link to your new remote repository:

	$ git remote add {repo name you create} {personal repo url}
	
You'll also want to create a local branch to work on inside of your development environment.

From the project folder where you'll be making changes:

	$ git checkout -b {local branch name you create}

You will need to fork each repository where you will be making changes.

## Make Your Changes

The code.  Write it.  Test it.  

At some point, there will be coding standards.
At that point, this document should be updated with links to:
- The C standard
- The python standard

For now, the code should be well structured:
- There should be appropriate documentation for new methods and variables.
	+ Use the doxygen comment format of '/** _comments_ */' for the method documentation so that it can be picked up during doc generation
- Use descriptive names for methods and variables.
- Try to match the styling of the surrounding code.

Update any documentation areas that are affected by your changes.  For instance, if you found that a uart configuration option was not available
for a certain board type, you would edit kubos-hal/kubos-hal/uart.h in the appropriate comment section with a new note about the unsupported option.
- If you want to test your documentation changes:
	+ Run the command 'doxygen docs/Doxyfile' from the project folder with the new documentation to regenerate the html documentation
	+ Open up {project}/html/index.html in a web browser
	+ Browse to your doc updates to verify

Add or update any unit tests that are affected by your changes.  For instance, if support for i2c slave mode is added for the STM32F4 board, then the
kubos-hal-stm32f4/test/i2c.c file's test\_i2c\_slave test case should be updated to test the successful execution of the board in slave mode.

Commit your changes and push to your remote repository:

	$ git add {files you changed}
	$ git commit -m "Descriptive message about the changes you made"
	$ git push {repo name} {local branch name}
	
[Commit early, commit often](http://www.databasically.com/2011/03/14/git-commit-early-commit-often/)

## Create a Pull Request

Once all of your changes for an issue have been completed, you should create a pull request in order to bring the changes into the main code's
master branch.  You will need to create a pull request for each repository you are making changes to.

From the github page for your personal repository that contains the changes you want to merge:
- Click the 'Branch:' dropdown on the left-hand side and select the local branch containing your changes
- Click the 'New pull request' button
- The title of the pull request should be the JIRA issue number followed by a descriptive title
	+ Ex. JIRA-111 Adding i2c slave mode for STM32F4
- In the description field, add a small summary of the changes you made.  The title should have indicated the bulk of the changes you made,
but it's also good to mention things like documentation updates and any miscellaneous changes that were made (for example, fixing any bugs
that you ran into while working on your code changes).
- Click 'Create pull request'


## Merge in New Changes From Master

After submitting your pull request, you may find that github has flagged one or more files as being in conflict with the current version of the file
in the master branch.  This means that someone else has committed code in the same file and similar area as you and your changes can't be 
automatically merged.

In order to resolve the conflict, execute the following steps within your development environment:

Merge the master branch into your local branch

	$ git checkout kubostech/master
	$ git pull kubostech master
	$ git checkout {local branch where your changes are}
	$ git merge kubostech/master

Git will edit any files with conflicts.  Conflicts will look like this:
	
		>>>Head
			New local changes
		==========
			New master changes
		<<<kubostech
		
Edit the files to resolve the conflicts.
Push the resolved changed to your remote repo

	$ git add {fixed files}
	$ git commit
	$ git push {repo name} {local branch name}

If you navigate to your pull request, you should now see that github says "This branch has no conflicts with the base branch", indicating that
the changes okay to merge (pending pull request approval).

## Wait for Pull Request Approval

Once your pull request has been submitted, it must be approved by at least one person before the request can be merged into the master branch.
Once it has been approved, you can go to your pull request page and then click the 'Merge' button.
- Note:  If your changes have been approved, but you don't see a 'Merge' button, you likely don't have permission to merge for that project. 
Talk to Ryan Plauche.

In all likelyhood, you'll need to make changes to your code before your pull request can be merged.  Make the changes in your local development 
environment and then commit and push them into your remote repo.  As long as you're still using the same local branch, these new changes will
be automatically added to your existing pull request.

## Mark the Issue as 'Done'

Before you mark the issue as done, verify the following:
- All features listed in the issue have been completed
- All relevant documentation changes have been made
- All relevant unit tests have been created or updated
- All code changes and related code have been tested
- All pull requests related to the issue have been approved and merged

Update the issue's 'Fix version' field to reflect the version that these changes are being implemented in.

Once all of the work for the issue has been completed, you can mark the issue as Done in one of two ways:
- From the full issue description page, click the 'In Progress' button
- From the 'Active Sprints' page, drag the issue from the 'To-Do' column into the 'In Progress' column