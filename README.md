# ech

## Quickstart
- `cargo install ech` - downloads and installs `ech` from `crates.io`
- `ech 8000` - runs ech on port 8000
- `ech` - chooses a random available port to run on

## Use
Simply returns any request back to the caller with a `HTTP/1.1 200 OK` response code and logs it to `stdout`, giving the full raw TCP stream as text.

Helpful when debugging a poorly documented application which is making requests, point it to the port ech is running on and see exactly what it's sending out.
