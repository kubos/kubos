# Kubos Manual Testing Package

This python module is meant to be used for easy manual testing of KubOS services. Currently, integration testing for hardware services is implemented. This utilizes the "testHardware" mutation for each hardware service outline in the hardware service schema.

## Usage

An example is provided for creating an integration test script in the code below.

.. code:

    import kubos_test
    config_location = "/path/to/system/config.toml"

    if __name__ == '__main__':
        test = kubos_test.IntegrationTest(config_location)
        print "Services Test"
        print "#############\n"
        test.test_services()
