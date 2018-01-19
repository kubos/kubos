import graphene

class Payload(graphene.ObjectType):
    """Class modeling the payload"""

    # Member modeling payload on/off state
    on = graphene.Boolean()

# Local payload instance so we can have persistence
my_payload = Payload(on=False)

class Query(graphene.ObjectType):
    """Query class used to define GraphQL query structures
    """
    payload = graphene.Field(Payload)

    def resolve_payload(self, info):
        return my_payload

class PayloadEnable(graphene.Mutation):
    """Mutation class for modeling enable mutation for Payload"""
    class Arguments:
        on = graphene.Boolean(required=True)

    Output = Payload

    def mutate(self, info, on):
        my_payload.on = on
        return my_payload

class Mutation(graphene.ObjectType):
    enable = PayloadEnable.Field()

schema = graphene.Schema(query=Query, mutation=Mutation)
