<img src="data/icons/es.danirod.Cartero.svg" width="128" height="128">

# Cartero

Make HTTP requests and test APIs.

> 🚧 This is a work in progress and therefore you should expect that the
> application may not have all the features at this moment.

## Motivation

This project exists because there aren't many native graphical HTTP testing
applications / graphical alternatives to cURL that are fully free software, and
I think the world has had enough of Electron / non-native applications that are
anonymously accesible until one day you are forced to create an account and
log in to use.

## Roadmap

v0.1 is the first iteration and development is in progress. **The goal with
version v0.1 is to have a basic user interface to make HTTP requests
graphically**, supporting only the most basic features:

* Make HTTP requests setting the endpoint URL and the HTTP verb to use.
* Configure the payload and the request headers.
* Get the response headers, body, status code, size and duration of a request.

To achieve version 0.1, the following has to be done:

* [ ] Settle on the project structure (translations, GTK, build tools...)
* [ ] Design the user interface components (GTK 4, probably Adw 1).
* [ ] Implement the internal HTTP client and connect it to the user interface.

On future versions, more capabilities will be added:

* Support for persisting requests to re-use them in future sessions.
* Support for variables, which can be configured per environment, such as
  production, staging or development.
* Export a request as a cURL command or generate code for Axios, net/http and
  other software libraries.

## How to build

Note that compiling via `cargo build` and `cargo run` is not supported anymore,
since more build steps are required in order to compile the application
(translate .blp files into .ui files...). While it may work if you run the
commands on your own, it won't be pretty.

Currently, to build the application you'll have to make sure that the required
libraries are installed on your system.

* glib >= 2.72
* gtk >= 4.6
* gtksourceview >= 5.4

### Meson

Make sure that you have Meson in your system. For instance,

```sh
sudo apt install meson
sudo dnf install meson
sudo pacman -S meson
```

Then use the following commands to build and install the application

```sh
meson setup build
ninja -C build
ninja -C build install
```

To avoid installing system-wide the application, you can use a prefix:

```sh
meson setup build --prefix=~/.local
ninja -C build
ninja -C build install
```

During development mode, you may also prefer to install into the development
directory. Because the application may depend on files generated by Meson that
are installed into specific locations, this may be a required step if you
suddenly cannot run the application due to missing schemas or resources.

```sh
meson setup build --prefix=install
ninja -C build
ninja -C build install
install/bin/cartero
```

**If you plan on contributing to the project**, use the development profile.

```sh
meson setup build -Dprofile=development
```

It will also configure a Git hook so that the source code is checked prior to
authoring a Git commit. The hook runs `cargo fmt` to assert that the code is
formatted. Read `hooks/pre-commit.hook` to inspect what the script does.

### Flatpak

Install the runtime:

```sh
flatpak install --user org.gnome.Sdk//45 org.freedesktop.Sdk.Extension.rust-stable//23.08
```

To build and run the Flatpak:

```sh
flatpak-builder --user flatpak_app build-aux/es.danirod.Cartero.json
flatpak-builder --run flatpak_app build-aux/es.danirod.Cartero.json cartero
```

To install the Flatpak into your system or user Flatpak, use the `--install`
flag and maybe the `--user`:

```sh
flatpak-builder --user --install flatpak_app build-aux/es.danirod.Cartero.json
```

You will find Cartero in your application launcher, or you can launch it with
`flatpak run es.danirod.Cartero`.

## Contributing

> 🐛 This project is currently a larva trying to grow. Do you want to get in?
> Take a seat!

This project is highly appreciative of contributions. If you know about Rust,
GTK or the GNOME technologies and want to help during the development, you can
contribute if you wish. [Fork the project][fork] and commit your code.

Some checklist rules before submitting a pull request:

* **Use a feature branch**, do not make your changes in the trunk branch
  directly.

* **Rebase your code** and make sure that you are working on top of the most
  recent version of the trunk branch, in case something has changed while you
  were working on your code.

* **Update the locales** if you changed strings. The ninja target that you are
  looking for is called `cartero-update-po` (such as `ninja -C build
  cartero-update-po`). Don't worry, you don't have to translate the strings by
  yourself, but make sure that the new templates are added to the .po and .pot
  files.

* **Use the pre-commit hook**. The pre-commit hook will validate that your code
  is formatted. It should be automatically configured if you run Meson in
  development mode (`-Dprofile=development`), but you can install it on your
  own or run `hooks/pre-commit.hook`.

The project is starting small, so if you want to do something big, it is best
to first start a discussion thread with your proposal in order to see how to
make it fit inside the application.

While this application is not official and at the moment is not affiliated with
GNOME, you are expected to follow the [GNOME Code of Conduct][coc] when
interacting with this repository.

## Credits and acknowledgments

Cartero is being developed by [Dani Rodríguez][danirod].

Big shoutout to the [contributors][contrib] who have sent patches or
translations!

Also, Christian suggested Cartero as the name for the application and I liked
it enough to call it like so, therefore shoutout to Christian as well!

Finally, shoutout to many of the GTK and GNOME Circle applications out there whose
source code I've read in order to know how to use some of the GTK features that
you cannot learn just by reading the official docs.

## Blog

Dani's [dev blog][blog] (in Spanish) of Cartero.

[coc]: https://conduct.gnome.org
[contrib]: https://github.com/danirod/cartero/graphs/contributors
[danirod]: https://github.com/danirod
[fork]: https://github.com/danirod/cartero/fork
[blog]: https://danirod.es/secciones/devlogs/cartero/
