pub enum SmfEventType {
    SmfEvtBase, // OGS_FSM_USER_SIG,
    SmfEvtS5cMessage,
    SmfEvtGMessage,
    SmfEvtGxMessage,
    SmfEvtGyMessage,
    SmfEvtN4Message,
    SmfEvtN4Timer,
    SmfEvtN4NoHeartbeat,
    SmfEvtSbiServer,
    SmfEvtSbiClient,
    SmfEvtSbiTimer,
    SmfEvtNgapMessage,
    SmfEvtNgapTimer,

   // SMF_EVT_5GSM_MESSAGE,
   // SMF_EVT_5GSM_TIMER,

   SmfEvtTop,
}

pub struct SmfEvent {
    id: u32,
    // ogs_pkbuf_t *pkbuf;
    timer_id: u32,

    // ogs_gtp_node_t *gnode;
    // ogs_gtp_xact_t *gtp_xact;

    ogs_pfcp_node_t *pfcp_node;
    ogs_pfcp_xact_t *pfcp_xact;
    ogs_pfcp_message_t *pfcp_message;

    union {
        ogs_diam_gx_message_t *gx_message;
        ogs_diam_gy_message_t *gy_message;
    };

    struct {
        ogs_sbi_request_t *request;
        ogs_sbi_response_t *response;
        void *data;
        int state;

        ogs_sbi_message_t *message;
    } sbi;

    struct {
        int type;
        ogs_ngap_message_t *message;
    } ngap;

    struct {
        uint8_t type;
        ogs_nas_5gs_message_t *message;
    } nas;

    smf_sess_t *sess;
} smf_event_t;

void smf_event_init(void);
void smf_event_final(void);

smf_event_t *smf_event_new(smf_event_e id);
void smf_event_free(smf_event_t *e);

const char *smf_event_get_name(smf_event_t *e);
