#!/bin/bash
bindgen wrapper.h -o src/bindings.rs \
    --default-enum-style=rust_non_exhaustive \
    --allowlist-type cef_.* \
    --allowlist-function cef_.* \
    --bitfield-enum .*_mask_t \
    -- -I /home/wuyuwei/Desktop/fec
