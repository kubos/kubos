#!/usr/bin/env python

from flask import Flask
from flask_graphql import GraphQLView
from schema import schema


def create_app():
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
