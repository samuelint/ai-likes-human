import socket


class ExtensionIpcPortService:
    def find_next_available_port(start_port=49152, end_port=65535):
        """
        Finds the next available port within the specified range.
        Default range is ephemeral port (https://en.wikipedia.org/wiki/Ephemeral_port).

        Args:
            start_port (int): The starting port number (inclusive).
            end_port (int): The ending port number (inclusive).

        Returns:
            int: The next available port number, or None if no port is available.
        """
        for port in range(start_port, end_port + 1):
            try:
                s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
                s.bind(("localhost", port))
                s.close()
                return port
            except OSError:
                continue
        return None
