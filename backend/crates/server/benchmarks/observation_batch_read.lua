wrk.host    = "localhost"
wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
response = function(status, headers, body)
  if status ~= 200 then
    io.write("Status: ".. status .."\n")
    io.write("Body:\n")
    io.write(body .. "\n")
  end
end
wrk.body = [[
{
  "resourceType": "Bundle",
  "type": "batch",
  "entry": [
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    },
    {
      "request": {
        "method": "GET",
        "url": "Observation/mwmvqsjqnnbaq7c4bvl6d8zuhe"
      }
    }
  ]
}
]]