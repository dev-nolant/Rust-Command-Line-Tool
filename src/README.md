
# 🛠️ Ping Module

## Overview
This module provides functionality to send ICMP echo requests (ping) to a specified destination IP address. It demonstrates how to use low-level Rust code to interact with Windows API functions for network operations.

## Dependencies
This module relies on the following Rust standard library components:
- `std::ffi::c_void`: For working with raw pointers and C-style void pointers.
- `std::fmt`: For implementing the `Debug` trait.
- `std::mem`: For memory operations like `size_of` and `transmute`.
- `std::time::Instant`: For measuring elapsed time.
- `std::thread`: For handling thread operations like sleeping.

## How It Works
The `ping` module interacts with Windows API functions to send ICMP echo requests. The key functions used are `LoadLibraryA` and `GetProcAddress` to dynamically load and retrieve the addresses of functions from `IPHLPAPI.dll`.

### Key Components
- **IPAddr**: A struct representing an IPv4 address.
- **IpOptionInformation**: A struct containing IP option information such as TTL and TOS.
- **IcmpEchoReply**: A struct representing the ICMP echo reply.
- **IcmpSendEcho**: A type definition for the ICMP send echo function.
- **IcmpCreateFile**: A type definition for creating an ICMP handle.

The module will send ICMP echo requests to the specified IP address and display the results, including round-trip time and other statistics.

### Notes
- Ensure that you have the necessary permissions to use ICMP echo requests on your system.
- This code is specific to Windows due to the use of Windows API functions. Adaptations would be required for other operating systems.

### Contributing
Contributions are welcome! Feel free to fork the repository, make changes, and submit a pull request. You will be listed as a contributor.

### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
