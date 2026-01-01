/* TigerΔ: Dynamic Resonance Core (Time-Quantized Version)
 * Fix #1: Coarse time buckets instead of raw ktime
 */

#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <bpf/bpf_helpers.h>

char LICENSE[] SEC("license") = "GPL";

/* Dynamic salts: 0 = PHI, 1 = PI */
struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 2);
    __type(key, __u32);
    __type(value, __u64);
} config_map SEC(".maps");

/* Global resonance accumulator (will be fixed later) */
struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
} resonance_state SEC(".maps");

/* Policy switch: 0 = PASS, 1 = DROP */
struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u32);
} policy_map SEC(".maps");

static __always_inline __u64 rotl64(__u64 x, __u32 r)
{
    return (x << r) | (x >> (64 - r));
}

SEC("xdp")
int tiger_delta_xdp(struct xdp_md *ctx)
{
    void *data     = (void *)(long)ctx->data;
    void *data_end = (void *)(long)ctx->data_end;

    if (data + sizeof(struct ethhdr) > data_end)
        return XDP_PASS;

    struct ethhdr *eth = data;
    if (eth->h_proto != __constant_htons(ETH_P_IP))
        return XDP_PASS;

    struct iphdr *ip = data + sizeof(struct ethhdr);
    if ((void *)(ip + 1) > data_end)
        return XDP_PASS;

    /* --- Load dynamic salts --- */
    __u32 k_phi = 0, k_pi = 1;
    __u64 *phi_salt = bpf_map_lookup_elem(&config_map, &k_phi);
    __u64 *pi_salt  = bpf_map_lookup_elem(&config_map, &k_pi);

    __u64 phi = phi_salt ? *phi_salt : 0x6A09E667F3BCC909ULL;
    __u64 pi  = pi_salt  ? *pi_salt  : 0x243F6A8885A308D3ULL;

    /* --- Feature vector --- */
    __u64 v[4];

    v[0] = ((__u64)ip->saddr << 32) | ip->daddr;
    v[1] = ((__u64)ip->protocol << 48) | ip->tot_len;
    v[2] = (__u64)ctx->rx_queue_index;

    /* FIX #1: time quantization (~4 ms buckets) */
    v[3] = bpf_ktime_get_ns() >> 22;

    /* --- Entropy accumulation --- */
    __u64 acc = 0;
#pragma unroll
    for (int i = 0; i < 4; i++) {
        acc = (acc + rotl64(v[i] ^ pi, 13 + i)) * phi;
    }

    __u32 key = 0;
    __u64 *state = bpf_map_lookup_elem(&resonance_state, &key);
    if (!state)
        return XDP_PASS;

    __u64 new_state = (*state + acc) >> 1;
    bpf_map_update_elem(&resonance_state, &key, &new_state, BPF_ANY);

    __u32 *mode = bpf_map_lookup_elem(&policy_map, &key);
    if (mode && *mode == 1 && new_state > 0x8000000000000000ULL)
        return XDP_DROP;

    return XDP_PASS;
}
