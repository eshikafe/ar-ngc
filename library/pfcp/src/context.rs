use std::net::SocketAddr;

pub const PFCP_DEFAULT_PDR_PRECEDENCE: u32 = 255;
pub const PFCP_INDIRECT_PDR_PRECEDENCE: u3 = 1;
pub const PFCP_UP2CP_PDR_PRECEDENCE: u32 = 1;
pub const PFCP_CP2UP_PDR_PRECEDENCE: u32 = 1000;

pub const PFCP_DEFAULT_CHOOSE_ID: u32 = 5;
pub const PFCP_INDIRECT_DATA_FORWARDING_CHOOSE_ID: u32 = 10;

pub const MAX_NUM_OF_DEV: u32 = 16;
pub const MAX_NUM_OF_SUBNET: u32 = 16;

// typedef struct ogs_pfcp_node_s ogs_pfcp_node_t;

pub struct PfcpContext {
    pub pfcp_port: u32,   // PFCP local port
    pub tun_ifname: &str, // PFCP TUN Interface Name */
    pub pfcp_list: Vec,   // PFCP IPv4 Server List */
    pub pfcp_list6: Vec,  // PFCP IPv6 Server List */
    // ogs_sock_t      *pfcp_sock;     /* PFCP IPv4 Socket */
    // ogs_sock_t      *pfcp_sock6;    /* PFCP IPv6 Socket */
    pub pfcp_addr: SocketAddr,  // PFCP IPv4 Address
    pub pfcp_addr6: SocketAddr, // PFCP IPv6 Address

    pfcp_started: u32, // UTC time when the PFCP entity started

    // CP Function Features
    pub cp_function_features: PfcpCpFunctionFeatures,
    // UP Function Features
    pub up_function_features: PfcpUpFunctionFeatures,
    pub up_function_features_len: u32,

    pub pfcp_peer_list: Vec,      // PFCP Node List */
    pub pfcp_node: *mut PfcpNode, // Iterator for Peer round-robin */

    pub dev_list: Vec, // Tun Device List */
    pub subnet_list: vec, /* UE Subnet List */

                       // ogs_hash_t      *object_teid_hash; /* hash table for PFCP OBJ(TEID) */
                       // ogs_hash_t      *far_f_teid_hash;  /* hash table for FAR(TEID+ADDR) */
                       // ogs_hash_t      *far_teid_hash; /* hash table for FAR(TEID) */
}

// #define OGS_SETUP_PFCP_NODE(__cTX, __pNODE) \
//     do { \
//         ogs_assert((__cTX)); \
//         ogs_assert((__pNODE)); \
//         (__cTX)->pfcp_node = __pNODE; \
//     } while(0)

pub struct PfcpNode {
    lnode: ogs_lnode_t,       // A node of list_t */
    sa_list: vec<SocketAddr>, // Socket Address List Candidate */
    sock: *mut ogs_sock_t,    // Socket Instance */
    addr: SocketAddr,         // Remote Address */
    local_list: Vec,
    remote_list: Vec,
    sm: ogs_fsm_t,                    /* A state machine */
    t_association: *mut ogs_timer_t,  /* timer to retry to associate peer node */
    t_no_heartbeat: *mut ogs_timer_t, /* heartbeat timer to check aliveness */
    tac: [u16; MAX_NUM_OF_TAI],
    num_of_tac: u8,
    dnn: [&str; MAX_DNN_LEN + 1],
    num_of_dnn: u8,
    e_cell_id: [u32; MAX_NUM_OF_CELL_ID],
    num_of_e_cell_id: u8,
    nr_cell_id: [u64; MAX_NUM_OF_CELL_ID],
    num_of_nr_cell_id: u8,

    // flag to enable/ disable full list RR for this node */
    rr_enable: u8,
    gtpu_resource_list: Vec, /* User Plane IP Resource Information */
    up_function_features: PfcpUpFunctionFeatures,
    up_function_features_len: u32,
}

enum PfcpObjectType {
    PfcpObjBase,
    PfcpObjSessType,
    PfcpObjPdrType,
    PfcpObjTop,
}

