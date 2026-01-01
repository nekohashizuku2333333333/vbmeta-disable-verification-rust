# vbmeta-disable-verification-rust

> Equals to [libxzr/vbmeta-disable-verification](https://github.com/libxzr/vbmeta-disable-verification),but in rust.
><br>Rewritten in vibe.

Patch Android vbmeta image and disable verification flags inside.

## Usage

```
$ cargo build
$ ./vbmeta-disable-verification vbmeta.img 
Successfully disabled verification on the provided vbmeta image.
```

Give it a vbmeta image and then verification will be disabled on it.

This should be equal to `fastboot --disable-verity --disable-verification flash vbmeta vbmeta.img`. The only difference is that it directly patch the image file. Fastboot doesn't provide the ability to generate an image with verification disabled, but sometimes I need it :) .