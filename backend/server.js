require('dotenv').config();
const express = require("express");
const cors = require("cors");
const fetch = require("node-fetch");

const app = express();
const PORT = 8787;
const API_KEY = process.env.FAIRSCALE_API_KEY;
const BASE_URL = "https://api2.fairscale.xyz"; 

app.use(cors());

app.get("/fairscore", async (req, res) => {
  const wallet = req.query.wallet;
  if (!wallet) return res.status(400).json({ error: "Wallet is required" });

  try {
    console.log(`🎬 Consultando FairScale para: ${wallet}`);
    const response = await fetch(`${BASE_URL}/score?wallet=${wallet}`, {
      headers: { "fairkey": API_KEY }
    });
    const data = await response.json();
    
    if (!response.ok) return res.status(response.status).json(data);

    res.json({
      fairscore: data.fairscore || 0,
      tier: data.tier || "bronze",
      badges: data.badges || []
    });
  } catch (e) {
    res.status(500).json({ error: "Erro interno no servidor" });
  }
});

app.listen(PORT, "0.0.0.0", () => {
  console.log(`🚀 Proxy MIGI Online na porta ${PORT}`);
});