struct PfcpObject {
    lnode: ogs_lnode_t,
    obj_type: PfcpObjectType,
}

// typedef struct ogs_pfcp_sess_s ogs_pfcp_sess_t;
// typedef struct ogs_pfcp_pdr_s ogs_pfcp_pdr_t;
// typedef struct ogs_pfcp_far_s ogs_pfcp_far_t;
// typedef struct ogs_pfcp_urr_s ogs_pfcp_urr_t;
// typedef struct ogs_pfcp_qer_s ogs_pfcp_qer_t;
// typedef struct ogs_pfcp_bar_s ogs_pfcp_bar_t;

struct PfcpPdr {
    obj: PfcpObject,
    index: u32,
    hash: u32,
    // struct {
    //     struct {
    //         int len;
    //         uint32_t key;
    //     } teid;
    // } hash;

    id_node: *mut u8,      /* Pool-Node for ID */
    id: PfcpPdrId,
    precedence: PfcpPrecedence,
    src_if: PfcpInterface,

    apn: &str,
    dnn: &str,

    ue_ip_addr: PfcpUeIpAddr
    ue_ip_addr_len: u32,

    f_teid: PfcpFTeid,
    f_teid_len: u32,

    chid: bool,
    choose_id: u8,

    ogs_pfcp_outer_header_removal_t outer_header_removal;
    outer_header_removal_len: u32,

    qfi: u8,

    ogs_pfcp_far_t          *far;

    num_of_urr: u32,
    ogs_pfcp_urr_t          *urr[OGS_MAX_NUM_OF_URR];

    ogs_pfcp_qer_t          *qer;

    num_of_flow: i32,
    char                    *flow_description[OGS_MAX_NUM_OF_FLOW_IN_PDR];

    ogs_list_t              rule_list;      /* Rule List */
    /* Related Context */
    ogs_pfcp_sess_t         *sess;
    void                    *gnode;         /* For CP-Function */
}

// typedef struct ogs_pfcp_far_hash_f_teid_s {
//     uint32_t teid;
//     uint32_t addr[4];
// } ogs_pfcp_far_hash_f_teid_t;

// typedef struct ogs_pfcp_far_s {
//     ogs_lnode_t             lnode;

//     struct {
//         struct {
//             int len;
//             ogs_pfcp_far_hash_f_teid_t key;
//         } f_teid;

//         struct {
//             int len;
//             uint32_t key;
//         } teid;
//     } hash;

//     uint8_t                 *id_node;      /* Pool-Node for ID */
//     ogs_pfcp_far_id_t       id;
//     ogs_pfcp_apply_action_t apply_action;
//     ogs_pfcp_interface_t    dst_if;
//     ogs_pfcp_outer_header_creation_t outer_header_creation;
//     int                     outer_header_creation_len;

//     ogs_pfcp_smreq_flags_t  smreq_flags;

//     uint32_t                num_of_buffered_packet;
//     ogs_pkbuf_t             *buffered_packet[OGS_MAX_NUM_OF_PACKET_BUFFER];

//     struct {
//         bool prepared;
//     } handover; /* Saved from N2-Handover Request Acknowledge */
//     /* Related Context */
//     ogs_pfcp_sess_t         *sess;
//     void                    *gnode;
// } ogs_pfcp_far_t;

// typedef struct ogs_pfcp_urr_s {
//     ogs_lnode_t             lnode;

//     uint8_t                 *id_node;      /* Pool-Node for ID */
//     ogs_pfcp_urr_id_t       id;

//     ogs_pfcp_measurement_method_t meas_method;
//     ogs_pfcp_reporting_triggers_t rep_triggers;
//     ogs_pfcp_measurement_information_t meas_info;
//     ogs_pfcp_measurement_period_t meas_period;

//     ogs_pfcp_volume_threshold_t vol_threshold;
//     ogs_pfcp_volume_quota_t vol_quota;

//     ogs_pfcp_event_threshold_t event_threshold;
//     ogs_pfcp_event_quota_t event_quota;

