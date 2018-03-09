"""Module for creating db connection and ORM models
"""
from sqlalchemy import *
from sqlalchemy.orm import (
    scoped_session, sessionmaker, relationship, backref)

from sqlalchemy.ext.declarative import declarative_base

db_session = None
Base = declarative_base()

class Telemetry(Base):
    __tablename__ = 'Telemetry'
    timestamp = Column(BigInteger, primary_key=True)
    subsystem = Column(String)
    param = Column(String)
    value = Column(BigInteger)

def init_db(path):
    global db_session
    global Base

    engine = create_engine("sqlite:///%s" % path, convert_unicode=True)
    db_session = scoped_session(sessionmaker(autocommit=False, autoflush=False, bind=engine))
    Base.query = db_session.query_property()

def get_db():
    global db_session

    if not db_session:
        print "ERRR"
        return None
    return db_session
