// +build ignore
#include <linux/bpf.h>
#include <linux/in.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <bpf/bpf_helpers.h>

SEC("xdp")
int mpr_kernel_packet_filter(struct xdp_md *ctx) {
    void *data_end = (void *)(long)ctx->data_end;
    void *data = (void *)(long)ctx->data;

    struct ethhdr *eth = data;
    if ((void *)(eth + 1) > data_end) return XDP_PASS;
    if (eth->h_proto != __constant_htons(ETH_P_IP)) return XDP_PASS;

    struct iphdr *iph = (void *)(eth + 1);
    if ((void *)(iph + 1) > data_end) return XDP_PASS;

    if (iph->protocol == IPPROTO_TCP) {
        char debug_msg[] = "MPR-Kernel-Isolation: Preempting saturating traffic loops at driver edge.";
        bpf_trace_printk(debug_msg, sizeof(debug_msg));
        return XDP_DROP; 
    }
    return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
