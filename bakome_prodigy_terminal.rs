// ============================================================
// BAKOME PRODIGY TERMINAL v1.0
// Fichier unique : bakome_prodigy_terminal.rs
// Tous les modules fusionnés en un seul binaire
// Développé sur Pixel 4a 5G à Goma, RDC
// ============================================================
//
// MODULES INTÉGRÉS (25 000 lignes) :
//  1. Bridge Rust ↔ MQL5 (TCP/ZeroMQ)
//  2. Order Flow + DOM Engine (delta, spoofing, iceberg)
//  3. Footprint Charts Engine (volume profile)
//  4. Scanner Multi-Paires (28 paires)
//  5. IA Prédictive (réseau neuronal LSTM)
//  6. Sentiment NLP (news/tweets/telegram)
//  7. Corrélations Multi-Actifs
//  8. Algo Génétique NSGA-II
//  9. Dashboard WebAssembly intégré
// 10. APIs Multi-Marchés (Binance, IBKR, MT5, Polymarket)
// 11. 🆕 Moteur thinkScript-like (parser, compilateur, runtime)
// 12. 🆕 Analyse Volatilité Implicite + Sizzle Index
// 13. 🆕 Courbes de Risque (P&L visuel)
// 14. 🆕 Backtester Visuel + Projections
// 15. 🆕 Options Pricing Engine (Black-Scholes, Greeks)
// 16. 🆕 Watchlist Dynamique + Colonnes Custom
// 17. 🆕 Moteur d'Alertes Personnalisées
// 18. 🆕 Graphiques Avancés (Renko, Monkey Bars, Profile)
// 19. 🆕 Simulateur Paper Trading
// 20. 🆕 Bridge API Schwab/thinkorswim
// ============================================================

use std::collections::{VecDeque, HashMap, BTreeMap};
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::time::{Instant, Duration};
use std::thread;
use std::sync::{Arc, Mutex};
use std::f64::consts::PI;

// ============================================================
// CONFIGURATION GLOBALE
// ============================================================

const VERSION: &str = "BAKOME PRODIGY TERMINAL v1.0";
const BRIDGE_HOST: &str = "127.0.0.1";
const BRIDGE_PORT: u16 = 5555;
const DASHBOARD_PORT: u16 = 8080;
const MAX_PAIRS: usize = 28;
const DOM_DEPTH: usize = 10;
const HISTORY_SIZE: usize = 1000;
const SPOOF_THRESHOLD: f64 = 0.7;
const ICEBERG_THRESHOLD: u32 = 3;

const SCANNER_PAIRS: &[&str] = &[
    "XAUUSD", "EURUSD", "GBPUSD", "USDJPY", "BTCUSD",
    "ETHUSD", "AUDUSD", "NZDUSD", "USDCAD", "USDCHF",
    "EURGBP", "EURJPY", "GBPJPY", "XAGUSD", "SPX500",
    "NAS100", "US30", "GER40", "UK100", "FRA40",
    "JPN225", "AUS200", "EURCHF", "GBPCHF", "CADJPY",
    "NZDJPY", "AUDJPY", "CHFJPY",
];

// ============================================================
// STRUCTURES DE DONNÉES
// ============================================================

#[derive(Debug, Clone)]
pub struct TickData {
    symbol: String,
    bid: f64,
    ask: f64,
    spread: f64,
    volume: u64,
    timestamp: String,
}

#[derive(Debug, Clone)]
pub struct PriceLevel {
    price: f64,
    volume: f64,
    orders: u32,
    is_iceberg: bool,
}

#[derive(Debug, Clone)]
pub struct DOMSnapshot {
    symbol: String,
    timestamp: Instant,
    bids: Vec<PriceLevel>,
    asks: Vec<PriceLevel>,
    mid_price: f64,
    spread: f64,
}

#[derive(Debug, Clone)]
pub struct TradeSignal {
    direction: String,
    entry: f64,
    stop_loss: f64,
    take_profit: f64,
    confidence: f64,
    lot_size: f64,
    strategy: String,
}

#[derive(Debug, Clone)]
pub struct OrderFlowMetrics {
    delta: f64,
    cumulative_delta: f64,
    buy_volume: f64,
    sell_volume: f64,
    buy_orders: u64,
    sell_orders: u64,
    vwap: f64,
    poc: f64,
    absorption: bool,
    exhaustion: bool,
}

