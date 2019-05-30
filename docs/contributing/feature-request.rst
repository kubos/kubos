Requesting New Features
=======================

If KubOS is missing a feature which you think it should have, feel free to open a
`GitHub issue <https://github.com/kubos/kubos/issues/new/choose>`__ and tell us about it.

Note: This process does require that you have a GitHub account.

Creating an Issue
-----------------

In order to create a new feature request, navigate to the `main Kubos repo <https://github.com/kubos/kubos>`__
and click 'Issues', then click the 'New issue' button.

Alternatively, use this shortcut link: https://github.com/kubos/kubos/issues/new/choose

Once here, you should see a list of available issue templates.
Click the 'Get started' button that belongs to the "Feature request" template.

The issue title should be short, but descriptive.
Ideally, it should name the component which you would like to improve, as well as a high-level
overview of the new feature.

Examples:

- Telemetry database service bulk insert
- Expose additional commands for OEM6
- KubOS porting guide

Filling Out the Template
------------------------

GitHub will auto-populate the issue's body with the feature request template.

The template sections should be self-explanatory (if they're not, please open a :doc:`bug report <reporting-bugs>`
to correct the problem).

At a high level, we're looking for a description of the new feature you would like to see and an
explanation of why the feature is important.

If applicable, you might also include an overview of the specific behavior you'd like to see.
For example, if you wanted to expose additional hardware functionality, you might provide the
schema for the new GraphQL request which will need to be added.

Exploring the Request
---------------------

A Kubos team member should respond to the issue within one business day in order to request any
additional information and lay out the next steps.

If the feature request is not accepted, the reason for the rejection will be recorded and the issue
will be closed.
A request could be declined because a duplicate request already exists, or because the Kubos team
feels that it does not match the product's goals.

If the feature request is accepted, there will likely be some back and forth between you and/or
Kubos team members in order to determine the specifics of how the feature request should be
implemented and the low-level desired behavior.

The team (or potentially you or another community member!) will then implement the changes needed to
fulfill the request.
The exact timeframe required to complete this work will be dictated by current team resources and
the request's relevance to current priorities.

Once code changes have been created, the relevant PR will be linked to the feature request.

Closing the Request
-------------------

Once the request has been implemented and the changes have been merged into the master branch, a
Kubos team member will close the issue.