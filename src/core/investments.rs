#![allow(clippy::useless_conversion)]

use core::f64;
use std::{collections::VecDeque, time::Duration};

use arrayvec::ArrayString;
use strum::EnumIter;

use crate::{core::{Float, PaperClips}, rng::PCRng};

pub const MAX_STOCKS: usize = 5;
pub const UPDATE_STOCK_SHOP: Duration = Duration::from_millis(1000);
pub const UPDATE_STOCKS_TIME: Duration = Duration::from_millis(2500);

pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Riskiness {
    Low = 7,
    Medium = 5,
    High = 1,
}

impl Riskiness {
    #[inline]
    pub const fn name(&self) -> &str {
        match self {
            Riskiness::Low => "Low Risk",
            Riskiness::Medium => "Med Risk",
            Riskiness::High => "High Risk",
        }
    }
    #[inline]
    pub const fn value(&self) -> u8 {
        match self {
            Riskiness::Low => 7,
            Riskiness::Medium => 5,
            Riskiness::High => 1,
        }
    }
}

pub type Symbol = ArrayString<4>;

#[derive(Debug, Clone, Copy)]
pub struct Stock {
    // these two aren't used
    // pub id: usize,
    // pub age: u64,
    pub symbol: Symbol,
    pub price: Float,
    pub amount: u32,
    pub profit: Float,
}

impl Stock {
    #[inline]
    pub fn total(&self) -> Float {
        self.price * self.amount as Float
    }
}

#[derive(Debug, Clone)]
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
    /// # secTotal
    /// Removed because it's calculated
    // pub sec_total: Float,
    /// # portTotal
    /// Removed because it's calculated
    // pub port_total: Float,

    /// # sellDelay
    pub sell_delay: u32,

    // var stockReportCounter = 0;

    /// # investmentEngineFlag
    pub engine_flag: bool,
}

impl Default for Investments {
    fn default() -> Self {
        Self {
            stocks: VecDeque::with_capacity(5),
            stock_index: 0,
            riskiness: Riskiness::Medium,
            max_port: MAX_STOCKS,
            invest_level: 0,
            invest_upgrade_cost: 100.0,
            stock_gain_threshold: 0.5,
            bankroll: 0.0,
            ledger: 0.0,
            sell_delay: 0,
            engine_flag: false,
        }
    }
}

impl Investments {
    #[inline]
    pub fn sec_total(&self) -> Float {
        self.stocks.iter().map(|s| s.total()).sum()
    }
    #[inline]
    pub fn port_total(&self) -> Float {
        self.bankroll + self.sec_total()
    } 
}

impl PaperClips {
    pub fn invest_upgrade(&mut self) {
        let Investments { invest_level, stock_gain_threshold, invest_upgrade_cost, .. } = &mut self.investments;

        self.strategy.yomi -= *invest_upgrade_cost;
        *invest_level += 1;
        *stock_gain_threshold += 0.01;
        *invest_upgrade_cost = (((*invest_level + 1) as Float).powf(f64::consts::E as Float) * 100.0).floor();

        self.console.push(format!("Investment engine upgraded, expected profit/loss ratio now {stock_gain_threshold:.2?}"));
    }
    pub fn invest_deposit(&mut self) {
        let Investments { bankroll, ledger, .. } = &mut self.investments;

        let amount = self.business.funds.floor();

        *ledger -= amount;
        *bankroll += amount;
        self.business.funds -= amount; // this used to be `= 0.0`, but that makes you waste money each time you click this button
    }
    pub fn invest_withdraw(&mut self) {
        let Investments { bankroll, ledger, .. } = &mut self.investments;
        
        *ledger += *bankroll;
        self.business.funds += *bankroll;
        *bankroll = 0.0;
    }
    pub fn stock_shop(&mut self) {
        let port_total = self.investments.port_total();
        let Investments { stocks, bankroll, riskiness, max_port, .. } = &mut self.investments;
        
        let budget = (port_total / riskiness.value() as Float).ceil();
        let r = (11 - riskiness.value()) as Float;
        let reserves = if *riskiness == Riskiness::High { 0.0 } else { (port_total/r).ceil() };

        let budget = match (*bankroll - budget < reserves, *riskiness == Riskiness::High, *bankroll > port_total / 10.0) {
            (true, true, true) => *bankroll,
            (true, true, false) => 0.0,
            (true, _, _) => *bankroll - reserves,
            (false, _, _) => budget,
        };

        if stocks.len() < *max_port && *bankroll >= 5.0 && budget >= 1.0 && *bankroll - budget >= reserves && self.rng.random_bool_no_best(0.25) {
            self.create_stock(budget);
        }
    }
    pub fn create_stock(&mut self, money: Float) {
        let Investments { stocks, stock_index, bankroll, .. } = &mut self.investments;
        *stock_index += 1;

        let roll = self.rng.random_float(true);
        let max_price: Float = match roll {
            r if r > 0.99 => 3000.0,
            r if r > 0.85 => 500.0,
            r if r > 0.60 => 150.0,
            r if r > 0.20 => 50.0,
            _ => 15.0,
        };
        let price = (self.rng.random_float(true) * max_price).ceil();
        let price = if price > money { money * roll } else { price };

        let amount = (money / price).floor().min(1000000.0);
        let total = price * amount;

        stocks.push_back(Stock {
            symbol: generate_symbol(&mut self.rng),
            price,
            amount: amount as u32,
            profit: 0.0,
        });

        *bankroll -= total;
    }
    pub fn sell_stock(&mut self) {
        let Investments { stocks, bankroll, .. } = &mut self.investments;
        
        if let Some(stock) = stocks.pop_front() {
            *bankroll += stock.total();
        }
    }
    pub fn update_stocks(&mut self) {
        let Investments { stocks, stock_gain_threshold, riskiness, .. } = &mut self.investments;

        for stock in stocks {
            if self.rng.random_bool(0.6, true) {
                let gain = self.rng.random_bool((*stock_gain_threshold).clamp(0.0, 1.0).into(), true);
                
                let delta = (self.rng.random_float(true) * stock.price / (4 * riskiness.value()) as Float).ceil();
                stock.price += if gain { delta } else { -delta };

                if stock.price == 0.0 && self.rng.random_bool(0.76, true) {
                    stock.price = 1.0;
                }

                let profit = delta * stock.amount as Float;
                stock.profit += if gain { profit } else { -profit }; 
            }
        }
    }
}

pub fn generate_symbol(rng: &mut PCRng) -> Symbol {
    let letters = match rng.random_float(false) {
        0.0..=0.01 => 1,
        0.01..=0.1 => 2,
        0.1..=0.4 => 3,
        _ => 4,
    };
    let mut symbol = Symbol::new_const();
    (0..letters).for_each(|_| symbol.push(ALPHABET[(rng.random_float_no_best() * ALPHABET.len() as Float) as usize]));
    symbol
}