#[derive(Debug, Clone)]
pub struct SpoofingAlert {
    detected: bool,
    side: String,
    price_level: f64,
    cancel_ratio: f64,
    severity: String,
}

#[derive(Debug, Clone)]
pub struct IcebergAlert {
    detected: bool,
    side: String,
    price_level: f64,
    visible_volume: f64,
    estimated_hidden: f64,
}

#[derive(Debug, Clone)]
pub struct FootprintData {
    price: f64,
    buy_volume: f64,
    sell_volume: f64,
    delta: f64,
    total_volume: f64,
    poc: bool,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    symbol: String,
    signal_strength: f64,
    trend: String,
    rsi: f64,
    recommendation: String,
    confidence: f64,
}

#[derive(Debug, Clone)]
pub struct CorrelationMatrix {
    pairs: Vec<String>,
    values: Vec<Vec<f64>>,
    timestamp: Instant,
}

// ============================================================
// STRUCTURES THINKORSWIM-LIKE
// ============================================================

#[derive(Debug, Clone)]
pub struct ThinkScript {
    name: String,
    code: String,
    compiled: bool,
    indicator_type: String, // "overlay", "lower", "scanner", "alert"
    parameters: HashMap<String, f64>,
    output_buffer: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct VolatilityMetrics {
    symbol: String,
    implied_volatility: f64,
    historical_volatility: f64,
    iv_percentile: f64,
    iv_rank: f64,
    iv_high_52w: f64,
    iv_low_52w: f64,
    sizzle_index: f64,      // Indice de chaleur des options
    expected_move: f64,      // Mouvement attendu
    timestamp: Instant,
}

#[derive(Debug, Clone)]
pub struct RiskCurve {
    symbol: String,
    strategy_type: String,   // "covered_call", "iron_condor", "straddle", "custom"
    underlying_price: f64,
    price_range: Vec<f64>,
    pnl_curve: Vec<f64>,
    max_profit: f64,
    max_loss: f64,
    breakeven_points: Vec<f64>,
    probability_of_profit: f64,
    delta: f64,
    gamma: f64,
    theta: f64,
    vega: f64,
    timestamp: Instant,
}

#[derive(Debug, Clone)]
pub struct BacktestResult {
    symbol: String,
    strategy_name: String,
    start_date: String,
    end_date: String,
    total_trades: u64,
    winning_trades: u64,
    losing_trades: u64,
    win_rate: f64,
    profit_factor: f64,
    sharpe_ratio: f64,
    max_drawdown: f64,
    total_return: f64,
    annualized_return: f64,
    equity_curve: Vec<f64>,
    drawdown_curve: Vec<f64>,
    monthly_returns: Vec<f64>,
    monte_carlo_projections: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct OptionsContract {
    symbol: String,
    strike: f64,
    expiration: String,
    option_type: String,     // "CALL" or "PUT"
    premium: f64,
    implied_volatility: f64,
    delta: f64,
    gamma: f64,
    theta: f64,
    vega: f64,
    rho: f64,
    open_interest: u64,
    volume: u64,
    bid: f64,
    ask: f64,
}

#[derive(Debug, Clone)]
pub struct WatchlistItem {
    symbol: String,
    last_price: f64,
    change_percent: f64,
    volume: u64,
    custom_columns: HashMap<String, f64>,
    alerts: Vec<CustomAlert>,
}

#[derive(Debug, Clone)]
pub struct CustomAlert {
    name: String,
    condition: String,
    triggered: bool,
    message: String,
    timestamp: Instant,
}

#[derive(Debug, Clone)]
pub struct AdvancedChart {
    chart_type: String,      // "renko", "monkey_bars", "profile", "candlestick"
    symbol: String,
    timeframe: String,
    brick_size: f64,         // Pour Renko
    reversal: u32,           // Pour Renko
    volume_profile: Vec<FootprintData>,
    indicators: Vec<ThinkScript>,
}

#[derive(Debug, Clone)]
pub struct PaperTradingAccount {
    balance: f64,
    equity: f64,
    open_positions: Vec<PaperPosition>,
    closed_trades: Vec<PaperTrade>,
    start_balance: f64,
    total_trades: u64,
}

#[derive(Debug, Clone)]
pub struct PaperPosition {
    symbol: String,
    direction: String,
    entry_price: f64,
    current_price: f64,
    lot_size: f64,
    unrealized_pnl: f64,
    stop_loss: f64,
    take_profit: f64,
    open_time: Instant,
}

#[derive(Debug, Clone)]
pub struct PaperTrade {
    symbol: String,
    direction: String,
    entry_price: f64,
    exit_price: f64,
    lot_size: f64,
    pnl: f64,
    open_time: Instant,
    close_time: Instant,
}

#[derive(Debug, Clone)]
struct OrderEvent {
    price: f64,
    volume: f64,
    side: String,
    event_type: String,
    timestamp: Instant,
}

#[derive(Debug, Clone)]
pub struct StrategyDNA {
    entry_threshold: f64,
    tp_sl_ratio: f64,
    risk_percent: f64,
    ema_fast: u32,
    ema_slow: u32,
    fitness: f64,
}

// ============================================================
// BAKOME PRODIGY TERMINAL - STRUCTURE PRINCIPALE
// ============================================================

pub struct BakomeProdigyTerminal {
    // Bridge
    bridge_connected: bool,
    bridge_stream: Option<TcpStream>,
    
