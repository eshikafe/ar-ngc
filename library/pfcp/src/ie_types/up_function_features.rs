// 8.2.25 UP Function Features

pub struct UpFunctionFeatures {
    pub ie_type: u16,
    pub ie_length: u16,
    pub supported_features: u16,
    pub additional_supported_features_1: u16,
    pub additional_supported_features_2: u16,
    
    union {
        struct {
/*
 * 5/8 TREU Sxb, Sxc, N4
 *   Traffic Redirection Enforcement in the UP function is supported
 *   by the UP function.
 * 5/7 HEEU Sxb, Sxc, N4
 *   Header Enrichment of Uplink traffic is supported by the UP function.
 * 5/6 PFDM Sxb, Sxc, N4
 *   The PFD Management procedure is supported by the UP function.
 * 5/5 FTUP Sxa, Sxb, N4
 *   F-TEID allocation / release in the UP function is supported
 *   by the UP function.
 * 5/4 TRST Sxb, Sxc, N4
 *   Traffic Steering is supported by the UP function.
 * 5/3 DLBD Sxa, N4
 *   The buffering parameter 'DL Buffering Duration' is supported
 *   by the UP function.
 * 5/2 DDND Sxa, N4
 *   The buffering parameter 'Downlink Data Notification Delay' is supported
 *   by the UP function.
 * 5/1 BUCP Sxa, N4
 *   Downlink Data Buffering in CP function is supported by the UP function.
 */
ED8(uint8_t treu:1;,
    uint8_t heeu:1;,
    uint8_t pfdm:1;,
    uint8_t ftup:1;,
    uint8_t trst:1;,
    uint8_t dldb:1;,
    uint8_t ddnd:1;,
    uint8_t bucp:1;)
        };
        uint8_t octet5;
    };
    union {
        struct {
/*
 * 6/8 EPFAR Sxa, Sxb, Sxc, N4
 *   The UP function supports the Enhanced PFCP Association Release feature
 *   (see clause 5.18).
 * 6/7 PFDE Sxb, N4
 *   The UP function supports a PFD Contents including a property
 *   with multiple values.
 * 6/6 FRRT Sxb, N4
 *   The UP function supports Framed Routing
 *   (see IETF RFC 2865 [37] and IETF RFC 3162 [38]).
 * 6/5 TRACE Sxa, Sxb, Sxc, N4
 *   The UP function supports Trace (see clause 5.15).
 * 6/4 QUOAC Sxb, Sxc, N4
 *   The UP function supports being provisioned with the Quota Action
 *   to apply when reaching quotas.
 * 6/3 UDBC Sxb, Sxc, N4
 *   Support of UL/DL Buffering Control
 * 6/2 PDIU Sxa, Sxb, Sxc, N4
 *   Support of PDI optimised signalling in UP function (see clause 5.2.1A.2).
 * 6/1 EMPU Sxa, Sxb, N4
 *   Sending of End Marker packets supported by the UP function.
 */
ED8(uint8_t epfar:1;,
    uint8_t pfde:1;,
    uint8_t frrt:1;,
    uint8_t trace:1;,
    uint8_t quoac:1;,
    uint8_t udbc:1;,
    uint8_t pdiu:1;,
    uint8_t empu:1;)
        };
        uint8_t octet6;
    };
    union {
        struct {
/*
 * 7/8 GCOM N4
 *   UPF support of 5G VN Group Communication.(See clause 5.23)
 * 7/7 BUNDL Sxa, Sxb, Sxc, N4
 *   PFCP messages bunding (see clause 6.5) is supported by the UP function.
 * 7/6 MTE N4
 *   UPF supports multiple instances of Traffic Endpoint IDs in a PDI.
 * 7/5 MNOP Sxa, Sxb, Sxc, N4
 *   The UP function supports measurement of number of packets
 *   which is instructed with the flag 'Measurement of Number of Packets'
 *   in a URR.See also clause 5.2.2.2.1.
 * 7/4 SSET N4
 *   UPF support of PFCP sessions successively controlled
 *   by different SMFs of a same SMF Set (see clause 5.22).
 * 7/3 UEIP Sxb, N4
 *   The UP function supports allocating UE IP addresses or prefixes
 *   (see clause 5.21).
 * 7/2 ADPDP Sxa, Sxb, Sxc, N4
 *   The UP function supports the Activation and Deactivation
 *   of Pre-defined PDRs (see clause 5.19).
 * 7/1 DPDRA Sxb, Sxc, N4
 *   The UP function supports Deferred PDR Activation or Deactivation.
 */
ED8(uint8_t gcom:1;,
    uint8_t bundl:1;,
    uint8_t mte:1;,
    uint8_t mnop:1;,
    uint8_t sset:1;,
    uint8_t ueip:1;,
    uint8_t adpdp:1;,
    uint8_t dpdra:1;)
        };
        uint8_t octet7;
    };
    union {
        struct {
/*
 * 8/8 MPTCP N4
 *   UPF support of MPTCP Proxy functionality (see clause 5.20)
 * 8/7 TSCU N4
 *   Time Sensitive Communication is supported by the UPF (see clause 5.26).
 * 8/6 IP6PL N4
 *   UPF supports:
 *     - UE IPv6 address(es) allocation with IPv6 prefix length other than
 *       default /64 (including allocating /128 individual IPv6 addresses),
 *       as specified in clause 4.6.2.2 of of 3GPP TS 23.316 [57]; and
 *     - multiple UE IPv6 addresses allocation using multiple instances
 *       of the UE IP Address IE in a same PDI or Traffic Endpoint,
 *       or using multiple PDIs or Traffic Endpoints
 *       with a different UE IP Address as specified in clause 5.21.1.
 * 8/5 IPTV N4
 *   UPF support of IPTV service (see clause 5.25)
 * 8/4 NORP Sxa, Sxb, Sxc, N4
 *   UP function support of Number of Reports as specified in clause 5.2.2.2.
 * 8/3 VTIME Sxb,N4
 *   UP function support of quota validity time feature.
 * 8/2 RTTL N4
 *   UPF supports redundant transmission at transport layer.
 * 8/1 MPAS N4
 *   UPF support for multiple PFCP associations to the SMFs in an SMF set
 *   (see clause 5.22.3).
 */
ED8(uint8_t mptcp:1;,
    uint8_t tscu:1;,
    uint8_t ip6pl:1;,
    uint8_t iptv:1;,
    uint8_t norp:1;,
    uint8_t vtime:1;,
    uint8_t rttl:1;,
    uint8_t mpas:1;)
        };
        uint8_t octet8;
    };
    union {
        struct {
/*
 * 9/8 RDS Sxb, N4
 *   UP function support of Reliable Data Service (see clause 5.29).
 * 9/7 DDDS N4
 *   UPF support of reporting the first buffered / discarded downlink data
 *   for downlink data delivery status notification.
 * 9/6 ETHAR N4
 *   UPF support of Ethernet PDU Session Anchor Relocation (see clause 5.13.6).
 * 9/5 CIOT Sxb, N4
 *   UP function support of CIoT feature,
 *   e.g.small data packet rate enforcement.(see 5.4.15)
 * 9/4 MT-EDT Sxa
 *   SGW-U support of reporting the size of DL Data Packets.
 *   (see clause 5.2.4.1).
 * 9/3 GPQM N4
 *   UPF support of per GTP-U Path QoS monitoring (see clause 5.24.5).
 * 9/2 QFQM N4
 *   UPF support of per QoS flow per UE QoS monitoring (see clause 5.24.4).
 * 9/1 ATSSS-LL N4
 *   UPF support of ATSSS-LLL steering functionality (see clause 5.20)
 */
ED8(uint8_t rds:1;,
    uint8_t ddds:1;,
    uint8_t ethar:1;,
    uint8_t ciot:1;,
    uint8_t mt_edt:1;,
    uint8_t gpqm:1;,
    uint8_t qfqm:1;,
    uint8_t atsss_ll:1;)
        };
        uint8_t octet9;
    };
    union {
        struct {
/*
 * 10/1 RTTWP N4
 *   UPF support of RTT measurements towards the UE Without PMF.
 */
ED2(uint8_t reserved:7;,
    uint8_t rttwp:1;)
        };
        uint8_t octet10;
    };
}
