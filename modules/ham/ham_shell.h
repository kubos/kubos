/*
 * KubOS Core Flight Services
 * Copyright (C) 2015 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#ifndef HAM_SHELL_H
#define HAM_SHELL_H

int kiss_init_cmd(int argc, char **argv);
int kiss_send_cmd(int argc, char **argv);
int ax25_getset_addr_cmd(int argc, char **argv);
int ax25ui_pkt_cmd(int argc, char **argv);
int ax25_pkt_cmd(int argc, char **argv);
int ax25_addr_cmd(int argc, char **argv);
int aprs_pos_cmd(int argc, char **argv);
int aprs_tlm_cmd(int argc, char **argv);
void ham_cmd_init(void);

#define HAM_SHELL_COMMANDS \
    { "kiss_init", "Initialize KISS interface", kiss_init_cmd }, \
    { "kiss_send", "Send KISS data", kiss_send_cmd }, \
    { "ax25_dest", "Get/set AX.25 dest callsign", ax25_getset_addr_cmd }, \
    { "ax25_src", "Get/set AX.25 source callsign", ax25_getset_addr_cmd }, \
    { "ax25_digis", "Get/set AX.25 digis", ax25_getset_addr_cmd }, \
    { "ax25_pkt", "Construct an AX.25 packet", ax25_pkt_cmd }, \
    { "ax25ui_pkt", "Construct an AX.25 UI packet", ax25ui_pkt_cmd }, \
    { "ax25_addr", "Encode a callsign into an address", ax25_addr_cmd }, \
    { "aprs_pos", "Construct an APRS position message", aprs_pos_cmd }, \
    { "aprs_tlm", "Construct an APRS telemetry message", aprs_tlm_cmd },

#endif
