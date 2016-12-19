/*
 * Copyright (C) 2016 Kubos Corporation
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
#ifndef TELEMETRY_CONFIG_H
#define TELEMETRY_CONFIG_H

/* Address used for the current CSP instance */
#define TELEMETRY_CSP_ADDRESS YOTTA_CFG_TELEMETRY_CSP_ADDRESS

/* Port number used for the telemetry server's CSP socket */
#define TELEMETRY_CSP_PORT 10

/* Number of telemetry subscribers */
#define TELEMETRY_NUM_SUBSCRIBERS YOTTA_CFG_TELEMETRY_SUBSCRIBERS_NUM

/* Number of subscriber read attempts */
#define TELEMETRY_SUBSCRIBER_READ_ATTEMPTS 10

#endif