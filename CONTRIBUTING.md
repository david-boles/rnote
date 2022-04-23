# How to build and contribute

## Building
### Building with Flatpak
There is a flatpak manifest in `build-aux/com.github.flxzt.rnote.Devel.json`.

Use Gnome Builder or vscode with the flatpak extension to build and run the application for you. **This is the easiest and recommended way.**

**Bugs and workarounds**

- If you encounter `bwrap: Can't find source path /run/user/1000/doc/by-app/com.github.flxzt.rnote: No such file or directory` when trying to run the flatpak, `xdg-document-portal` did not start yet. Starting it manually with `systemctl start --user xdg-document-portal` should fix it.

- As long as the flatpak is not installed on the system, The DirectoryList in the workspace browser does not update when files are created, removed or changed. It will work in the released flatpak.
--- 

If you don't have an IDE or extension to handle building flatpaks, you can also do it manually:

First the Gnome 42 SDK and some extensions are needed:

```bash
flatpak install org.gnome.Sdk//42 org.freedesktop.Sdk.Extension.rust-stable//21.08 org.gnome.Platform//42
```

**Build**  
Building the app with flatpak is done with:

```bash
flatpak-builder --user flatpak-app build-aux/com.github.flxzt.rnote.Devel.json
```

Creating a repo:

```bash
flatpak-builder --user --repo=flatpak-repo flatpak-app build-aux/com.github.flxzt.rnote.Devel.json
```


**Install**  
Install to the system as user with:

```bash
flatpak-builder --user --install flatpak-app build-aux/com.github.flxzt.rnote.Devel.json
```

**Run**  
Then it can be run.
From the build directory:

```bash
flatpak-builder --run flatpak-app build-aux/com.github.flxzt.rnote.Devel.json rnote
```

Or if it is installed:

```bash
flatpak run com.github.flxzt.rnote
```

### Build with Meson
The flatpak manifest calls the meson build system to build the application.
If a native build is wanted, meson can be called directly.

Make sure `rustc` and `cargo` are installed. Then run:

```bash
meson setup --prefix=/usr _mesonbuild
```
Meson will ask for the user password when needed.

To enable the development profile, set `-Dprofile=devel` as a parameter. Else the `default` profile will be set. ( This can be reconfigured later )

**Compile**  
Once the project is configured, it can be compiled with:

```bash
meson compile -C _mesonbuild
```

The compiled binary should now be here: `./_mesonbuild/target/release/rnote`.

**Install**  
Installing the binary into the system can be done with:

```bash
meson install -C _mesonbuild
```

**Test**  
Meson has some tests to validate the desktop, gresources, ... files.

```bash
meson test -v -C _mesonbuild
```

This places the files in the specified prefix and their subpaths. The binary should now be in `/usr/bin` (and therefore in PATH)

**Reconfigure**  
reconfiguring the meson build files can be done with:

```bash
meson configure -Dprofile=default _mesonbuild
```

For example if the profile needs to be changed.


**Uninstall**  
If you don't like rnote, or decided that is not worth your precious disk space, you can always uninstall it with:

```bash
sudo ninja uninstall -C _mesonbuild
```

#### Example Development Environment Setup
These instructions assume you use a Debian-based distro but should be easy to adapt to other environments:

1. Install VSCode and debugging extensions:
    - https://code.visualstudio.com/
    - https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer
    - https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb
2. Clone the repository:
    ```bash
    git clone https://github.com/flxzt/rnote.git
    cd rnote
    git submodule update --init
    ```
3. Install GTK4: https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html
4. Install additional dependencies:
    ```bash
    sudo apt install    \
    libadwaita-1-dev    \
    libasound2-dev      \
    libgstreamer1.0-dev \
    libpoppler-glib-dev
    ```
5. Set up Meson for in-place development builds:
    ```bash
    meson setup --prefix=$(pwd)/_mesoninstall -Dprofile=devel _mesonbuild
    ```
6. Create a task in `.vscode/tasks.json` for building Rnote:
    ```json
    {
        "version": "2.0.0",
        "tasks": [
            {
                "label": "Build Rnote",
                "type": "shell",
                "command": "meson compile -C _mesonbuild && meson install -C _mesonbuild",
                "options": {
                    "cwd": "${workspaceFolder}"
                }
            }
        ]
    }
    ```
7. Create a launch configuration in `.vscode/launch.json` for debugging Rnote:
    ```json
    {
        "version": "0.2.0",
        "configurations": [
            {
                "type": "lldb",
                "request": "custom",
                "name": "Debug Rnote",
                "preLaunchTask": "Build Rnote",
                "targetCreateCommands": [
                    "target create ${workspaceFolder}/_mesoninstall/bin/rnote"
                ],
                "processCreateCommands": [
                    // Optionally, arguments:
                    // "settings set target.run-args value1 value2 value3",
                    "process launch"
                ]
            }
        ]
    }
    ```

# Contribute
Please open an issue or ask in the `Github Discussions` section if you need help with anything!