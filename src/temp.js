const fetchF = async () => {
  const resp = await fetch("http://localhost:8000/9/milk", {
    method: "POST",
  });
//   if(resp.status === 429) {
//     let _  = await fetch("http://localhost:8000/9/refill", {
//         method: "POST",
//     })
//   }
  console.log("resp", resp.status);
};
const temp = async () => {
//   for (let i = 0; i < 20; i++) {
    let f = setInterval(() => {
      fetchF();
    }, 200);
    setTimeout(() => {
      clearInterval(f);
    }, 10000);
//   }
};

temp();
