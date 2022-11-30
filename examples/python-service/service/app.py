#!/usr/bin/env python3

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Boilerplate Flask setup for service application.
"""

from flask import Flask
from graphql_server.flask import GraphQLView
from .schema import schema


def create_app():
    """
    Creates graphql and graphiql endpoints
    """

    app = Flask(__name__)
    app.debug = True

    app.add_url_rule(
        '/',
        view_func=GraphQLView.as_view(
            'graphql',
            schema=schema,
            graphiql=False
        )
    )

    app.add_url_rule(
        '/graphiql',
        view_func=GraphQLView.as_view(
            'graphiql',
            schema=schema,
            graphiql=True
        )
    )

    return app
