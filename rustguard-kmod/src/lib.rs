// SPDX-License-Identifier: GPL-2.0

//! RustGuard — WireGuard kernel module in Rust.
//!
//! Registers a virtual network device (wg%d) and handles WireGuard packet
//! encrypt/decrypt directly in the network stack.
//!
//! The C shim (wg_net.c) handles net_device registration and sk_buff plumbing.
//! This side handles the WireGuard protocol: handshake, transport crypto, timers.

use kernel::prelude::*;

module! {
    type: RustGuard,
    name: "rustguard",
    author: "cali",
    description: "WireGuard VPN — Rust implementation",
    license: "GPL",
}

// FFI declarations for the C shim functions.
extern "C" {
    fn wg_create_device(rust_priv: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn wg_destroy_device(dev: *mut core::ffi::c_void);
    fn wg_skb_data(
        skb: *mut core::ffi::c_void,
        data: *mut *mut u8,
        len: *mut u32,
    );
    fn wg_kfree_skb(skb: *mut core::ffi::c_void);
    fn wg_tx_stats(dev: *mut core::ffi::c_void, bytes: u32);
}

/// Per-device state. Rust owns this, C holds a pointer to it.
struct WgDeviceState {
    /// Opaque pointer to the C net_device.
    net_dev: *mut core::ffi::c_void,
    // TODO: peer table
    // TODO: private key
    // TODO: listen port
    // TODO: UDP socket
}

// SAFETY: WgDeviceState is only accessed from contexts where the kernel
// guarantees single-threaded access (module init/exit) or under locks.
unsafe impl Send for WgDeviceState {}
unsafe impl Sync for WgDeviceState {}

struct RustGuard {
    dev_state: Pin<Box<WgDeviceState>>,
}

impl kernel::Module for RustGuard {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("rustguard: initializing\n");

        // Allocate device state.
        let mut state = Box::pin_init(kernel::init::pin_init!(WgDeviceState {
            net_dev: core::ptr::null_mut(),
        }), GFP_KERNEL)?;

        // Create the net_device. Pass a pointer to our state as the private data.
        // The C shim stores this in netdev_priv(dev)->rust_priv.
        let state_ptr = &*state as *const WgDeviceState as *mut core::ffi::c_void;
        // SAFETY: wg_create_device allocates and registers a net_device.
        // It stores state_ptr as the device's private data.
        let dev = unsafe { wg_create_device(state_ptr) };
        if dev.is_null() || (dev as isize) < 0 {
            pr_err!("rustguard: failed to create net device\n");
            return Err(ENOMEM);
        }

        // SAFETY: We have exclusive access to state during init.
        unsafe {
            let state_mut = Pin::get_unchecked_mut(state.as_mut());
            state_mut.net_dev = dev;
        }

        pr_info!("rustguard: device created\n");
        Ok(RustGuard { dev_state: state })
    }
}

impl Drop for RustGuard {
    fn drop(&mut self) {
        let dev = self.dev_state.net_dev;
        if !dev.is_null() {
            // SAFETY: dev was returned by wg_create_device and is valid.
            unsafe { wg_destroy_device(dev) };
        }
        pr_info!("rustguard: unloaded\n");
    }
}

/// Called by the C shim when a packet arrives on the wg interface for transmission.
/// This is ndo_start_xmit — the hot path.
///
/// # Safety
/// Called from C with a valid sk_buff pointer and our rust_priv pointer.
#[no_mangle]
pub extern "C" fn rustguard_xmit(
    skb: *mut core::ffi::c_void,
    _priv: *mut core::ffi::c_void,
) -> i32 {
    // For now: count the packet and drop it.
    // TODO: look up peer by AllowedIPs, encrypt, send via UDP socket.
    unsafe { wg_kfree_skb(skb) };

    // NETDEV_TX_OK = 0
    0
}

/// Called by the C shim when the device is being torn down.
///
/// # Safety
/// Called from C with our rust_priv pointer.
#[no_mangle]
pub extern "C" fn rustguard_dev_uninit(_priv: *mut core::ffi::c_void) {
    pr_info!("rustguard: device uninit\n");
}
