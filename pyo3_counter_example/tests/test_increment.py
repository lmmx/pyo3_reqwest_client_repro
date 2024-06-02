import pyo3_counter_example

def test_increment():
    c = pyo3_counter_example.Counter()
    assert c.get_value() == 0
    c.increment()
    assert c.get_value() == 1
    c.increment()
    assert c.get_value() == 2