//     ogs_pfcp_time_threshold_t time_threshold;
//     ogs_pfcp_time_quota_t time_quota;

//     ogs_pfcp_quota_holding_time_t quota_holding_time;

//     ogs_pfcp_dropped_dl_traffic_threshold_t dropped_dl_traffic_threshold;

//     ogs_pfcp_quota_validity_time_t quota_validity_time;

//     ogs_pfcp_sess_t         *sess;
// } ogs_pfcp_urr_t;

// typedef struct ogs_pfcp_qer_s {
//     ogs_lnode_t             lnode;

//     uint8_t                 *id_node;      /* Pool-Node for ID */
//     ogs_pfcp_qer_id_t       id;

//     ogs_pfcp_gate_status_t  gate_status;
//     ogs_pfcp_bitrate_t      mbr;
//     ogs_pfcp_bitrate_t      gbr;

//     uint8_t                 qfi;

//     ogs_pfcp_sess_t         *sess;
// } ogs_pfcp_qer_t;

// typedef struct ogs_pfcp_bar_s {
//     ogs_lnode_t             lnode;

//     uint8_t                 *id_node;      /* Pool-Node for ID */
//     ogs_pfcp_bar_id_t       id;

//     ogs_pfcp_sess_t         *sess;
// } ogs_pfcp_bar_t;

// typedef struct ogs_pfcp_sess_s {
//     ogs_pfcp_object_t   obj;

//     ogs_list_t          pdr_list;       /* PDR List */
//     ogs_list_t          far_list;       /* FAR List */
//     ogs_list_t          urr_list;       /* URR List */
//     ogs_list_t          qer_list;       /* QER List */
//     ogs_pfcp_bar_t      *bar;           /* BAR Item */
//     OGS_POOL(pdr_id_pool, uint8_t);
//     OGS_POOL(far_id_pool, uint8_t);
//     OGS_POOL(urr_id_pool, uint8_t);
//     OGS_POOL(qer_id_pool, uint8_t);
//     OGS_POOL(bar_id_pool, uint8_t);
// } ogs_pfcp_sess_t;

// typedef struct ogs_pfcp_subnet_s ogs_pfcp_subnet_t;
// typedef struct ogs_pfcp_ue_ip_s {
//     uint32_t        addr[4];
//     bool            static_ip;

//     /* Related Context */
//     ogs_pfcp_subnet_t    *subnet;
// } ogs_pfcp_ue_ip_t;

// typedef struct ogs_pfcp_dev_s {
//     ogs_lnode_t     lnode;

//     char            ifname[OGS_MAX_IFNAME_LEN];
//     ogs_socket_t    fd;

//     ogs_poll_t      *poll;
//     bool            is_tap;
//     uint8_t         mac_addr[6];
// } ogs_pfcp_dev_t;

// typedef struct ogs_pfcp_subnet_s {
//     ogs_lnode_t     lnode;

//     ogs_ipsubnet_t  sub;                    /* Subnet : 2001:db8:cafe::0/48 */
//     ogs_ipsubnet_t  gw;                     /* Gateway : 2001:db8:cafe::1 */
//     char            dnn[OGS_MAX_DNN_LEN+1]; /* DNN : "internet", "volte", .. */
// #define OGS_MAX_NUM_OF_SUBNET_RANGE 16
//     struct {
//         const char *low;
//         const char *high;
//     } range[OGS_MAX_NUM_OF_SUBNET_RANGE];
//     int num_of_range;

//     int             family;         /* AF_INET or AF_INET6 */
//     uint8_t         prefixlen;      /* prefixlen */
//     OGS_POOL(pool, ogs_pfcp_ue_ip_t);

//     ogs_pfcp_dev_t  *dev;           /* Related Context */
// } ogs_pfcp_subnet_t;

// typedef struct ogs_pfcp_rule_s {
//     ogs_lnode_t lnode;

