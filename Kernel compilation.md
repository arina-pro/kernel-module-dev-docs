# Kernel compilation
This guide goes through the steps to build the Linux kernel and its modules from source code and to boot and start kernel module development.

Note: run the following commands with flag `-j$(nproc)` (number of processing units) to speed up the process. Do not forget `LLVM=1` flag to tell `make` to use `Clang`.

## 1. Configure the kernel

### 1a. Copy existing configuration
To ensure that the necessary drivers are supported, copy your existing kernel configuration file to `linux-rust-<..>` folder.
```
// Run from inside linux-rust-<..> folder
cp /boot/config-$(uname -r) .config
```

### 1b. Modify configuration to enable Rust
Run following command
```
// Run from inside linux-rust-<..> folder
make LLVM=1 -j$(nproc) menuconfig
```
and enable `Rust support` option (you can search by typing `/`). If you do not find it, check which requirements are not met.
```
CONFIG_RUST=y`
```

## 2. Build the kernel
To check Rust is available
```
// Run from inside linux-rust-<..> folder
make LLVM=1 -j$(nproc) rustavailable
```
To build Rust analyzer
```
// Run from inside linux-rust-<..> folder
make LLVM=1 -j$(nproc) rust-analyzer
```
To `make` the kernel
```
make LLVM=1 -j$(nproc)
```
By default, `make` or `make all` build these 3 targets: `vmlinux` (bare kernel), `modules` (built-in) and compressed kernel image (`bzImage`).

## 3. Build and install the modules
Build modules
```
// Run from inside linux-rust-<..> folder
make LLVM=1 -j$(nproc) modules
```
Install them
```
// Run from inside linux-rust-<..> folder
sudo make LLVM=1 -j$(nproc) modules_install
```

## 4. Configure boot and boot with the newly built kernel
Update `initramfs` and `grub` configuration to be able to boot with the new kernel
```
// Run from inside linux-rust-<..> folder
sudo make LLVM=1 -j$(nproc) install
```
To reboot, run
```
reboot
```

After rebooting, check your kernel version
```
uname -r
```
You should see the version you installed in environment setup.

Now you are ready to build your first kernel module!

## (Optional) Re-compile kernel
Unless you are certain that you did not corrupt the kernel build files, first run the cleaning command
```
// Run from inside linux-rust-<..> folder
make mrproper
```
to delete all the generated files.

Then repeat steps 1-4.

Note: every time you modify `.config`, you need to re-compile the kernel, if the change is not specific to a module, and/or modules (the whole process or only modules, if the change is related to a module).
