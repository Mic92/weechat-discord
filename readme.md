Weechat Discord
===============

### Deadness indicator: 100%

I'm done. I'm 900% done. Someone can feel free to fork this jar of horrors, but I'm done dealing with the nightmare of the Discord protocol. The PR on discord-api-docs removing the /ack endpoint documentation was the last straw, it was *clearly* never intended to be used as a client. As there is no API that's actually intended to be used, I'm not going to maintain this steaming pile of Rust FFI barbarity.

I'll leave things the way they are so folks can still use it, but don't expect me to be enthusiastic about bug reports.

---

### Building

Dependencies:

* Weechat developer libraries. Usually called `weechat-dev`, or sometimes just `weechat` includes them.
* OpenSSL 1.0 (NOT 1.1). Usually called `openssl-1.0`, `libssl1.0-dev`, or something similar. *This is important*. The Makefile will set up paths to override the version to 1.0, so if you get errors building openssl-sys-extras, run `cargo clean` and use the Makefile.

The makefile should give enough information for build commands. Here's the essentials:

    cd weechat-discord # or wherever you cloned it
    cargo build --release

This will produce a shared object called `target/release/libweecord.so`. Place it in your weechat plugins directory, which is probably located at `~/.weechat/plugins` (may need to be created)

The Makefile has a tiny bit of automation that helps with development:

    make # (same as make all) just runs that `cargo build --release` command, produces weecord.so
    make install # builds and copies the .so to ~/.weechat/plugins, creating the dir if required
    make run # installs and runs `weechat -a` (-a means "don't autoconnect to servers")

Maybe important note: The previous version of this project, written in Go, used to get **really upset** when the .so was modified during the same weechat session, even if unloaded. When developing, make sure to completely quit weechat when updating the .so, just to be sure (otherwise you might get a SIGSEGV and hard crash).

### Using

Due to some idiocracy on Discord's part, [you will need to obtain a login token](https://github.com/hammerandchisel/discord-api-docs/issues/69#issuecomment-223886862). The wonderful tip by the Discord devs on how to do that, though, doesn't work: for me, and only for chromium, I went into inspector (ctrl+shift+i, because Discord *also* decided to do the *wonderful* thing of ripping out right-click menus), Application tab, Local Storage on left, discordapp.com, token entry.

Set that token (make sure to remove the quotes if you copied them in!):

    /discord token 123456789ABCDEF

Then, connect:

    /discord connect

Note you may also have to adjust a few settings for best use:

    ## doesn't work currently: weechat.completion.default_template -> append "|%(weecord_completion)"
    weechat.bar.status.items -> replace buffer_name with buffer_short_name
    plugins.var.python.go.short_name -> on (if you use go.py)
