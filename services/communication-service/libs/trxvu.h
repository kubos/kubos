typedef enum {
  RX_MAX_SIZE = 200
} ffi_constants;
typedef enum {
  RADIO_OK = 0,
  RADIO_RX_EMPTY,
  RADIO_ERROR,
  RADIO_ERROR_CONFIG
} KRadioStatus;
typedef enum {
    RADIO_TX_RATE_1200 = 0x01,
    RADIO_TX_RATE_2400 = 0x02,
    RADIO_TX_RATE_4800 = 0x04,
    RADIO_TX_RATE_9600 = 0x08
} RadioTXRate;
typedef enum {
    RADIO_IDLE_UNKNOWN = 0,
    RADIO_IDLE_OFF,
    RADIO_IDLE_ON
} RadioIdleState;
typedef enum {
    RADIO_HARD_RESET,
    RADIO_SOFT_RESET
} KRadioReset;
typedef struct {
    uint16_t interval;
    char *   msg;
    uint8_t  len;
} radio_tx_beacon;
typedef struct {
    uint8_t ascii[6];
    uint8_t ssid;
} ax25_callsign;
typedef struct {
    RadioTXRate     data_rate;
    RadioIdleState  idle;
    radio_tx_beacon beacon;
    ax25_callsign   to;
    ax25_callsign   from;
} radio_config;
typedef struct {
    uint16_t msg_size;
    uint16_t doppler_offset;
    uint16_t signal_strength;
    uint8_t message[RX_MAX_SIZE];
} radio_rx_message;

KRadioStatus k_radio_init(void);
void k_radio_terminate(void);
KRadioStatus k_radio_configure(radio_config * config);
KRadioStatus k_radio_reset(KRadioReset type);
KRadioStatus k_radio_send(char * buffer, int len, uint8_t * response);
KRadioStatus k_radio_recv(radio_rx_message * buffer, uint8_t * len);
