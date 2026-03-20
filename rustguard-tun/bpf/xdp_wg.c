/* XDP program: redirect UDP:51820 to AF_XDP, pass everything else */
#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/ipv6.h>
#include <linux/udp.h>
#include <linux/in.h>

#define SEC(NAME) __attribute__((section(NAME), used))

/* BPF helper functions */
static void *(*bpf_map_lookup_elem)(void *map, const void *key) = (void *)1;
static long (*bpf_redirect_map)(void *map, __u32 key, __u64 flags) = (void *)51;

/* XSKMAP definition — old-style struct bpf_map_def */
struct bpf_map_def {
    unsigned int type;
    unsigned int key_size;
    unsigned int value_size;
    unsigned int max_entries;
    unsigned int map_flags;
};

struct bpf_map_def SEC("maps") xsks_map = {
    .type        = BPF_MAP_TYPE_XSKMAP,
    .key_size    = sizeof(__u32),
    .value_size  = sizeof(__u32),
    .max_entries = 64,
};

SEC("xdp")
int xdp_wireguard(struct xdp_md *ctx)
{
    void *data     = (void *)(__u64)ctx->data;
    void *data_end = (void *)(__u64)ctx->data_end;

    struct ethhdr *eth = data;
    if ((void *)(eth + 1) > data_end)
        return XDP_PASS;

    void *l4 = 0;

    if (eth->h_proto == __constant_htons(ETH_P_IP)) {
        struct iphdr *ip = (void *)(eth + 1);
        if ((void *)(ip + 1) > data_end)
            return XDP_PASS;
        if (ip->protocol != IPPROTO_UDP)
            return XDP_PASS;
        l4 = (void *)ip + (ip->ihl * 4);
    } else if (eth->h_proto == __constant_htons(ETH_P_IPV6)) {
        struct ipv6hdr *ip6 = (void *)(eth + 1);
        if ((void *)(ip6 + 1) > data_end)
            return XDP_PASS;
        if (ip6->nexthdr != IPPROTO_UDP)
            return XDP_PASS;
        l4 = (void *)(ip6 + 1);
    } else {
        return XDP_PASS;
    }

    struct udphdr *udp = l4;
    if ((void *)(udp + 1) > data_end)
        return XDP_PASS;

    if (udp->dest != __constant_htons(51820))
        return XDP_PASS;

    __u32 index = ctx->rx_queue_index;
    if (bpf_map_lookup_elem(&xsks_map, &index))
        return bpf_redirect_map(&xsks_map, index, XDP_PASS);

    return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
