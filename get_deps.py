#!/usr/bin/python3

import toml

master_toml = toml.load("Cargo.toml")

member_dependencies = {}
installed_dependencies = {}

for member in master_toml['workspace']['members']:
    member_toml = toml.load(f"{member}/Cargo.toml")
    if 'dependencies' in member_toml:
        for dep, attr in member_toml['dependencies'].items():
            if 'path' in attr:
                continue
            member_dependencies[dep] = attr

lock_toml = toml.load("Cargo.lock")

# print(member_dependencies)

for member in lock_toml['package']:
    if member['name'] in member_dependencies:
        installed_dependencies[member['name']] = member['version']
        print(f"{member['name']}\t{member['version']}\thttps://crates.io/crates/{member['name']}")