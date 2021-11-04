# ech

## Quickstart
Download and install `ech` from `crates.io`
```bash
cargo install ech
```
Runs ech on port 8000
```bash
ech 8000
```
Choose a random available port to run on
```bash
ech
```

## Use
Simply returns any request back to the caller with a `HTTP/1.1 200 OK` response code and logs it to `stdout`, giving the full raw TCP stream as text.

Helpful when debugging a poorly documented application making unknown requests, point it to the port ech is running on and see exactly what is being sent out.
