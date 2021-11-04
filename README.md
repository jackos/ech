# ech

## Quickstart
```bash
cargo install ech
```
Downloads and installs `ech` from `crates.io`
```ech 8000```
Runs ech on port 8000
```ech```
Chooses a random available port to run on

## Use
Simply returns any request back to the caller with a `HTTP/1.1 200 OK` response code and logs it to `stdout`, giving the full raw TCP stream as text.

Helpful when debugging a poorly documented application making unknown requests, point it to the port ech is running on and see exactly what is being sent out.
