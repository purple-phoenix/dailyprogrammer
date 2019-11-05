import os
import unittest
import xmlrunner


def test_all():
    loader = unittest.TestLoader()
    suites = loader.discover(".", "*_tests.py")
    runner = xmlrunner.XMLTestRunner("results")
    for suite in suites:
        runner.run(suite)


if __name__ == '__main__':
    test_all()
