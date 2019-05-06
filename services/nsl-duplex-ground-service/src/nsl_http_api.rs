//
// Copyright (C) 2019 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use ground_comms_service::CommsResult;
use std::cell::RefCell;
use std::str;

static BASE_URL: &str = "https://data2.nsldata.com/~gsdata/webAPIv1.0";
static DOWNLOAD_URL: &str = "downloadDuplex.php";
static UPLOAD_URL: &str = "uploadDuplex.php";

#[derive(Debug, Serialize, Deserialize)]
struct DownloadItem {
    DT_NSL_RECEIVED: String,
    FileID: String,
    OrigFileName: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DownloadList {
    message: String,
    requestResult: bool,
    results: Vec<DownloadItem>,
}

#[derive(Clone, Debug)]
pub struct RadioConn {
    client: reqwest::Client,
    min_file_id: RefCell<u16>,
    up_count: RefCell<u32>,
    mission_id: u16,
}

impl RadioConn {
    pub fn new(mission_id: u16) -> RadioConn {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        RadioConn {
            client,
            min_file_id: RefCell::new(0),
            up_count: RefCell::new(0),
            mission_id,
        }
    }

    pub fn initialize(&self, user: &str, password: &str) -> CommsResult<()> {
        self.login(user, password).unwrap();
        let mut min_file_id = self.min_file_id.borrow_mut();
        *min_file_id = self.get_min_id() + 1;
        println!("nsl initial file id {}", *min_file_id);
        Ok(())
    }

    fn get_download_list(&self, min_file_id: Option<u16>) -> Vec<DownloadItem> {
        let params = match min_file_id {
            Some(id) => [("MissionID", self.mission_id), ("MinFileID", id)],
            None => [("MissionID", self.mission_id), ("MinFileID", 0)],
        };
        let mut res = self
            .client
            .post(&format!("{}/{}", BASE_URL, DOWNLOAD_URL))
            .form(&params)
            .send()
            .unwrap();
        let json_result: DownloadList = res.json().unwrap();
        json_result.results
    }

    fn download_file(&self, file_id: u16) -> Vec<u8> {
        let _params = [("MissionID", self.mission_id), ("FileID", file_id)];
        let mut res = self
            .client
            .post(&format!(
                "{}/{}?MissionID={}&FileID={}",
                BASE_URL, DOWNLOAD_URL, self.mission_id, file_id
            ))
            .send()
            .unwrap();
        let mut buf: Vec<u8> = vec![];
        res.copy_to(&mut buf).unwrap();
        buf
    }

    fn get_min_id(&self) -> u16 {
        let list = self.get_download_list(None);
        if list.len() > 0 {
            list[list.len() - 1].FileID.parse::<u16>().unwrap()
        } else {
            0
        }
    }

    fn upload_file(&self, file_name: &str, file_body: &[u8]) {
        let form = reqwest::multipart::Form::new();
        let datafile =
            reqwest::multipart::Part::bytes(file_body.to_vec()).file_name(file_name.to_owned());
        let form = form.part("datafile", datafile);
        let req = self
            .client
            .post(&format!("{}/{}", BASE_URL, UPLOAD_URL))
            .query(&[("MissionID", self.mission_id)])
            .multipart(form);

        let _res = req.send().unwrap();
    }

    pub fn login(&self, user: &str, password: &str) -> Result<(), String> {
        let params = [("UserName", user), ("Password", password)];
        let res = self
            .client
            .post(&format!("{}/login.php", BASE_URL))
            .form(&params)
            .send()
            .unwrap();
        if let Some(_cookies) = res.headers().get("set-cookie") {
            Ok(())
        } else {
            Err("Login failed".to_string())
        }
    }
}

impl ground_comms_service::CommsConnection for RadioConn {
    fn read(&self) -> CommsResult<Vec<u8>> {
        let mut min_file_id = self.min_file_id.borrow_mut();
        let download_list = self.get_download_list(Some(*min_file_id));
        if download_list.len() > 0 {
            let item = download_list.first().unwrap();

            let file_id = item.FileID.parse::<u16>().unwrap();
            if item.OrigFileName == "ping" {
                *min_file_id = file_id + 1;
                bail!("Got a ping, skipping");
            } else {
                println!("Downloading file:{}", file_id);
                let payload = self.download_file(file_id);
                println!("Read some packet {:?}", &payload);
                *min_file_id = file_id + 1;
                Ok(payload)
            }
        } else {
            bail!("No data available");
        }
    }

    fn write(&self, data: &[u8]) -> CommsResult<()> {
        let mut up_count = self.up_count.borrow_mut();
        self.upload_file(&format!("up{}", *up_count), data);
        *up_count += 1;
        Ok(())
    }
}
