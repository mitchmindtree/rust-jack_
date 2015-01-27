# rust-jack [![Build Status](https://travis-ci.org/nicklan/rust-jack.svg?branch=master)](https://travis-ci.org/nicklan/rust-jack)

This is a wrapper in Rust around the c api for
[Jack](http://jackaudio.org/).  It includes some example clients in
the bin directory.

Many functions are missing at the moment but eventually all jack api
calls will be included.  Pull requests for adding more functions are
welcome!

You can view a list of which functions are implemented and which
aren't
[here](http://htmlpreview.github.io/?https://raw.githubusercontent.com/nicklan/rust-jack/master/funcs.html)

The Jack api can be found
[here](http://jackaudio.org/files/docs/html/index.html).
At some point there will be a proper api doc for this rust library as
well.

# Installation
Use [Cargo](http://doc.crates.io/) and add the following to your Cargo.toml

```
[dependencies.jack]
git = "https://github.com/nicklan/rust-jack.git"
```


#Building
to build simply do

    cargo build


You will need to have libjack installed.