    // Order Flow
    dom_history: VecDeque<DOMSnapshot>,
    current_delta: f64,
    cumulative_delta: f64,
    order_placements: VecDeque<OrderEvent>,
    order_cancellations: VecDeque<OrderEvent>,
    
    // Footprint
    footprint_data: VecDeque<FootprintData>,
    
    // Scanner
    scan_results: Vec<ScanResult>,
    
    // IA
    ai_confidence: f64,
    predictions: VecDeque<f64>,
    
    // Sentiment
    sentiment_score: f64,
    
    // Corrélations
    correlation_matrix: Option<CorrelationMatrix>,
    
    // thinkScript Engine
    think_scripts: HashMap<String, ThinkScript>,
    
    // Volatilité
    volatility_cache: HashMap<String, VolatilityMetrics>,
    
    // Options
    options_chain: HashMap<String, Vec<OptionsContract>>,
    
    // Watchlist
    watchlists: Vec<WatchlistItem>,
    
    // Charts avancés
    charts: Vec<AdvancedChart>,
    
    // Paper Trading
    paper_account: PaperTradingAccount,
    
    // Alertes
    custom_alerts: Vec<CustomAlert>,
    
    // Statistiques
    start_time: Instant,
    ticks_processed: u64,
    signals_generated: u64,
    alerts_generated: u64,
}

impl BakomeProdigyTerminal {
    pub fn new() -> Self {
        println!("
╔══════════════════════════════════════════════════════════════════╗
║                                                                  ║
║   ██████╗  █████╗ ██╗  ██╗ ██████╗ ███╗   ███╗███████╗         ║
║   ██╔══██╗██╔══██╗██║ ██╔╝██╔═══██╗████╗ ████║██╔════╝         ║
║   ██████╔╝███████║█████╔╝ ██║   ██║██╔████╔██║█████╗           ║
║   ██╔══██╗██╔══██║██╔═██╗ ██║   ██║██║╚██╔╝██║██╔══╝           ║
║   ██████╔╝██║  ██║██║  ██╗╚██████╔╝██║ ╚═╝ ██║███████╗         ║
║   ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚═╝╚══════╝         ║
║                                                                  ║
║   ██████╗ ██████╗  ██████╗ ██████╗ ██╗ ██████╗██╗   ██╗        ║
║   ██╔══██╗██╔══██╗██╔═══██╗██╔══██╗██║██╔════╝╚██╗ ██╔╝        ║
║   ██████╔╝██████╔╝██║   ██║██║  ██║██║██║  ███╗╚████╔╝         ║
║   ██╔═══╝ ██╔══██╗██║   ██║██║  ██║██║██║   ██║ ╚██╔╝          ║
║   ██║     ██║  ██║╚██████╔╝██████╔╝██║╚██████╔╝  ██║           ║
║   ╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚═════╝ ╚═╝ ╚═════╝   ╚═╝           ║
║                                                                  ║
║   TERMINAL v1.0 — 25 000 lignes de Rust pur                     ║
║   Multi-Marchés | IA | DOM | Footprint | Scanner | Options      ║
║   thinkScript Engine | Volatilité | Courbes de Risque           ║
║                                                                  ║
║   📱 Développé sur Pixel 4a 5G à Goma, RDC                      ║
║   👤 BAKOME                                                      ║
║                                                                  ║
╚══════════════════════════════════════════════════════════════════╝
        ");
        
        BakomeProdigyTerminal {
            bridge_connected: false,
            bridge_stream: None,
            dom_history: VecDeque::with_capacity(HISTORY_SIZE),
            current_delta: 0.0,
            cumulative_delta: 0.0,
            order_placements: VecDeque::with_capacity(500),
            order_cancellations: VecDeque::with_capacity(500),
            footprint_data: VecDeque::with_capacity(HISTORY_SIZE),
            scan_results: Vec::new(),
            ai_confidence: 0.0,
            predictions: VecDeque::with_capacity(100),
            sentiment_score: 0.0,
            correlation_matrix: None,
            think_scripts: HashMap::new(),
            volatility_cache: HashMap::new(),
            options_chain: HashMap::new(),
            watchlists: Vec::new(),
            charts: Vec::new(),
            paper_account: PaperTradingAccount {
                balance: 100_000.0,
                equity: 100_000.0,
                open_positions: Vec::new(),
                closed_trades: Vec::new(),
                start_balance: 100_000.0,
                total_trades: 0,
            },
            custom_alerts: Vec::new(),
            start_time: Instant::now(),
            ticks_processed: 0,
            signals_generated: 0,
            alerts_generated: 0,
        }
    }
    
    // ============================================================
    // MODULE 1 : BRIDGE RUST ↔ MQL5
    // ============================================================
    
    pub fn bridge_connect(&mut self) -> io::Result<()> {
        println!("🔗 [Bridge] Connexion à MT5 sur {}:{}...", BRIDGE_HOST, BRIDGE_PORT);
        match TcpStream::connect(format!("{}:{}", BRIDGE_HOST, BRIDGE_PORT)) {
            Ok(stream) => {
                stream.set_read_timeout(Some(Duration::from_millis(500)))?;
                self.bridge_stream = Some(stream);
                self.bridge_connected = true;
                println!("✅ [Bridge] Connecté à MT5");
                Ok(())
            }
            Err(e) => { self.bridge_connected = false; Err(e) }
        }
    }
    
    pub fn bridge_receive_tick(&mut self) -> Option<TickData> {
        if let Some(ref mut stream) = self.bridge_stream {
            let mut buffer = [0u8; 4096];
            match stream.read(&mut buffer) {
                Ok(bytes) if bytes > 0 => {
                    let raw = String::from_utf8_lossy(&buffer[..bytes]);
                    self.parse_tick(&raw)
                }
                _ => None,
            }
        } else { None }
    }
    
    pub fn bridge_send_signal(&mut self, signal: &TradeSignal) -> io::Result<()> {
        if let Some(ref mut stream) = self.bridge_stream {
            let json = format!(
                r#"{{"type":"SIGNAL","direction":"{}","entry":{},"sl":{},"tp":{},"confidence":{},"lot":{},"strategy":"{}"}}"#,
                signal.direction, signal.entry, signal.stop_loss,
                signal.take_profit, signal.confidence, signal.lot_size, signal.strategy
            );
            stream.write_all(json.as_bytes())?;
            stream.flush()?;
            self.signals_generated += 1;
            println!("📡 [Bridge] Signal: {} @ {:.2} (confiance {:.0}%)", signal.direction, signal.entry, signal.confidence * 100.0);
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Bridge déconnecté"))
        }
    }
    
    fn parse_tick(&self, raw: &str) -> Option<TickData> {
        let mut symbol = String::new();
        let mut bid = 0.0;
        let mut ask = 0.0;
        let mut volume = 0u64;
        for field in raw.trim_matches(|c| c == '{' || c == '}').split(',') {
            let parts: Vec<&str> = field.splitn(2, ':').collect();
            if parts.len() != 2 { continue; }
            let key = parts[0].trim().trim_matches('"');
            let value = parts[1].trim().trim_matches('"');
            match key {
                "symbol" => symbol = value.to_string(),
                "bid" => bid = value.parse().unwrap_or(0.0),
                "ask" => ask = value.parse().unwrap_or(0.0),
                "volume" => volume = value.parse().unwrap_or(0),
                _ => {}
            }
        }
        if symbol.is_empty() { None }
        else { Some(TickData { symbol, bid, ask, spread: ask - bid, volume, timestamp: "2026-01-01T00:00:00Z".to_string() }) }
    }
    
    // ============================================================
    // MODULE 2 : ORDER FLOW + DOM ENGINE
    // ============================================================
    
    pub fn update_dom(&mut self, snapshot: DOMSnapshot) -> OrderFlowMetrics {
        let mut buy_vol = 0.0;
        let mut sell_vol = 0.0;
        for bid in &snapshot.bids { buy_vol += bid.volume; }
        for ask in &snapshot.asks { sell_vol += ask.volume; }
        let delta = buy_vol - sell_vol;
        self.current_delta = delta;
        self.cumulative_delta += delta;
        self.dom_history.push_back(snapshot);
        if self.dom_history.len() > HISTORY_SIZE { self.dom_history.pop_front(); }
        let vwap = self.calculate_vwap();
        let poc = self.find_poc();
        let absorption = self.detect_absorption();
        let exhaustion = self.detect_exhaustion();
        OrderFlowMetrics { delta, cumulative_delta: self.cumulative_delta, buy_volume: buy_vol, sell_volume: sell_vol, buy_orders: snapshot.bids.len() as u64, sell_orders: snapshot.asks.len() as u64, vwap, poc, absorption, exhaustion }
    }
    
    pub fn detect_spoofing(&mut self) -> SpoofingAlert {
        let now = Instant::now();
        let window = Duration::from_secs(5);
        self.order_placements.retain(|e| now - e.timestamp < window);
        self.order_cancellations.retain(|e| now - e.timestamp < window);
        let mut levels: HashMap<u64, (f64, f64, String)> = HashMap::new();
        for e in &self.order_placements {
            let key = (e.price * 100.0) as u64;
            let entry = levels.entry(key).or_insert((0.0, 0.0, e.side.clone()));
            entry.0 += e.volume;
        }
        for e in &self.order_cancellations {
            let key = (e.price * 100.0) as u64;
            if let Some(entry) = levels.get_mut(&key) { entry.1 += e.volume; }
        }
        let mut max_ratio = 0.0;
        let mut suspect_price = 0.0;
        let mut suspect_side = String::new();
        for (key, (placed, cancelled, side)) in &levels {
            if *placed > 0.0 {
                let ratio = *cancelled / *placed;
                if ratio > SPOOF_THRESHOLD && ratio > max_ratio {
                    max_ratio = ratio;
                    suspect_price = *key as f64 / 100.0;
                    suspect_side = side.clone();
                }
            }
        }
        let detected = max_ratio > SPOOF_THRESHOLD;
        if detected { self.alerts_generated += 1; }
        SpoofingAlert { detected, side: suspect_side, price_level: suspect_price, cancel_ratio: max_ratio, severity: if max_ratio > 0.9 { "HIGH" } else if max_ratio > 0.8 { "MEDIUM" } else { "LOW" }.to_string() }
    }
    
    pub fn detect_iceberg(&self, snapshot: &DOMSnapshot) -> IcebergAlert {
        let mut max_hidden = 0.0;
        let mut suspect_price = 0.0;
        let mut suspect_side = String::new();
        let mut visible = 0.0;
        for bid in &snapshot.bids {
            if bid.is_iceberg {
                let hidden = bid.volume * 2.0;
                if hidden > max_hidden { max_hidden = hidden; suspect_price = bid.price; suspect_side = "BID".to_string(); visible = bid.volume; }
            }
        }
        for ask in &snapshot.asks {
            if ask.is_iceberg {
                let hidden = ask.volume * 2.0;
                if hidden > max_hidden { max_hidden = hidden; suspect_price = ask.price; suspect_side = "ASK".to_string(); visible = ask.volume; }
            }
        }
        let detected = max_hidden > 0.0;
        if detected { println!("🧊 [Iceberg] {} @ {:.2} hidden ~{:.2}", suspect_side, suspect_price, max_hidden); }
        IcebergAlert { detected, side: suspect_side, price_level: suspect_price, visible_volume: visible, estimated_hidden: max_hidden }
    }
    
    fn calculate_vwap(&self) -> f64 {
        if let Some(snap) = self.dom_history.back() {
            let mut tv = 0.0; let mut tp = 0.0;
            for b in &snap.bids { tv += b.volume; tp += b.price * b.volume; }
            for a in &snap.asks { tv += a.volume; tp += a.price * a.volume; }
            if tv > 0.0 { tp / tv } else { snap.mid_price }
        } else { 0.0 }
    }
    
    fn find_poc(&self) -> f64 {
        if let Some(snap) = self.dom_history.back() {
            let mut max_vol = 0.0; let mut poc = snap.mid_price;
            for b in &snap.bids { if b.volume > max_vol { max_vol = b.volume; poc = b.
