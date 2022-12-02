#!/usr/bin/env python
import unittest

from sub1_lib import Sub1


class TestPipelines(unittest.TestCase):
    def test_attrs(self):
        s = Sub1(10, 20)
        assert 10 == s.base_attr
        assert 20 == s.sub1_attr

    def test_methods(self):
        s = Sub1(10, 20)
        assert 40 == s.base_add(30)

if __name__ == "__main__":
    unittest.main()

