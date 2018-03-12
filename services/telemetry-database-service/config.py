import yaml

config = None

def init_config(path):
    global config

    with open(path) as ymlfile:
        config = yaml.load(ymlfile)
