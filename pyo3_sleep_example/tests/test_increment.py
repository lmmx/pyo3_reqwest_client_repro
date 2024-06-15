import pyo3_sleep_example

def test_increment():
    c = pyo3_sleep_example.Counter()
    assert c.get_value() == 0
    c.increment()
    assert c.get_value() == 1
    c.increment()
    assert c.get_value() == 2

def test_sleep():
    c = pyo3_sleep_example.Counter()
    assert c.get_value() == 0
    c.increment()
    assert c.get_value() == 1
    c.increment()
    assert c.get_value() == 2
