import unittest
from .main import reduce_polymers, is_reactive

class DayFiveTests(unittest.TestCase):
    def test_example_result(self):
        result = reduce_polymers('dabAcCaCBAcCcaDA')

        self.assertEqual(result, 'dabCBAcaDA')

    def test_is_reactive(self):
        self.assertTrue(is_reactive('aA'))
        self.assertTrue(is_reactive('Ff'))

        self.assertFalse(is_reactive('bb'))
        self.assertFalse(is_reactive('CC'))
        self.assertFalse(is_reactive('CD'))
        self.assertFalse(is_reactive('AB'))