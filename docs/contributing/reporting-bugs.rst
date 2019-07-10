Reporting Bugs
==============

While all devs like to think that they write perfect code, we know that reality doesn't quite agree.

If you encounter a bug while using KubOS or while working with our source code, please open
a `GitHub issue <https://github.com/kubos/kubos/issues/new/choose>`__ and tell us about it.

Notes:

- This process does require that you have a GitHub account
- If you find a small bug that is very easily fixed (like a typo or one-line change), feel free to
  just correct it and submit a PR, instead of going through this full process

Creating an Issue
-----------------

In order to create a new bug report, navigate to the `main Kubos repo <https://github.com/kubos/kubos>`__
and click 'Issues', then click the 'New issue' button.

Alternatively, use this shortcut link: https://github.com/kubos/kubos/issues/new/choose

Once here, you should see a list of available issue templates.
Click the 'Get started' button that belongs to the "Bug report" template.

The issue title should be short, but descriptive.
Ideally, it should name the area in which the error occurs, as well as a high-level overview of the
problem.

Examples:

- Monitor service failing to start
- Can't change log level for Rust app
- Kubos Linux install instructions no longer valid

Filling Out the Template
------------------------

GitHub will auto-populate the issue's body with the bug report template.
We appreciate all the information you're able to provide, however we acknowledge that sometimes
the information just isn't available, or you might not have the time in order to create an in-depth
report.

The template sections should be self-explanatory (if they're not, please open a bug report to
correct the problem).

Some sections should be included with all bug reports:

- Description
- Severity

Some sections are only relevant if the problem is with a running process or system behavior:

- System Details
- To Reproduce
- Bug Output
- Workaround

Fixing the Problem
------------------

A Kubos team member should respond to the issue within one business day in order to request any
additional information and lay out the next steps.

The team (or potentially a community member!) will correct the issue in an appropriate timeframe.
The exact timeframe required to complete this work will be dictated by current team resources, as
well as the issue's severity.

Once code changes have been created, the relevant PR will be linked to the bug report.

Closing the Report
------------------

Once the bug has been fixed and the changes have been merged into the master branch, a Kubos team
member will close the issue.