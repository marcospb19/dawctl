See bytes\_disposal.txt to understand the messages bytes structure.

Each .JSON file contain only frames from wireshark where frame.len == 126, that is, only messages sent to the device from windows drivers and synapse tweaks.

The .py script filters unwanted fffffff lines that are used for polling, and show the bytes for each iteration of test, from this we could understand the bytes sequence and write our own version of it in Rust.