//     union {
//         struct {
// ED6(uint8_t     spare1:3;,
//     uint8_t     bid:1;,
//     uint8_t     fl:1;,
//     uint8_t     spi:1;,
//     uint8_t     ttc:1;,
//     uint8_t     fd:1;)
//         };
//         uint8_t flags;
//     };

//     ogs_ipfw_rule_t ipfw;
//     uint32_t sdf_filter_id;

//     /* Related Context */
//     ogs_pfcp_pdr_t  *pdr;
// } ogs_pfcp_rule_t;

// void ogs_pfcp_context_init(void);
// void ogs_pfcp_context_final(void);
// ogs_pfcp_context_t *ogs_pfcp_self(void);
// int ogs_pfcp_context_parse_config(const char *local, const char *remote);

// ogs_pfcp_node_t *ogs_pfcp_node_new(ogs_sockaddr_t *sa_list);
// void ogs_pfcp_node_free(ogs_pfcp_node_t *node);

// ogs_pfcp_node_t *ogs_pfcp_node_add(
//         ogs_list_t *list, ogs_sockaddr_t *addr);
// ogs_pfcp_node_t *ogs_pfcp_node_find(
//         ogs_list_t *list, ogs_sockaddr_t *addr);
// void ogs_pfcp_node_remove(ogs_list_t *list, ogs_pfcp_node_t *node);
// void ogs_pfcp_node_remove_all(ogs_list_t *list);

// ogs_gtpu_resource_t *ogs_pfcp_find_gtpu_resource(ogs_list_t *list,
//         char *dnn, ogs_pfcp_interface_t source_interface);
// int ogs_pfcp_setup_far_gtpu_node(ogs_pfcp_far_t *far);
// int ogs_pfcp_setup_pdr_gtpu_node(ogs_pfcp_pdr_t *pdr);

// void ogs_pfcp_sess_clear(ogs_pfcp_sess_t *sess);

// ogs_pfcp_pdr_t *ogs_pfcp_pdr_add(ogs_pfcp_sess_t *sess);
// ogs_pfcp_pdr_t *ogs_pfcp_pdr_find(
//         ogs_pfcp_sess_t *sess, ogs_pfcp_pdr_id_t id);
// ogs_pfcp_pdr_t *ogs_pfcp_pdr_find_or_add(
//         ogs_pfcp_sess_t *sess, ogs_pfcp_pdr_id_t id);

// void ogs_pfcp_object_teid_hash_set(
//         ogs_pfcp_object_type_e type, ogs_pfcp_pdr_t *pdr);
// ogs_pfcp_object_t *ogs_pfcp_object_find_by_teid(uint32_t teid);

// ogs_pfcp_pdr_t *ogs_pfcp_pdr_find_by_choose_id(
//         ogs_pfcp_sess_t *sess, uint8_t choose_id);

// void ogs_pfcp_pdr_reorder_by_precedence(
//         ogs_pfcp_pdr_t *pdr, ogs_pfcp_precedence_t precedence);
// void ogs_pfcp_pdr_associate_far(ogs_pfcp_pdr_t *pdr, ogs_pfcp_far_t *far);
// void ogs_pfcp_pdr_associate_urr(ogs_pfcp_pdr_t *pdr, ogs_pfcp_urr_t *urr);
// void ogs_pfcp_pdr_associate_qer(ogs_pfcp_pdr_t *pdr, ogs_pfcp_qer_t *qer);
// void ogs_pfcp_pdr_remove(ogs_pfcp_pdr_t *pdr);
// void ogs_pfcp_pdr_remove_all(ogs_pfcp_sess_t *sess);

// ogs_pfcp_far_t *ogs_pfcp_far_add(ogs_pfcp_sess_t *sess);
// ogs_pfcp_far_t *ogs_pfcp_far_find(
//         ogs_pfcp_sess_t *sess, ogs_pfcp_far_id_t id);
// ogs_pfcp_far_t *ogs_pfcp_far_find_or_add(
//         ogs_pfcp_sess_t *sess, ogs_pfcp_far_id_t id);

