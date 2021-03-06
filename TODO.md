# RISC-V

- [x] println macro
- [x] memory allocator
- [x] paging
- [ ] virtio driver
  - [x] Block
  - [ ] GPU
  - [ ] Input
- [x] user app
- [ ] system call

# Arm

- [ ] interrupt
  - [x] exception level 1 (corresponds to RISC-V's supervisor mode)
- [x] paging

# Common

- [x] process management
- [x] [giant lock](https://en.wikipedia.org/wiki/Giant_lock)
- [ ] inter-process communication
- [x] file system
- [ ] user application
  - [ ] program loader
- [ ] NIC driver
- [ ] TCP/IP protocol stack
- [ ] formal verification
  - https://github.com/xldenis/creusot ?
  - lean prover?
  - https://github.com/model-checking/kani
- testing
- [ ] binary compatibility with linux
- [ ] window system
  - https://github.com/ghaerr/microwindows
  - wayland?
- [x] x86_64 support
  - [x] boot
    - [x] UEFI
  - [x] print