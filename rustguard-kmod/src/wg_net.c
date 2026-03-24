// SPDX-License-Identifier: GPL-2.0
/*
 * RustGuard — C shim for networking primitives.
 *
 * The kernel's Rust bindings (6.12) don't expose net_device, sk_buff,
 * or the crypto API. This thin C layer handles device registration and
 * provides callback hooks that the Rust module implements.
 *
 * Think of it as the plumbing — Rust handles the protocol, C handles
 * the kernel API surface that doesn't have Rust abstractions yet.
 */

#include <linux/module.h>
#include <linux/netdevice.h>
#include <linux/etherdevice.h>
#include <linux/if_arp.h>
#include <linux/rtnetlink.h>
#include <linux/ip.h>
#include <linux/ipv6.h>
#include <linux/skbuff.h>

#define WG_MTU 1420
#define WG_NETDEV_NAME "wg%d"

/* Forward declarations — implemented in Rust. */
extern netdev_tx_t rustguard_xmit(struct sk_buff *skb, void *priv);
extern void rustguard_dev_uninit(void *priv);

/* Per-device private data — pointer to Rust-side state. */
struct wg_device {
	void *rust_priv;
};

/* ── net_device_ops callbacks ──────────────────────────────────────── */

static netdev_tx_t wg_xmit(struct sk_buff *skb, struct net_device *dev)
{
	struct wg_device *wg = netdev_priv(dev);

	if (unlikely(!wg->rust_priv)) {
		kfree_skb(skb);
		dev->stats.tx_dropped++;
		return NETDEV_TX_OK;
	}

	return rustguard_xmit(skb, wg->rust_priv);
}

static int wg_open(struct net_device *dev)
{
	netif_start_queue(dev);
	return 0;
}

static int wg_stop(struct net_device *dev)
{
	netif_stop_queue(dev);
	return 0;
}

static const struct net_device_ops wg_netdev_ops = {
	.ndo_open       = wg_open,
	.ndo_stop       = wg_stop,
	.ndo_start_xmit = wg_xmit,
};

/* ── Device setup ──────────────────────────────────────────────────── */

static void wg_setup(struct net_device *dev)
{
	dev->netdev_ops = &wg_netdev_ops;

	/* WireGuard is a point-to-point tunnel, not Ethernet. */
	dev->type = ARPHRD_NONE;
	dev->flags = IFF_POINTOPOINT | IFF_NOARP;
	dev->priv_flags |= IFF_NO_QUEUE;

	dev->mtu = WG_MTU;
	dev->hard_header_len = 0;
	dev->addr_len = 0;
	dev->needed_headroom = 0;
	dev->needed_tailroom = 0;

	/* No features — raw IP in, encrypted UDP out. */
	dev->features |= NETIF_F_LLTX;
	dev->features |= NETIF_F_NETNS_LOCAL;
}

/* ── Exported to Rust ──────────────────────────────────────────────── */

/*
 * Allocate and register a new WireGuard net_device.
 * Returns the net_device pointer on success, ERR_PTR on failure.
 * The Rust side stores this and calls wg_destroy_device on cleanup.
 */
struct net_device *wg_create_device(void *rust_priv)
{
	struct net_device *dev;
	struct wg_device *wg;
	int ret;

	dev = alloc_netdev(sizeof(struct wg_device), WG_NETDEV_NAME,
			   NET_NAME_UNKNOWN, wg_setup);
	if (!dev)
		return ERR_PTR(-ENOMEM);

	wg = netdev_priv(dev);
	wg->rust_priv = rust_priv;

	ret = register_netdev(dev);
	if (ret) {
		free_netdev(dev);
		return ERR_PTR(ret);
	}

	return dev;
}
EXPORT_SYMBOL_GPL(wg_create_device);

/*
 * Unregister and free a WireGuard net_device.
 */
void wg_destroy_device(struct net_device *dev)
{
	if (!dev)
		return;
	unregister_netdev(dev);
	free_netdev(dev);
}
EXPORT_SYMBOL_GPL(wg_destroy_device);

/*
 * Inject a decrypted packet into the kernel network stack.
 * Called by Rust after successful transport decryption.
 * Takes ownership of the skb.
 */
void wg_net_rx(struct net_device *dev, struct sk_buff *skb)
{
	skb->dev = dev;
	skb->ip_summed = CHECKSUM_UNNECESSARY;
	skb->protocol = ip_tunnel_parse_protocol(skb);
	skb_reset_network_header(skb);

	dev->stats.rx_packets++;
	dev->stats.rx_bytes += skb->len;

	netif_rx(skb);
}
EXPORT_SYMBOL_GPL(wg_net_rx);

/*
 * Allocate an skb for outgoing encrypted data.
 * Returns NULL on allocation failure.
 */
struct sk_buff *wg_alloc_skb(unsigned int len)
{
	return alloc_skb(len, GFP_ATOMIC);
}
EXPORT_SYMBOL_GPL(wg_alloc_skb);

/*
 * Get the raw data pointer and length from an skb.
 * For reading the plaintext packet before encryption.
 */
void wg_skb_data(struct sk_buff *skb, unsigned char **data, unsigned int *len)
{
	*data = skb->data;
	*len = skb->len;
}
EXPORT_SYMBOL_GPL(wg_skb_data);

/*
 * Free an skb (e.g., after we've consumed the plaintext).
 */
void wg_kfree_skb(struct sk_buff *skb)
{
	kfree_skb(skb);
}
EXPORT_SYMBOL_GPL(wg_kfree_skb);

/*
 * Update TX stats after successful encryption + send.
 */
void wg_tx_stats(struct net_device *dev, unsigned int bytes)
{
	dev->stats.tx_packets++;
	dev->stats.tx_bytes += bytes;
}
EXPORT_SYMBOL_GPL(wg_tx_stats);
