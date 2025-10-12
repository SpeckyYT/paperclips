use core::f64;
use std::{collections::VecDeque, time::Duration};

use rand::{random, random_bool, random_range};

use crate::paperclips_core::{Float, PaperClips};

pub const UPDATE_STOCK_SHOP: Duration = Duration::from_millis(1000);
pub const UPDATE_STOCKS_TIME: Duration = Duration::from_millis(2500);

pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Debug, Clone, Copy)]
pub enum Riskiness {
    Low = 7,
    Medium = 5,
    High = 1,
}

pub struct Stock {
    id: usize,
    symbol: String,
    price: Float,
    amount: u32,
    total: Float,
    profit: Float,
    age: u64,
}

impl Stock {
    pub fn update_total(&mut self) {
        self.total = self.price * self.amount as Float;
    }
}

pub struct Investments {
    pub stocks: VecDeque<Stock>,

    // # portfolioSize
    // Techincally this is just `stocks.len()`.
    // REMOVED for the reason above.
    // pub portfolio_size: usize,

    /// # stockID
    pub stock_index: usize,

    // var secTotal = 0;
    // var sellDelay = 0;

    /// # riskiness
    pub riskiness: Riskiness,
    /// # maxPort
    pub max_port: usize,

    // var m = 0;
    /// # investLevel
    pub invest_level: u32,
    /// # investUpgradeCost
    pub invest_upgrade_cost: Float,

    /// # stockGainThreshold
    pub stock_gain_threshold: Float,
    /// # bankroll
    pub bankroll: Float,
    /// # ledger
    pub ledger: Float,
    /// # portTotal
    pub port_total: Float,

    /// # sellDelay
    pub sell_delay: u8,

    // var stockReportCounter = 0;

    /// # investmentEngineFlag
    pub engine_flag: bool,
}

impl Default for Investments {
    fn default() -> Self {
        Self {
            stocks: VecDeque::with_capacity(10),
            stock_index: 0,
            riskiness: Riskiness::Medium,
            max_port: 5,
            invest_level: 0,
            invest_upgrade_cost: 100.0,
            stock_gain_threshold: 0.5,
            bankroll: 0.0,
            ledger: 0.0,
            port_total: 0.0,
            sell_delay: 0,
            engine_flag: false,
        }
    }
}

impl PaperClips {
    pub fn invest_upgrade(&mut self) {
        // TODO: uncomment and add variables
        let Investments { invest_level, stock_gain_threshold, invest_upgrade_cost, .. } = &mut self.investments;

        self.strategy.yomi -= *invest_upgrade_cost;
        *invest_level += 1;
        *stock_gain_threshold += 0.01;
        *invest_upgrade_cost = (((*invest_level + 1) as Float).powf(f64::consts::E as Float) * 100.0).floor();

        self.messages.push(format!("Investment engine upgraded, expected profit/loss ratio now {stock_gain_threshold:.2?}"));
    }
    pub fn invest_deposit(&mut self) {
        let Investments { bankroll, ledger, .. } = &mut self.investments;

        *ledger -= self.business.funds;
        *bankroll = (*bankroll + self.business.funds).floor();
        self.business.funds = 0.0;
    }
    pub fn invest_withdraw(&mut self) {
        let Investments { bankroll, ledger, .. } = &mut self.investments;
        
        *ledger += *bankroll;
        self.business.funds += *bankroll;
        *bankroll = 0.0;
    }
    pub fn stock_shop(&mut self) {
        let Investments { stocks, bankroll, port_total, riskiness, max_port, .. } = &mut self.investments;
        
        let budget = (*port_total/ *riskiness as u8 as Float).ceil();
        let r = (11 - *riskiness as u8) as Float;
        let reserves = if matches!(riskiness, Riskiness::High) { 0.0 } else { (*port_total/r).ceil() };

        let budget = match (*bankroll - budget < reserves, matches!(riskiness, Riskiness::High), *bankroll > *port_total / 10.0) {
            (true, true, true) => *bankroll,
            (true, true, false) => 0.0,
            (true, _, _) => *bankroll - reserves,
            (_, _, _) => budget,
        };

        if stocks.len() < *max_port && *bankroll >= 5.0 && budget >= 1.0 && *bankroll - budget >= reserves {
            if random_bool(0.25) {
                self.create_stock(budget);
            }
        }
    }
    pub fn create_stock(&mut self, money: Float) {
        let Investments { stocks, stock_index, bankroll, .. } = &mut self.investments;
        *stock_index += 1;

        let roll = random::<Float>();
        let max_price: Float = match roll {
            r if r > 0.99 => 3000.0,
            r if r > 0.85 => 500.0,
            r if r > 0.60 => 150.0,
            r if r > 0.20 => 50.0,
            _ => 15.0,
        };
        let price = random_range(1.0..=max_price).ceil();
        let price = if price > money { money * roll } else { price };

        let amount = (money / price).floor().max(1000000.0);

        stocks.push_back(Stock {
            id: *stock_index,
            symbol: generate_symbol(),
            price,
            amount: amount as u32,
            total: price * amount,
            profit: 0.0,
            age: 0,
        });

        *bankroll -= price * amount;
    }
    pub fn sell_stock(&mut self) {
        let Investments { stocks, bankroll, .. } = &mut self.investments;
        
        if let Some(stock) = stocks.pop_front() {
            *bankroll += stock.total;
        }
    }
    pub fn update_stocks(&mut self) {
        let Investments { stocks, stock_gain_threshold, riskiness, .. } = &mut self.investments;

        for stock in stocks {
            stock.age += 1;
            if random_bool(0.6) {
                let gain = random_bool((*stock_gain_threshold).into());
                
                let delta = (random::<Float>() * stock.price / (4 * *riskiness as u8) as Float).ceil();
                stock.price += if gain { delta } else { -delta };

                if stock.price == 0.0 && random_bool(0.76) {
                    stock.price = 1.0;
                }

                stock.update_total();

                let profit = delta * stock.amount as Float;
                stock.profit += if gain { profit } else { -profit }; 
            }
        }
    }
}

pub fn generate_symbol() -> String {
    let letters = match random::<Float>() {
        r if r <= 0.01 => 1,
        r if r > 0.1 => 2,
        r if r > 0.4 => 3,
        _ => 4,
    };
    (0..=letters).map(|_| ALPHABET[random_range(0..ALPHABET.len())]).collect()
}
