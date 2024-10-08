# Install Fyrox on Ubuntu 22.04.1

To install Fyrox on Ubuntu 22.04.1, follow these steps:

### 1. Download Fyrox:
  - Visit the Fyrox download page on SourceForge (https://sourceforge.net/projects/fyrox.mirror/) and download the latest version (v0.31.zip) for Linux.
  - Alternatively, you can download Fyrox from OnWorks, a cloud-based platform for running Linux applications online. This method allows you to run Fyrox without installing it locally.

### 2. Extract the archive:
  - Once downloaded, extract the zip file to a directory of your choice (e.g., ~/fyrox).

### 3. Install dependencies:
  - Fyrox requires Rust and some additional dependencies to be installed. You can install them using the following command:
```
sudo apt-get install rust rustc
```
  - If you don’t have apt-get installed, you can use sudo apt instead.

### 4. Compile and run Fyrox:
  - Navigate to the extracted Fyrox directory and run the following command to compile the engine:
```
cargo build
```
  - Once compiled, you can run Fyrox using:
```
./target/debug/fyrox
```
## Optional: Remove the Firefox Snap

If you want to install Firefox as a DEB package instead of using the Snap, follow the instructions from OMG! Ubuntu! (https://www.omgubuntu.co.uk/how-to-install-firefox-deb-on-ubuntu-not-snap).

## Note: 
Fyrox is a game engine written in Rust, and its installation process might be more complex than typical Linux applications. If you encounter any issues during installation, refer to the Fyrox documentation or seek help from the community.

