require('dotenv').config();
const express = require("express");
const cors = require("cors");
const fetch = require("node-fetch");

const app = express();
const PORT = 8787;
const API_KEY = process.env.FAIRSCALE_API_KEY;

app.use(cors());

app.get("/fairscore", async (req, res) => {
  const wallet = req.query.wallet;
  if (!wallet) return res.status(400).json({ error: "Wallet is required" });

  try {
    const url = `https://api2.fairscale.xyz/score?wallet=${wallet}`;
    console.log(`🎬 Consultando FairScale para: ${wallet}`);

    const response = await fetch(url, {
      method: 'GET',
      headers: { 
        'accept': 'application/json',
        'fairkey': API_KEY.trim()
      }
    });

    const data = await response.json();

    if (!response.ok) {
      console.error("❌ Erro FairScale:", data);
      return res.status(response.status).json(data);
    }

    // Retorna os dados completos para o seu site brilhar
    res.json({
      fairscore: data.fairscore || 0,
      tier: data.tier || "Bronze",
      badges: data.badges || []
    });

  } catch (e) {
    console.error("🔥 Erro Crítico:", e.message);
    res.status(500).json({ error: "Server Error" });
  }
});

app.listen(PORT, "0.0.0.0", () => {
  console.log(`🚀 Proxy Online com a Chave Nova!`);
});
