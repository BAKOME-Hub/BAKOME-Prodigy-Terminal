
<p align="center">
  <img src="https://via.placeholder.com/800x400/0a0a0a/00ff88?text=BAKOME+PRODIGY+TERMINAL+25K+Rust+thinkScript+Options" alt="BAKOME PRODIGY TERMINAL" width="100%">
</p>

---

## 📖 Description

**🇫🇷 Français**
BAKOME PRODIGY TERMINAL est un terminal de trading quantique monolithique écrit en Rust, dépassant 25 000 lignes dans un seul fichier. Il fusionne 18 modules avancés : bridge MetaTrader 5, Order Flow avec spoofing/iceberg, Footprint Charts, scanner 28 paires, IA neuronale, NLP sentiment, corrélations multi-actifs, algorithme génétique NSGA-II, moteur thinkScript-like complet, volatilité implicite avec Sizzle Index, courbes de risque P&L, pricing d'options Black-Scholes avec Greeks, backtester visuel Monte Carlo, graphiques Renko/Profile, alertes personnalisées, watchlist dynamique, paper trading, et dashboard web. Compilé en moins de 15 Mo, tourne sur un Pixel 4a 5G. Développé à Goma, RDC.

**🇬🇧 English**
BAKOME PRODIGY TERMINAL is a monolithic quantum trading terminal written in Rust, exceeding 25,000 lines in a single file. It fuses 18 advanced modules: MetaTrader 5 bridge, Order Flow with spoofing/iceberg, Footprint Charts, 28-pair scanner, neural AI, NLP sentiment, multi-asset correlations, NSGA-II genetic algorithm, complete thinkScript-like engine, implied volatility with Sizzle Index, P&L risk curves, Black-Scholes options pricing with Greeks, Monte Carlo visual backtester, Renko/Profile charts, custom alerts, dynamic watchlist, paper trading, and web dashboard. Compiles under 15 MB, runs on a Pixel 4a 5G. Built in Goma, DRC.

**🇪🇸 Español**
BAKOME PRODIGY TERMINAL es un terminal de trading cuántico monolítico escrito en Rust, superando 25,000 líneas en un solo archivo. Fusiona 18 módulos avanzados: bridge MetaTrader 5, Order Flow con spoofing/iceberg, Footprint Charts, escáner 28 pares, IA neuronal, sentimiento NLP, correlaciones multi-activos, algoritmo genético NSGA-II, motor thinkScript-like completo, volatilidad implícita con Sizzle Index, curvas de riesgo P&L, pricing de opciones Black-Scholes con Greeks, backtester visual Monte Carlo, gráficos Renko/Profile, alertas personalizadas, watchlist dinámica, paper trading, y dashboard web. Compila en menos de 15 MB, ejecuta en Pixel 4a 5G. Desarrollado en Goma, RDC.

---

## ⚡ Modules / Features / Módulos

| Module | Description |
|--------|-------------|
| 🔗 **Bridge Rust ↔ MQL5** | Communication ultra-rapide avec MetaTrader 5 |
| 📊 **Order Flow + DOM** | Delta, déséquilibres, carnet d'ordres temps réel |
| 🕵️ **Détecteur Spoofing** | Fausses murailles d'ordres institutionnelles |
| 🧊 **Détecteur Iceberg** | Ordres cachés des grandes mains |
| 👣 **Footprint Charts** | Volume profile, delta acheteur/vendeur, POC |
| 🔍 **Scanner 28 Paires** | Forex, Crypto, Indices simultanément |
| 🧠 **IA Prédictive** | Réseau neuronal pour prédire les mouvements |
| 📰 **Sentiment NLP** | Analyse des news, tweets en temps réel |
| 🔗 **Corrélations Multi-Actifs** | XAUUSD vs DXY vs BTC vs SPX |
| 🧬 **Algo Génétique NSGA-II** | Stratégies qui évoluent automatiquement |
| 🧠 **thinkScript Engine** | Parser, compilateur, runtime pour scripts customs |
| 📊 **Volatilité + Sizzle Index** | IV, HV, rang centile, activité options inhabituelle |
| 📈 **Courbes de Risque P&L** | Profits/pertes visuels, probabilité de profit |
| ⏪ **Backtester Visuel** | Monte Carlo, equity curve, drawdown |
| 🧮 **Options Pricing** | Black-Scholes, Delta, Gamma, Theta, Vega, Rho |
| 📋 **Watchlist Dynamique** | Colonnes custom, prix temps réel |
| 🔔 **Alertes Personnalisées** | Conditions configurables par symbole |
| 🎨 **Graphiques Avancés** | Renko, Monkey Bars, Volume Profile |
| 🧪 **Paper Trading** | Simulateur complet avec P&L tracking |
| 💻 **Dashboard Web** | Interface live sur port 8080 |

---

## 🌍 Marchés Supportés

| Marché | Plateformes | Actifs |
|--------|-------------|--------|
| 🪙 **Crypto Spot & Futures** | Binance, Bybit, OKX, Hyperliquid | BTC, ETH, SOL |
| 📈 **Forex & Métaux** | MetaTrader 5 (bridge natif) | XAUUSD, EURUSD, 28 paires |
| 📊 **Actions & ETFs** | Interactive Brokers, Alpaca | SPX, NASDAQ |
| 🎯 **Marchés Prédictifs** | Polymarket | Événements tokenisés |
| 📋 **Options** | Black-Scholes Engine intégré | CALL/PUT, Greeks |

---

## ⚙️ Quick Install

```bash
# 1. Cloner le repo
git clone https://github.com/BAKOME-Hub/BAKOME-Prodigy-Terminal.git
cd BAKOME-Prodigy-Terminal

# 2. Compiler (Rust requis)
cargo build --release

# 3. Lancer
./target/release/bakome_prodigy_terminal

# 4. Dashboard → http://localhost:8080
