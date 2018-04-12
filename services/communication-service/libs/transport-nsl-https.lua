local request = require('coro-http').request
local JSON = require 'json'


return function (username, password, mission_name)
  local cookie
  local mission_id
  local min_file_id

  local function login()
    local head, body = request("GET", string.format(
      "https://data2.nsldata.com/~gsdata/webAPIv1.0/login.php?UserName=%s&Password=%s",
      assert(username, 'Missing username arg'),
      assert(password, 'Missing password arg')
    ))
    if head.code ~= 200 then
      error("Unexpected HTTP response: " .. head.code .. ' - ' .. body)
    end
    for i = 1, #head do
      local k, v = unpack(head[i])
      if k:lower() == 'set-cookie' then
        local value = v:match("(PHPSESSID=%w+)")
        if value then
          cookie = {"Cookie", value}
        end
      end
    end
    assert(cookie, "No php session key given")
  end

  local function req(method, url)
    if not cookie then login() end
    local head, body = request(method, url, {
      cookie
    })
    if head.code ~= 200 then
      if body and #body == 0 then body = nil end
      error("Unexpected HTTP response: " .. head.code .. ' - ' .. (body or head.reason))
    end
    return JSON.parse(body)
  end

  local function logout()
    if not cookie then return end
    local res = req('GET', 'https://data2.nsldata.com/~gsdata/webAPIv1.0/logout.php')
    p(res)
    cookie = nil
  end

  local function get_mission()
    if mission_id then return end
    if not cookie then login() end
    local missions = req('GET', 'https://data2.nsldata.com/~gsdata/webAPIv1.0/missions.php')
    for _, v in ipairs(missions.results) do
      if v.MissionName:lower() == mission_name:lower() then
        mission_id = assert(tonumber(v.MissionID))
      end
    end
    assert(mission_id, "No such mission by name")
  end

  local function get_esns()
    if not mission_id then get_mission() end
    local res = req('GET', 'https://data2.nsldata.com/~gsdata/webAPIv1.0/ESNs.php?MissionID=' .. mission_id)
    assert(res.requestResult)
    return res.results
  end

  local function get_min_file_id()
    if not mission_id then get_mission() end
    local res = req('GET', 'https://data2.nsldata.com/~gsdata/fepAPI/downloadDuplex.php?FullListing=DoFullListing&MissionID=' .. mission_id)
    table.sort(res, function (a, b)
      return tonumber(a[1]) > tonumber(b[1])
    end)
    -- TODO: if you don't want to replay the last file, add 1 to result.
    min_file_id = tonumber(res[1][1])
  end

  local function get_new_files()
    if not min_file_id then get_min_file_id() end
    local res = req('GET', 'https://data2.nsldata.com/~gsdata/webAPIv1.0/duplex.php?MissionID=' .. mission_id
       .. '&MinFileID=' .. min_file_id)
    return res.results
  end

  local function download_file(file_id)
    local res = req('GET', 'https://data2.nsldata.com/~gsdata/fepAPI/downloadDuplex.php?MissionID=' .. mission_id
      .. '&FileID=' .. file_id)
    p(res)
  end

  coroutine.wrap(function ()
    local success, message = xpcall(function ()
    for _, meta in ipairs(get_new_files()) do
      p(meta)
      download_file(meta.FileID)
    end

    end, debug.traceback)
    if not success then
      print(message)
    end
  end)()

  local function read()
    print "TODO: implement read"
    coroutine.yield()
  end

  local function write(data)
    print "TODO: implement write"
    p(data)
    coroutine.yield()
  end

  return read, write
end
