const http = require("http");

async function fetchServerResponse(port, endpoint) {
  return new Promise((resolve, reject) => {
    const options = {
      hostname: "localhost",
      port: port,
      path: endpoint,
      method: "POST",
    };

    const req = http.request(options, (res) => {
      let data = "";

      res.on("data", (chunk) => {
        data += chunk;
      });

      res.on("end", () => {
        resolve({
          statusCode: res.statusCode,
          body: data,
        });
      });
    });

    req.on("error", (e) => {
      reject(e.message);
    });

    req.end();
  });
}

async function compareServers(endpoint) {
  try {
    const response3000 = await fetchServerResponse(3000, endpoint);
    const response8000 = await fetchServerResponse(8000, endpoint);
    console.log(`\nComparing responses for endpoint: ${endpoint}`);

    console.log(`Response from server on port 3000:\n`, response3000);
    console.log(`Response from server on port 8000:\n`, response8000);

    const areResponsesEqual =
      response3000.statusCode === response8000.statusCode &&
      response3000.body === response8000.body;

    if (areResponsesEqual) {
      console.log("\n✅ Responses are identical.");
    } else {
      console.log("\n❌ Responses differ on ");
      console.log("Differences:");
      if (response3000.statusCode !== response8000.statusCode) {
        console.log(
          `- Status code: ${response3000.statusCode} vs ${response8000.statusCode}`
        );
      }
      if (response3000.body !== response8000.body) {
        console.log(
          `- Body content:\n  Port 3000: ${response3000.body}\n  Port 8000: ${response8000.body}`
        );
      }
    }
  } catch (error) {
    console.error("Error fetching responses:", error);
  }
}

// Endpoint to compare
// const endpoint = '/your-endpoint-here';
let endpoints = [
  "/12/reset",
  "/12/place/cookie/1",
  "/12/place/milk/2",
  "/12/place/cookie/2",
  "/12/place/milk/3",
  "/12/place/milk/3",
  "/12/place/cookie/3",
  "/12/reset",
  "/12/place/milk/4",
  "/12/place/milk/4",
  "/12/place/milk/4",
  "/12/place/milk/4",
];
// Call the function
let a = async () => {
  for (let endpoint of endpoints) {
    await compareServers(endpoint);
  }
}
a();

