"""
This module creates the schema bindings between GraphQL queries and
SQLAlchemy models. In this case there is a single query and single
binding dealing with the Telemetry table.
"""
import graphene
from graphene_sqlalchemy import SQLAlchemyObjectType
from models import Telemetry as TelemetryModel
from models import get_db

class Telemetry(SQLAlchemyObjectType):
    """Telemetry class for use in Graphene queries
    """
    class Meta:
        """Meta class for pointing to TelemetryModel class
        """
        model = TelemetryModel


class Query(graphene.ObjectType):
    """Query class used to define GraphQL query structures
    """
    telemetry = graphene.List(Telemetry,
                              first=graphene.Int(),
                              subsystem=graphene.String(),
                              param=graphene.String(),
                              timestamp_before=graphene.Float(),
                              timestamp_after=graphene.Float())

    def resolve_telemetry(self,
                          info,
                          first=None,
                          subsystem=None,
                          param=None,
                          timestamp_before=None,
                          timestamp_after=None):
        model = Telemetry._meta.model
        query = Telemetry.get_query(info)
        print(type(query))
        if subsystem:
            query = query.filter(model.subsystem == subsystem)
        if param:
            query = query.filter(model.param == param)
        if timestamp_before:
            query = query.filter(model.timestamp <= timestamp_before)
        if timestamp_after:
            query = query.filter(model.timestamp >= timestamp_after)
        if first:
            return query.limit(first)
        return query.all()

class CreateTelemetry(graphene.Mutation):

    class Arguments:
        subsystem = graphene.String(required=True)
        param = graphene.String(required=True)
        value = graphene.Float(required=True)
        timestamp = graphene.Float(required=True)

    Output = Telemetry

    def mutate(self, info, subsystem, param, value, timestamp):
        db_session = get_db()
        new_telem = TelemetryModel(subsystem=subsystem,
                         param=param,
                         value=value,
                         timestamp=timestamp)

        db_session.add(new_telem)
        db_session.commit()

        return Telemetry(subsystem=subsystem, param=param, value=value, timestamp=timestamp)


class Mutation(graphene.ObjectType):
    create_telemetry = CreateTelemetry.Field()

schema = graphene.Schema(query=Query, mutation=Mutation)
