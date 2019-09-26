Running KubOS Core Services Locally
===================================

This document will lead you through the steps of configuring and running the KubOS core
services locally. All of the KubOS core services are capable of being run within
your local development environment. This will allow you to get up and running with
KubOS before you have an OBC in hand.

A prerequisite for following this document is having your
:doc:`local environment setup <local-setup>` and having a copy of the
`KubOS repo <https://github.com/kubos/kubos>`__ cloned locally.

Configuring Services
--------------------

The KubOS core services all rely on a central file for run-time configuration information.
In a standard Kubos Linux system, that file will be located at
``/etc/kubos-config.toml``. As a result, this is the default location
where services will check. When running locally, it is unlikely that file location
will exist, so we will need to create a custom config file and then
manually pass it to the service.

This central configuration file is in the ``toml`` format and contains a section
for each service. Here is a `default config <https://github.com/kubos/kubos/blob/master/tools/default_config.toml>`__
you can use when running the core services. It can be found at ``tools/default_config.toml``
in the Kubos repo folder.

In the config file each service has their own section(s). Any custom configuration
for the service will go in a section named after the service::

    [service-name]
    config_option = "value"

Any service which exposes a GraphQL interface will have a separate section for
configuring the GraphQL server::

    [service-name.addr]
    ip = "127.0.0.1"
    port = 8000

All of the KubOS core services require an ``.addr`` section in the configuration file.

A local configuration file will only need config data for the services which you
intend to run locally. If you would like to run a hardware service, each existing
:doc:`hardware service <../ecosystem/services/hardware-services>`
provided by Kubos documents their configuration options within their respective pages.

Running Services
----------------

Each service can either be run from the base of the Kubos repo with this command::

    $ cargo run --bin service-name -- -c tools/default_config.toml

Or by navigating to the service's source folder and running this command::

    $ cargo run -- -c ../../tools/default_config.toml

These examples point to the default config file provided with the repo with
the argument ``-- -c tools/default_config.toml``, however that path can be
changed to point to a custom config file.