// void ogs_pfcp_far_f_teid_hash_set(ogs_pfcp_far_t *far);
// ogs_pfcp_far_t *ogs_pfcp_far_find_by_error_indication(ogs_pkbuf_t *pkbuf);

// void ogs_pfcp_far_teid_hash_set(ogs_pfcp_far_t *far);
// ogs_pfcp_far_t *ogs_pfcp_far_find_by_teid(uint32_t teid);

// void ogs_pfcp_far_remove(ogs_pfcp_far_t *far);
// void ogs_pfcp_far_remove_all(ogs_pfcp_sess_t *sess);

// ogs_pfcp_urr_t *ogs_pfcp_urr_add(ogs_pfcp_sess_t *sess);
// ogs_pfcp_urr_t *ogs_pfcp_urr_find(
//         ogs_pfcp_sess_t *sess, ogs_pfcp_urr_id_t id);
// ogs_pfcp_urr_t *ogs_pfcp_urr_find_or_add(
//         ogs_pfcp_sess_t *sess, ogs_pfcp_urr_id_t id);
// void ogs_pfcp_urr_remove(ogs_pfcp_urr_t *urr);
// void ogs_pfcp_urr_remove_all(ogs_pfcp_sess_t *sess);

// ogs_pfcp_qer_t *ogs_pfcp_qer_add(ogs_pfcp_sess_t *sess);
// ogs_pfcp_qer_t *ogs_pfcp_qer_find(
//         ogs_pfcp_sess_t *sess, ogs_pfcp_qer_id_t id);
// ogs_pfcp_qer_t *ogs_pfcp_qer_find_or_add(
//         ogs_pfcp_sess_t *sess, ogs_pfcp_qer_id_t id);
// void ogs_pfcp_qer_remove(ogs_pfcp_qer_t *qer);
// void ogs_pfcp_qer_remove_all(ogs_pfcp_sess_t *sess);

// ogs_pfcp_bar_t *ogs_pfcp_bar_new(ogs_pfcp_sess_t *sess);
// void ogs_pfcp_bar_delete(ogs_pfcp_bar_t *bar);

// ogs_pfcp_rule_t *ogs_pfcp_rule_add(ogs_pfcp_pdr_t *pdr);
// ogs_pfcp_rule_t *ogs_pfcp_rule_find_by_sdf_filter_id(
//         ogs_pfcp_sess_t *sess, uint32_t sdf_filter_id);
// void ogs_pfcp_rule_remove(ogs_pfcp_rule_t *rule);
// void ogs_pfcp_rule_remove_all(ogs_pfcp_pdr_t *pdr);

// int ogs_pfcp_ue_pool_generate(void);
// ogs_pfcp_ue_ip_t *ogs_pfcp_ue_ip_alloc(
//         uint8_t *cause_value, int family, const char *dnn, uint8_t *addr);
// void ogs_pfcp_ue_ip_free(ogs_pfcp_ue_ip_t *ip);

// ogs_pfcp_dev_t *ogs_pfcp_dev_add(const char *ifname);
// void ogs_pfcp_dev_remove(ogs_pfcp_dev_t *dev);
// void ogs_pfcp_dev_remove_all(void);
// ogs_pfcp_dev_t *ogs_pfcp_dev_find_by_ifname(const char *ifname);

// ogs_pfcp_subnet_t *ogs_pfcp_subnet_add(
//         const char *ipstr, const char *mask_or_numbits,
//         const char *dnn, const char *ifname);
// ogs_pfcp_subnet_t *ogs_pfcp_subnet_next(ogs_pfcp_subnet_t *subnet);
// void ogs_pfcp_subnet_remove(ogs_pfcp_subnet_t *subnet);
// void ogs_pfcp_subnet_remove_all(void);
// ogs_pfcp_subnet_t *ogs_pfcp_find_subnet(int family);
// ogs_pfcp_subnet_t *ogs_pfcp_find_subnet_by_dnn(int family, const char *dnn);

// void ogs_pfcp_pool_init(ogs_pfcp_sess_t *sess);
// void ogs_pfcp_pool_final(ogs_pfcp_sess_t *sess);
