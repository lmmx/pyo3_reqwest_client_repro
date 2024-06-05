from pytest import mark

from pyo3_file_handler_example import FileHandler, append_text


@mark.parametrize(
    "inputs",
    [("Hello, world!", " Goodbye, world!")],
)
def test_file_writing(inputs):
    input1, input2 = inputs
    # Create a new FileHandler instance for writing
    handler = FileHandler("example.txt", "w")
    handler.write(input1)
    handler.close()

    # Create a new FileHandler instance for reading
    handler = FileHandler("example.txt", "r")
    assert handler.read() == input1  # Should print "Hello, world!"
    handler.close()

    # Append text using the append_text function
    handler = FileHandler("example.txt", "a")
    handler.write(input2)
    handler.close()

    # Read the appended file
    handler = FileHandler("example.txt", "r")
    # Should print "Hello, world!" and Goodbye, world!"
    assert handler.read() == (input1 + input2)
    handler.close()
