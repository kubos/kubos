/*
 * Copyright (C) 2018 Kubos Corporation
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

use serde_json;

#[derive(Serialize)]
pub enum Config {
    /// Select MTM to use for measurement. 0 - Internal, 1 - External
    #[serde(rename = "0x2002")]
    Select(u8),
    /// Integration time selection for idle mode MTM measurements. <em>Refer to Table 3-10 of the iMTQ User Manual for more information</em>
    #[serde(rename = "0x2003")]
    InternalIntegrationTime(u8),
    /// Integration time selection for idle mode MTM measurements. <em>Refer to Table 3-10 of the iMTQ User Manual for more information</em>
    #[serde(rename = "0x2004")]
    ExternalIntegrationTime(u8),
    /// iMTQ axis to which the MTM x-axis is mapped
    #[serde(rename = "0x2005")]
    InternalMapX(u8),
    /// iMTQ axis to which the MTM y-axis is mapped
    #[serde(rename = "0x2006")]
    InternalMapY(u8),
    /// iMTQ axis to which the MTM z-axis is mapped
    #[serde(rename = "0x2007")]
    InternalMapZ(u8),
    /// iMTQ axis to which the MTM x-axis is mapped
    #[serde(rename = "0x2008")]
    ExternalMapX(u8),
    /// iMTQ axis to which the MTM y-axis is mapped
    #[serde(rename = "0x2009")]
    ExternalMapY(u8),
    /// iMTQ axis to which the MTM z-axis is mapped
    #[serde(rename = "0x200A")]
    ExternalMapZ(u8),
    /// MTM raw -> corrected correction matrix (Row 1, Column 1)
    #[serde(rename = "0xA001")]
    MatrixR1C1(u8),
    /// MTM raw -> corrected correction matrix (Row 1, Column 2)
    #[serde(rename = "0xA002")]
    MatrixR1C2(u8),
    /// MTM raw -> corrected correction matrix (Row 1, Column 3)
    #[serde(rename = "0xA003")]
    MatrixR1C3(u8),
    /// MTM raw -> corrected correction matrix (Row 2, Column 1)
    #[serde(rename = "0xA004")]
    MatrixR2C1(u8),
    /// MTM raw -> corrected correction matrix (Row 2, Column 2)
    #[serde(rename = "0xA005")]
    MatrixR2C2(u8),
    /// MTM raw -> corrected correction matrix (Row 2, Column 3)
    #[serde(rename = "0xA006")]
    MatrixR2C3(u8),
    /// MTM raw -> corrected correction matrix (Row 3, Column 1)
    #[serde(rename = "0xA007")]
    MatrixR3C1(u8),
    /// MTM raw -> corrected correction matrix (Row 3, Column 2)
    #[serde(rename = "0xA008")]
    MatrixR3C2(u8),
    /// MTM raw -> corrected correction matrix (Row 3, Column 3)
    #[serde(rename = "0xA009")]
    MatrixR3C3(u8),
    /// MTM raw -> corrected correction bias vector (X-axis value)
    #[serde(rename = "0xA00A")]
    BiasX(u8),
    /// MTM raw -> corrected correction bias vector (Y-axis value)
    #[serde(rename = "0xA00B")]
    BiasY(u8),
    /// MTM raw -> corrected correction bias vector (Z-axis value)
    #[serde(rename = "0xA00C")]
    BiasZ(u8),
}

impl Config {
    pub fn as_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_select() {
        assert_eq!("{\"0x2002\":1}".to_string(), Config::Select(1).as_json());
    }

    #[test]
    fn test_serialize_internal_integration_time() {
        assert_eq!(
            "{\"0x2003\":0}".to_string(),
            Config::InternalIntegrationTime(0).as_json()
        );
    }

    #[test]
    fn test_serialize_external_integration_time() {
        assert_eq!(
            "{\"0x2004\":2}".to_string(),
            Config::ExternalIntegrationTime(2).as_json()
        );
    }

    #[test]
    fn test_serialize_internal_map_x() {
        assert_eq!(
            "{\"0x2005\":3}".to_string(),
            Config::InternalMapX(3).as_json()
        );
    }

    #[test]
    fn test_serialize_internal_map_y() {
        assert_eq!(
            "{\"0x2006\":1}".to_string(),
            Config::InternalMapY(1).as_json()
        );
    }

    #[test]
    fn test_serialize_internal_map_z() {
        assert_eq!(
            "{\"0x2007\":5}".to_string(),
            Config::InternalMapZ(5).as_json()
        );
    }

    #[test]
    fn test_serialize_external_map_x() {
        assert_eq!(
            "{\"0x2008\":0}".to_string(),
            Config::ExternalMapX(0).as_json()
        );
    }

    #[test]
    fn test_serialize_external_map_y() {
        assert_eq!(
            "{\"0x2009\":1}".to_string(),
            Config::ExternalMapY(1).as_json()
        );
    }

    #[test]
    fn test_serialize_external_map_z() {
        assert_eq!(
            "{\"0x200A\":2}".to_string(),
            Config::ExternalMapZ(2).as_json()
        );
    }

    #[test]
    fn test_serialize_matrix() {
        assert_eq!(
            "{\"0xA001\":1}".to_string(),
            Config::MatrixR1C1(1).as_json()
        );
        assert_eq!(
            "{\"0xA002\":1}".to_string(),
            Config::MatrixR1C2(1).as_json()
        );
        assert_eq!(
            "{\"0xA003\":1}".to_string(),
            Config::MatrixR1C3(1).as_json()
        );
        assert_eq!(
            "{\"0xA004\":1}".to_string(),
            Config::MatrixR2C1(1).as_json()
        );
        assert_eq!(
            "{\"0xA005\":1}".to_string(),
            Config::MatrixR2C2(1).as_json()
        );
        assert_eq!(
            "{\"0xA006\":1}".to_string(),
            Config::MatrixR2C3(1).as_json()
        );
        assert_eq!(
            "{\"0xA007\":1}".to_string(),
            Config::MatrixR3C1(1).as_json()
        );
        assert_eq!(
            "{\"0xA008\":1}".to_string(),
            Config::MatrixR3C2(1).as_json()
        );
        assert_eq!(
            "{\"0xA009\":1}".to_string(),
            Config::MatrixR3C3(1).as_json()
        );
    }

    #[test]
    fn test_serialize_bias() {
        assert_eq!("{\"0xA00A\":0}".to_string(), Config::BiasX(0).as_json());
        assert_eq!("{\"0xA00B\":0}".to_string(), Config::BiasY(0).as_json());
        assert_eq!("{\"0xA00C\":0}".to_string(), Config::BiasZ(0).as_json());
    }
}
