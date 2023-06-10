SetHttpTimeout(1)
local SERVER_URL = "http://your-rust-server.com"

function OnStableStudy(studyId, tags, metadata)
    local url = SERVER_URL .. "/studies/" .. studyId
    local headers = {
        ["Content-Type"] = "application/json"
    }
    local body = "{}" 

    local response, status = HttpPost(url, body, headers)
    if status ~= 200 then
        print("Error sending stable study notification: " .. status)
    end
end

function OnJobSuccess(jobId)
    local url = SERVER_URL .. "/jobs/" .. jobId
    local headers = {
        ["Content-Type"] = "application/json"
    }
    local body = "{}"

    local response, status = HttpPost(url, body, headers)
    if status ~= 200 then
        print("Error sending job success notification: " .. status)
    end
end
