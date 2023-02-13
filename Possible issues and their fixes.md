# Possible issues and their fixes
This is a non-exhaustive list of problems encountered during the Linux kernel module development process.

## `x509 certs`
### Fix
sed modify `.config` for `TRUSTED` and `REVOCATION_KEYS` to `""`

## Missing modules
### Issue
`.config` was modified at some point and is no longer supported in the current kernel version. 
### Fix
and needs to be re-copied from `/boot/` and 
On boot, Ubuntu crashed: missing modules, root UUID=... not found (`cat /proc/modules; ls dev` message). Disk problem. Did you install the drivers correctly? The first build of Linux kernel takes ~1h for 2 processor cores due to various drivers installation.

## `/dev/sdaN: clean, nnn/nnn files, nnn/nnn blocks` on boot
### Issue
Disk space error. Did you expand the hard disk of VM? Re-partitioned?
### Fix
Follow [this answer](https://unix.stackexchange.com/a/610154), could be also done while experiencing booting error (press `Ctrl+Alt+F3` to boot login).

## `Exec format error
### Issue
when trying to run a module: unsupported in current kernel version.
### Fix
Make sure to run the module that is supported in the current kernel.

## `Module not found`
### Issue
Are you sure you ran `make modules_install`? Did you run as `sudo` (as it needs access to `/lib/modules/`)? Did it finish without errors?
### Fix
Check `/lib/modules/` and the current kernel directory (`$(uname -r)`).

## `make` error: `Permission denied`
### Issue
During `make` and/or `make modules` permission denied? It is probably due to overwriting files if you are re-building the kernel.
### Fix
Clean all the generated files with `make mrproper`.

## `rustc` / `rustup` not found
### Issue
Are you running `make` as `sudo`?
### Fix
It is recommended to run `make` as user, fix the errors that demand `sudo` access (see previous issue). You have as well the option to run `sudo -s` and set-up Rust toolchain (Section 3) from `root`, which may not be the best solution.