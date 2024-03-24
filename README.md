# Docker Dashboard

A minimalistic version of Docker-Desktop for Linux running only in the terminal written in Rust.

---

# Not finished yet!
# Work in progress!


## Requirements
 
The Following are **required** to run the application:
- `rust` >=  `1.77.0 (2021 Edition)`
- `curl` >= `7.50.0`

Optional dependencies (but **highly recommended**):
- **Nerdfonts** >= `3.0.0`

Internet is only needed when web access is required, for example <a href=https://hub.docker.com/>DockerHub</a> images.

## Optional ToDos
- [ ] Replace `curl` with the inbuilt `tokio::net::UnixStream`
- [ ] Add remote socket + authentication