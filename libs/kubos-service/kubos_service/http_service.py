#!/usr/bin/env python3

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.


"""
Wrapper for creating a HTTP based Kubos service
"""

from flask import Flask
from graphql_server.flask import GraphQLView


def start(config, schema, context={}):
    """
    Creates flask based graphql and graphiql endpoints
    """

    app = Flask(__name__)
    app.debug = True

    app.add_url_rule(
        '/',
        view_func=GraphQLView.as_view(
            'graphql',
            schema=schema,
            context=context,
            graphiql=False
        )
    )

    app.add_url_rule(
        '/graphiql',
        view_func=GraphQLView.as_view(
            'graphiql',
            schema=schema,
            context=context,
            graphiql=True
        )
    )

    app.run(config.ip, config.port)
