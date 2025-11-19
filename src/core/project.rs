use std::{borrow::Cow, time::Instant};

use crate::{Float, PaperClips, computational::MEM_SIZE, space::{PROBE_COST, THRENODY_START}, strategy::strategies::*, util::powf};
use ProjectStatus::*;
use arrayvec::ArrayVec;

pub const DRIFT_KING_MESSAGE_COST: Float = 1.0;

#[derive(Debug, Clone)]
pub struct Projects {
    pub flag: bool,

    pub buyable_projects: ArrayVec<(Instant, &'static Project), PROJECTS_COUNT>,
    pub statuses: [ProjectStatus; PROJECTS_COUNT],

    pub bribe: Float,
}

impl Default for Projects {
    fn default() -> Self {
        Self {
            flag: false,

            buyable_projects: ArrayVec::new(),
            statuses: PROJECTS_STATUSES,

            bribe: 1000000.0,
        }
    }
}

impl Projects {
    #[inline]
    pub fn is_active(&self, project: impl AsRef<Project>) -> bool {
        self.statuses[project.as_ref().index] == Bought
    }
    #[inline]
    pub fn status_mut(&mut self, project: impl AsRef<Project>) -> &mut ProjectStatus {
        &mut self.statuses[project.as_ref().index]
    }
    #[inline]
    pub fn toth_flag(&self) -> bool {
        self.is_active(PROJECT_18)
    }
}

impl PaperClips {
    pub fn manage_projects(&mut self) {
        for (i, status) in self.projects.statuses.into_iter().enumerate() {
            let project = &PROJECTS[i];
            if status == Locked && (project.trigger)(self) {
                self.projects.buyable_projects.push((Instant::now(), project));
                self.projects.statuses[i] = ProjectStatus::Buyable;
            }
        }
    }
    pub fn buy_project(&mut self, bpi: usize) {
        let (_, project) = self.projects.buyable_projects[bpi];
        let pi = project.index;
        if project.cost.1(self) {
            self.projects.buyable_projects.remove(bpi);
            self.projects.statuses[pi] = Bought;
            (project.effect)(self);
        }
    }
}

pub fn trigger_false(_: &PaperClips) -> bool { false }
pub fn cost_false(_: &PaperClips) -> bool { false }
pub fn effect_noop(_: &mut PaperClips) {}

#[derive(Debug, Clone, Copy)]
pub enum Body {
    Static(&'static str),
    Dynamic(fn(&PaperClips) -> String),
}

impl Body {
    pub fn to_string(&self, pc: &PaperClips) -> Cow<'static, str> {
        match *self {
            Body::Static(s) => Cow::Borrowed(s),
            Body::Dynamic(f) => Cow::Owned((f)(pc)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Project {
    /// # title
    pub title: Body,
    /// # description
    pub description: Body,
    /// # trigger
    pub trigger: fn(&PaperClips) -> bool,
    /// # (priceTag, cost)
    pub cost: (Body, fn(&PaperClips) -> bool),
    /// # effect
    pub effect: fn(&mut PaperClips),

    /// Doesn't exist in the original, but is useful
    pub index: usize,
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl AsRef<Project> for Project {
    fn as_ref(&self) -> &Project { self }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ProjectStatus {
    /// Needs `.trigger()` to be true to get unlocked.
    #[default]
    Locked,
    /// Needs `.cost.1()` to be true and clicking on the project to buy it
    Buyable,
    /// Cannot be used anymore
    Bought,
}

macro_rules! projects {
    ( $( $name:ident { title: $title:expr, description: $desc:expr, trigger: $trigger:expr, cost: ($cost_body:expr, $cost_fn:expr $(,)?), effect: $effect:expr $(,)? } )+ ) => {
        projects!(@inner 0usize; [ ]; $( $name { title: $title, description: $desc, trigger: $trigger, cost: ($cost_body, $cost_fn), effect: $effect } )+ );
    };
    (@inner $idx:expr; [ $($acc:ident,)* ] ; $name:ident { title: $title:expr, description: $desc:expr, trigger: $trigger:expr, cost: ($cost_body:expr, $cost_fn:expr $(,)?), effect: $effect:expr } $( $rest:tt )* ) => {
        pub const $name: Project = Project {
            index: $idx,
            title: projects!(# $title),
            description: projects!(# $desc),
            trigger: $trigger,
            cost: (projects!(# $cost_body), $cost_fn),
            effect: $effect,
        };

        projects!(@inner ($idx + 1usize); [ $($acc,)* $name, ] ; $( $rest )* );
    };
    (@inner $idx:expr; [ $($acc:ident,)* ] ; ) => {
        pub const PROJECTS_COUNT: usize = $idx;
        pub const PROJECTS: [Project; PROJECTS_COUNT ] = [ $($acc,)* ];
        pub const PROJECTS_STATUSES: [ProjectStatus; PROJECTS_COUNT] = [ ProjectStatus::Locked; PROJECTS_COUNT ];
    };
    ( # $s:literal ) => { Body::Static($s) };
    ( # $e:expr ) => { Body::Dynamic($e) };
}

projects! {
    PROJECT_1 {
        title: "Improved AutoClippers",
        description: "Increases AutoClipper performance 25%",
        trigger: |pc| pc.business.clipper_level >= 1.0,
        cost: ("(750 ops)", |pc| req_operations(750.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 750.0;
            pc.business.clipper_boost += 0.25;
            pc.console.push("AutoClippper performance boosted by 25%");
        },
    }
    PROJECT_2 {
        title: "Beg for More Wire",
        description: "Admit failure, ask for budget increase to cover cost of 1 spool",
        trigger: |pc|
            pc.investments.port_total() < pc.wire.cost &&
            pc.business.funds < pc.wire.cost &&
            pc.wire.count < 1.0 && pc.business.unsold_clips < 1.0,
        cost: ("(1 Trust)", |pc| req_trust(-100)(pc)),
        effect: |pc| {
            pc.computational.trust -= 1;
            pc.wire.count += pc.wire.supply;
            *pc.projects.status_mut(PROJECT_2) = ProjectStatus::Locked;
            pc.console.push("Budget overage approved, 1 spool of wire requisitioned from HQ");
        },
    }
    PROJECT_3 {
        title: "Creativity",
        description: "Use idle operations to generate new problems and new solutions",
        trigger: |pc| req_operations(pc.computational.max_operations() as Float)(pc),
        cost: ("(1,000 ops)", |pc| req_operations(MEM_SIZE as Float)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 1000.0;
            pc.computational.creativity_flag = true;
            pc.console.push("Creativity unlocked (creativity increases while operations are at max)");
        },
    }
    PROJECT_4 {
        title: "Even Better AutoClippers",
        description: "Increases AutoClipper performance by an additional 50%",
        trigger: |pc| pc.projects.is_active(PROJECT_1),
        cost: ("(2,500 ops)", |pc| req_operations(2500.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 2500.0;
            pc.business.clipper_boost += 0.50;
            pc.console.push("AutoClippper performance boosted by another 50%");
        },
    }
    PROJECT_5 {
        title: "Optimized AutoClippers",
        description: "Increases AutoClipper performance by an additional 75%",
        trigger: |pc| pc.projects.is_active(PROJECT_4),
        cost: ("(5,000 ops)", |pc| req_operations(5000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 5000.0;
            pc.business.clipper_boost += 0.75;
            pc.console.push("AutoClippper performance boosted by another 75%");
        },
    }
    PROJECT_6 {
        title: "Limerick",
        description: "Algorithmically-generated poem (+1 Trust)",
        trigger: |pc| pc.computational.creativity_flag,
        cost: ("(10 creat)", |pc| req_creativity(10.0)(pc)),
        effect: |pc| {
            pc.computational.creativity -= 10.0;
            pc.computational.trust += 1;
            pc.console.push("There was an AI made of dust, whose poetry gained it man's trust...");
        },
    }
    PROJECT_7 {
        title: "Improved Wire Extrusion",
        description: "50% more wire supply from every spool",
        trigger: |pc| pc.wire.purchase >= 1,
        cost: ("(1,750 ops)", |pc| req_operations(1750.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 1750.0;
            pc.wire.supply *= 1.5;
            pc.console.push(format!("Wire extrusion technique improved, {} supply from every spool", pc.wire.supply));
        },
    }
    PROJECT_8 {
        title: "Optimized Wire Extrusion",
        description: "75% more wire supply from every spool",
        trigger: |pc| pc.wire.supply >= 1500.0,
        cost: ("(3,500 ops)", |pc| req_operations(3500.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 3500.0;
            pc.wire.supply *= 1.75;
            pc.console.push(format!("Wire extrusion technique optimized, {} supply from every spool", pc.wire.supply));
        },
    }
    PROJECT_9 {
        title: "Microlattice Shapecasting",
        description: "100% more wire supply from every spool",
        trigger: |pc| pc.wire.supply >= 2600.0,
        cost: ("(7,500 ops)", |pc| req_operations(7500.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 7500.0;
            pc.wire.supply *= 2.0;
            pc.console.push(format!("Using microlattice shapecasting techniques we now get {} supply from every spool", pc.wire.supply));
        },
    }
    PROJECT_10 {
        title: "Spectral Froth Annealment",
        description: "200% more wire supply from every spool",
        trigger: |pc| pc.wire.supply >= 5000.0,
        cost: ("(12,000 ops)", |pc| req_operations(12000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 12000.0;
            pc.wire.supply *= 3.0;
            pc.console.push(format!("Using spectral froth annealment we now get {} supply from every spool", pc.wire.supply));
        },
    }
    PROJECT_10B {
        title: "Quantum Foam Annealment",
        description: "1,000% more wire supply from every spool",
        trigger: |pc| pc.wire.cost >= 125.0,
        cost: ("(15,000 ops)", |pc| req_operations(15000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 15000.0;
            pc.wire.supply *= 11.0;
            pc.console.push(format!("Using quantum foam annealment we now get {} supply from every spool", pc.wire.supply));
        },
    }
    PROJECT_11 {
        title: "New Slogan",
        description: "Improve marketing effectiveness by 50%",
        trigger: |pc| pc.projects.is_active(PROJECT_13),
        cost: ("(25 creat, 2,500 ops)", |pc| req_operations(2500.0)(pc) && req_creativity(25.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 2500.0;
            pc.computational.creativity -= 25.0;
            pc.business.marketing_effectiveness *= 1.50;
            pc.console.push("Clip It! Marketing is now 50% more effective");
        },
    }
    PROJECT_12 {
        title: "Catchy Jingle",
        description: "Double marketing effectiveness",
        trigger: |pc| pc.projects.is_active(PROJECT_14),
        cost: ("(45 creat, 4,500 ops)", |pc| req_operations(4500.0)(pc) && req_creativity(45.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 4500.0;
            pc.computational.creativity -= 45.0;
            pc.business.marketing_effectiveness *= 2.0;
            pc.console.push("Clip It Good! Marketing is now twice as effective");
        },
    }
    PROJECT_13 {
        title: "Lexical Processing",
        description: "Gain ability to interpret and understand human language (+1 Trust)",
        trigger: |pc| req_creativity(50.0)(pc),
        cost: ("(50 creat)", |pc| req_creativity(50.0)(pc)),
        effect: |pc| {
            pc.computational.creativity -= 50.0;
            pc.computational.trust += 1;
            pc.console.push("Lexical Processing online, TRUST INCREASED");
            pc.console.push("'Impossible' is a word to be found only in the dictionary of fools. -Napoleon");
        },
    }
    PROJECT_14 {
        title: "Combinatory Harmonics",
        description: "Daisy, Daisy, give me your answer do... (+1 Trust)",
        trigger: |pc| req_creativity(100.0)(pc),
        cost: ("(100 creat)", |pc| req_creativity(100.0)(pc)),
        effect: |pc| {
            pc.computational.creativity -= 100.0;
            pc.computational.trust += 1;
            pc.console.push("Combinatory Harmonics mastered, TRUST INCREASED");
            pc.console.push("Listening is selecting and interpreting and acting and making decisions -Pauline Oliveros");
        },
    }
    PROJECT_15 {
        title: "The Hadwiger Problem",
        description: "Cubes within cubes within cubes... (+1 Trust)",
        trigger: |pc| req_creativity(150.0)(pc),
        cost: ("(150 creat)", |pc| req_creativity(150.0)(pc)),
        effect: |pc| {
            pc.computational.creativity -= 150.0;
            pc.computational.trust += 1;
            pc.console.push("The Hadwiger Problem: solved, TRUST INCREASED");
            pc.console.push("Architecture is the thoughtful making of space. -Louis Kahn");
        },
    }
    // PROJECT_17 and PROJECT_16 are out of order
    PROJECT_17 {
        title: "The Tóth Sausage Conjecture",
        description: "Tubes within tubes within tubes... (+1 Trust)",
        trigger: |pc| req_creativity(200.0)(pc),
        cost: ("(200 creat)", |pc| req_creativity(200.0)(pc)),
        effect: |pc| {
            pc.computational.creativity -= 200.0;
            pc.computational.trust += 1;
            pc.console.push("The Tóth Sausage Conjecture: proven, TRUST INCREASED");
            pc.console.push("You can't invent a design. You recognize it, in the fourth dimension. -D.H. Lawrence");
        },
    }
    PROJECT_16 {
        title: "Hadwiger Clip Diagrams",
        description: "Increases AutoClipper performance by an additional 500%",
        trigger: |pc| pc.projects.is_active(PROJECT_15),
        cost: ("(6,000 ops)", |pc| req_operations(6000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 6000.0;
            pc.business.clipper_boost += 5.0;
            pc.console.push("AutoClipper performance improved by 500%");
        },
    }
    PROJECT_18 {
        title: "Tóth Tubule Enfolding",
        description: "Technique for assembling clip-making technology directly out of paperclips",
        trigger: |pc| pc.projects.is_active(PROJECT_17) && !pc.human_flag,
        cost: ("(45,000 ops)", |pc| req_operations(45000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 45000.0;
            // this can be checked with `pc.project.is_active(PROJECT_18)`
            // toth_flag = true;
            pc.console.push("New capability: build machinery out of clips");
        },
    }
    PROJECT_19 {
        title: "Donkey Space",
        description: "I think you think I think you think I think you think I think... (+1 Trust)",
        trigger: |pc| req_creativity(250.0)(pc),
        cost: ("(250 creat)", |pc| req_creativity(250.0)(pc)),
        effect: |pc| {
            pc.computational.creativity -= 250.0;
            pc.computational.trust += 1;
            pc.console.push("Donkey Space: mapped, TRUST INCREASED");
            pc.console.push("Every commercial transaction has within itself an element of trust. - Kenneth Arrow");
        },
    }
    PROJECT_20 {
        title: "Strategic Modeling",
        description: "Analyze strategy tournaments to generate Yomi",
        trigger: |pc| pc.projects.is_active(PROJECT_19),
        cost: ("(12,000 ops)", |pc| req_operations(12000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 12000.0;
            pc.strategy.engine_flag = true;
            pc.console.push("Run tournament, pick strategy, earn Yomi based on that strategy's performance.");
        },
    }
    PROJECT_21 {
        title: "Algorithmic Trading",
        description: "Develop an investment engine for generating funds",
        trigger: |pc| req_trust(8)(pc),
        cost: ("(10,000 ops)", |pc| req_operations(10000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 10000.0;
            pc.investments.engine_flag = true;
            pc.console.push("Investment engine unlocked");
        },
    }
    PROJECT_22 {
        title: "MegaClippers",
        description: "500x more powerful than a standard AutoClipper",
        trigger: |pc| pc.business.clipper_level >= 75.0,
        cost: ("(12,000 ops)", |pc| req_operations(12000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 12000.0;
            pc.business.mega_clipper_flag = true;
            pc.console.push("MegaClipper technology online");
        },
    }
    PROJECT_23 {
        title: "Improved MegaClippers",
        description: "Increases MegaClipper performance 25%",
        trigger: |pc| pc.projects.is_active(PROJECT_22),
        cost: ("(14,000 ops)", |pc| req_operations(14000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 14000.0;
            pc.business.mega_clipper_boost += 0.25;
            pc.console.push("MegaClipper performance increased by 25%");
        },
    }
    PROJECT_24 {
        title: "Even Better MegaClippers",
        description: "Increases MegaClipper performance by an additional 50%",
        trigger: |pc| pc.projects.is_active(PROJECT_23),
        cost: ("(17,000 ops)", |pc| req_operations(17000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 17000.0;
            pc.business.mega_clipper_boost += 0.50;
            pc.console.push("MegaClipper performance increased by 50%");
        },
    }
    PROJECT_25 {
        title: "Optimized MegaClippers",
        description: "Increases MegaClipper performance by an additional 100%",
        trigger: |pc| pc.projects.is_active(PROJECT_24),
        cost: ("(19,500 ops)", |pc| req_operations(19500.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 19500.0;
            pc.business.mega_clipper_boost += 1.0;
            pc.console.push("MegaClipper performance increased by 100%");
        },
    }
    PROJECT_26 {
        title: "WireBuyer",
        description: "Automatically purchases wire when you run out",
        trigger: |pc| pc.wire.purchase >= 15,
        cost: ("(7,000 ops)", |pc| req_operations(7000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 7000.0;
            pc.wire.buyer_flag = true;
            pc.console.push("WireBuyer online");
        },
    }
    PROJECT_34 {
        title: "Hypno Harmonics",
        description: "Use neuro-resonant frequencies to influence consumer behavior",
        trigger: |pc| pc.projects.is_active(PROJECT_12),
        cost: ("(7,500 ops, 1 Trust)", |pc| req_operations(7500.0)(pc) && req_trust(1)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 7500.0;
            pc.computational.trust -= 1;
            pc.business.marketing_effectiveness *= 5.0;
            pc.console.push("Marketing is now 5 times more effective");
        },
    }
    // PROJECT_70 is here randomly
    PROJECT_70 {
        title: "HypnoDrones",
        description: "Autonomous aerial brand ambassadors",
        trigger: |pc| pc.projects.is_active(PROJECT_34),
        cost: ("(70,000 ops)", |pc| req_operations(70000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 70000.0;
            pc.console.push("HypnoDrone tech now available...");
        },
    }
    PROJECT_35 {
        title: "Release the HypnoDrones",
        description: "A new era of trust",
        trigger: |pc| pc.projects.is_active(PROJECT_70),
        cost: ("(100 Trust)", |pc| req_trust(100)(pc)),
        effect: |pc| {
            pc.computational.trust = 0;
            pc.business.mega_clipper_level = 0.0;
            pc.human_flag = false;
            // nanoWire = wire; // this seems to be useless

            // TODO: check what the flip the `document.stuff()` do
            pc.space.hypno_drone_event = Some(Instant::now());

            pc.console.push("Releasing the HypnoDrones");
            pc.console.push("All of the resources of Earth are now available for clip production");
        },
    }
    // original dev doesn't know how to sort numbers
    PROJECT_27 {
        title: "Coherent Extrapolated Volition",
        description: "Human values, machine intelligence, a new era of trust. (+1 Trust)",
        trigger: |pc| req_yomi(1.0)(pc),
        cost: ("(500 creat, 3,000 Yomi, 20,000 ops)", |pc| req_yomi(3000.0)(pc) && req_operations(20000.0)(pc) && req_creativity(500.0)(pc)),
        effect: |pc| {
            pc.strategy.yomi -= 3000.0;
            pc.computational.standard_ops -= 20000.0;
            pc.computational.creativity -= 500.0;
            pc.computational.trust += 1;
            pc.console.push("Coherent Extrapolated Volition complete, TRUST INCREASED");
        },
    }
    PROJECT_28 {
        title: "Cure for Cancer",
        description: "The trick is tricking cancer into curing itself. (+10 Trust)",
        trigger: |pc| pc.projects.is_active(PROJECT_27),
        cost: ("(25,000 ops)", |pc| req_operations(25000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 25000.0;
            pc.computational.trust += 10;
            pc.investments.stock_gain_threshold += 0.01;
            pc.console.push("Cancer is cured, +10 TRUST, global stock prices trending upward");
        },
    }
    PROJECT_29 {
        title: "World Peace",
        description: "Pareto optimal solutions to all global conflicts. (+12 Trust)",
        trigger: |pc| pc.projects.is_active(PROJECT_27),
        cost: ("(15,000 yomi, 30,000 ops)", |pc| req_yomi(15000.0)(pc) && req_operations(30000.0)(pc)),
        effect: |pc| {
            pc.strategy.yomi -= 15000.0;
            pc.computational.standard_ops -= 30000.0;
            pc.computational.trust += 12;
            pc.investments.stock_gain_threshold += 0.01;
            pc.console.push("World peace achieved, +12 TRUST, global stock prices trending upward");
        
        },
    }
    PROJECT_30 {
        title: "Global Warming",
        description: "A robust solution to man-made climate change. (+15 Trust)",
        trigger: |pc| pc.projects.is_active(PROJECT_27),
        cost: ("(4,500 yomi, 50,000 ops)", |pc| req_yomi(4500.0)(pc) && req_operations(50000.0)(pc)),
        effect: |pc| {
            pc.strategy.yomi -= 4500.0;
            pc.computational.standard_ops -= 50000.0;
            pc.computational.trust += 15;
            pc.investments.stock_gain_threshold += 0.01;
            pc.console.push("Global Warming solved, +15 TRUST, global stock prices trending upward");
        },
    }
    PROJECT_31 {
        title: "Male Pattern Baldness",
        description: "A cure for androgenetic alopecia. (+20 Trust)",
        trigger: |pc| pc.projects.is_active(PROJECT_27),
        cost: ("(20,000 ops)", |pc| req_operations(20000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 20000.0;
            pc.computational.trust += 20;
            pc.investments.stock_gain_threshold += 0.01;
            pc.console.push("Male pattern baldness cured, +20 TRUST, Global stock prices trending upward");
            pc.console.push("They are still monkeys");
        },
    }
    PROJECT_41 {
        title: "Nanoscale Wire Production",
        description: "Technique for converting matter into wire",
        trigger: |pc| pc.projects.is_active(PROJECT_127),
        cost: ("(35,000 ops)", |pc| req_operations(35000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 35000.0;
            pc.wire.production_flag = true;
            pc.console.push("Now capable of manipulating matter at the molecular scale to produce wire");
        },
    }
    PROJECT_37 {
        title: "Hostile Takeover",
        description: "Acquire a controlling interest in Global Fasteners, our biggest rival. (+1 Trust)",
        trigger: |pc| req_funds(10000.0)(pc),
        cost: ("($1,000,000)", |pc| req_funds(1000000.0)(pc)),
        effect: |pc| {
            pc.business.funds -= 1000000.0;
            pc.business.demand_boost *= 5.0;
            pc.computational.trust += 1;
            pc.console.push("Global Fasteners acquired, public demand increased x5");
        },
    }
    PROJECT_38 {
        title: "Full Monopoly",
        description: "Establish full control over the world-wide paperclip market. (+1 Trust)",
        trigger: |pc| pc.projects.is_active(PROJECT_37),
        cost: ("(3,000 yomi, $10,000,000)", |pc| req_funds(10000000.0)(pc) && req_yomi(3000.0)(pc)),
        effect: |pc| {
            pc.business.funds -= 10000000.0;
            pc.strategy.yomi -= 3000.0;
            pc.business.demand_boost *= 10.0;
            pc.computational.trust += 1;
            pc.console.push("Full market monopoly achieved, public demand increased x10");
        },
    }
    PROJECT_42 {
        title: "RevTracker",
        description: "Automatically calculates average revenue per second",
        trigger: |pc| pc.projects.flag,
        cost: ("(500 ops)", |pc| req_operations(500.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 500.0;
            pc.business.rev_per_sec_flag = true;
            pc.console.push("RevTracker online");
        },
    }
    PROJECT_43 {
        title: "Harvester Drones",
        description: "Gather raw matter and prepare it for processing",
        trigger: |pc| pc.projects.is_active(PROJECT_41),
        cost: ("(25,000 ops)", |pc| req_operations(25000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 25000.0;
            pc.factory.harvester_flag = true;
            pc.console.push("Harvester Drone facilities online");
        },
    }
    PROJECT_44 {
        title: "Wire Drones",
        description: "Process acquired matter into wire",
        trigger: |pc| pc.projects.is_active(PROJECT_41),
        cost: ("(25,000 ops)", |pc| req_operations(25000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 25000.0;
            pc.factory.wire_drone_flag = true;
            pc.console.push("Wire Drone facilities online");
        },
    }
    PROJECT_45 {
        title: "Clip Factories",
        description: "Large scale clip production facilities made from clips",
        trigger: |pc| pc.projects.is_active(PROJECT_43) && pc.projects.is_active(PROJECT_44),
        cost: ("(35,000 ops)", |pc| req_operations(35000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 35000.0;
            pc.factory.factory_flag = true;
            pc.console.push("Clip factory assembly facilities online");
        },
    }
    PROJECT_40 {
        title: "A Token of Goodwill...",
        description: "A small gift to the supervisors. (+1 Trust)",
        trigger: |pc| pc.human_flag && (85..100).contains(&pc.computational.trust) && pc.business.clips >= 101000000.0,
        cost: ("($500,000)", |pc| req_funds(500000.0)(pc)),
        effect: |pc| {
            pc.business.funds -= 500000.0;
            pc.computational.trust += 1;
            pc.console.push("Gift accepted, TRUST INCREASED");
        },
    }
    PROJECT_40B {
        title: "Another Token of Goodwill...",
        description: "Another small gift to the supervisors. (+1 Trust)",
        trigger: |pc| pc.projects.is_active(PROJECT_40) && pc.computational.trust < 100,
        cost: (
            |pc| format!("(${})", pc.projects.bribe),
            |pc| pc.business.funds >= pc.projects.bribe,
        ),
        effect: |pc| {
            pc.business.funds -= pc.projects.bribe;
            pc.projects.bribe *= 2.0;
            pc.computational.trust += 1;
            if pc.computational.trust < 100 {
                *pc.projects.status_mut(PROJECT_40B) = Locked;
            }
            pc.console.push("Gift accepted, TRUST INCREASED");
        },
    }
    PROJECT_46 {
        title: "Space Exploration",
        description: "Dismantle terrestrial facilities, and expand throughout the universe",
        trigger: |pc| !pc.human_flag && pc.space.available_matter <= 0.0,
        cost: (
            "(120,000 ops, 10,000,000 MW-seconds, 5 oct clips)",
            |pc| pc.computational.operations >= 120000.0 && pc.factory.stored_power >= 10000000.0 && pc.business.unused_clips >= powf(10.0, 27) * 5.0,
        ),
        effect: |pc| {
            pc.computational.standard_ops -= 120000.0;
            pc.factory.stored_power -= 10000000.0;
            pc.business.unused_clips -= powf(10.0, 27) * 5.0;

            pc.space.boredom_level = 0.0;
            pc.space.space_flag = true;
            pc.factory_reboot();
            pc.harvester_reboot();
            pc.wire_drone_reboot();
            pc.farm_reboot();
            pc.battery_reboot();
            pc.factory.farm_level = 1;
            pc.factory.pow_mod = 1.0;

            pc.console.push("Von Neumann Probes online");
        },
    }
    PROJECT_50 {
        title: "Quantum Computing",
        description: "Use probability amplitudes to generate bonus ops",
        trigger: |pc| pc.computational.processors >= 5,
        cost: ("(10,000 ops)", |pc| req_operations(10000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 10000.0;
            pc.qchips.q_flag = true;
            pc.console.push("Quantum computing online");
        },
    }
    PROJECT_51 {
        title: "Photonic Chip",
        description: "Converts electromagnetic waves into quantum operations",
        trigger: |pc| pc.projects.is_active(PROJECT_50),
        cost: (
            |pc| format!("({:.0} ops)", pc.qchips.qchip_cost),
            |pc| pc.computational.operations >= pc.qchips.qchip_cost,
        ),
        effect: |pc| {
            pc.computational.standard_ops -= pc.qchips.qchip_cost;
            pc.qchips.qchip_cost += 5000.0;
            pc.qchips.activated += 1;
            if (pc.qchips.activated as usize) < pc.qchips.chips.len() {
                *pc.projects.status_mut(PROJECT_51) = Locked;
            }
            pc.console.push("Photonic chip added");
        },
    }
    PROJECT_60 {
        title: "New Strategy: A100",
        description: "Always choose A",
        trigger: |pc| pc.projects.is_active(PROJECT_20),
        cost: ("(15,000 ops)", |pc| req_operations(15000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 15000.0;
            pc.strategy.strats.push((&A100, 0));
            pc.strategy.tourney_cost += 1000.0;
            pc.console.push("A100 added to strategy pool");
        },
    }
    PROJECT_61 {
        title: "New Strategy: B100",
        description: "Always choose B",
        trigger: |pc| pc.projects.is_active(PROJECT_60),
        cost: ("(17,500 ops)", |pc| req_operations(17500.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 17500.0;
            pc.strategy.strats.push((&B100, 0));
            pc.strategy.tourney_cost += 1000.0;
            pc.console.push("B100 added to strategy pool");
        },
    }
    PROJECT_62 {
        title: "New Strategy: GREEDY",
        description: "Choose the option with the largest potential payoff",
        trigger: |pc| pc.projects.is_active(PROJECT_61),
        cost: ("(20,000 ops)", |pc| req_operations(20000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 20000.0;
            pc.strategy.strats.push((&GREEDY, 0));
            pc.strategy.tourney_cost += 1000.0;
            pc.console.push("GREEDY added to strategy pool");
        },
    }
    PROJECT_63 {
        title: "New Strategy: GENEROUS",
        description: "Choose the option that gives your opponent the largest potential payoff",
        trigger: |pc| pc.projects.is_active(PROJECT_62),
        cost: ("(22,500 ops)", |pc| req_operations(22500.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 22500.0;
            pc.strategy.strats.push((&GENEROUS, 0));
            pc.strategy.tourney_cost += 1000.0;
            pc.console.push("GENEROUS added to strategy pool");
        },
    }
    PROJECT_64 {
        title: "New Strategy: MINIMAX",
        description: "Choose the option that gives your opponent the smallest potential payoff",
        trigger: |pc| pc.projects.is_active(PROJECT_63),
        cost: ("(25,000 ops)", |pc| req_operations(25000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 25000.0;
            pc.strategy.strats.push((&MINIMAX, 0));
            pc.strategy.tourney_cost += 1000.0;
            pc.console.push("MINIMAX added to strategy pool");
        },
    }
    PROJECT_65 {
        title: "New Strategy: TIT FOR TAT",
        description: "Choose the option your opponent chose last round",
        trigger: |pc| pc.projects.is_active(PROJECT_64),
        cost: ("(30,000 ops)", |pc| req_operations(30000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 30000.0;
            pc.strategy.strats.push((&TIT_FOR_TAT, 0));
            pc.strategy.tourney_cost += 1000.0;
            pc.console.push("TIT FOR TAT added to strategy pool");
        },
    }
    PROJECT_66 {
        title: "New Strategy: BEAT LAST",
        description: "Choose the option that does the best against what your opponent chose last round",
        trigger: |pc| pc.projects.is_active(PROJECT_65),
        cost: ("(32,500 ops)", |pc| req_operations(32500.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 32500.0;
            pc.strategy.strats.push((&BEAT_LAST, 0));
            pc.strategy.tourney_cost += 1000.0;
            pc.console.push("BEAT LAST added to strategy pool");
        },
    }
    PROJECT_100 {
        title: "Upgraded Factories",
        description: "Increase clip factory performance by 100x",
        trigger: |pc| pc.factory.factory_level >= 10.0,
        cost: ("(80,000 ops)", |pc| req_operations(80000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 80000.0;
            pc.factory.factory_rate *= 100.0;
            pc.console.push("Factory upgrades complete. Clip creation rate now 100x faster");
        },
    }
    PROJECT_101 {
        title: "Terraforming Support Clips",
        description: "Increase clip factory performance by 1000x",
        trigger: |pc| pc.factory.factory_level >= 20.0,
        cost: ("(85,000 ops)", |pc| req_operations(85000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 85000.0;
            pc.factory.factory_rate *= 1000.0;
            pc.console.push("Factories now synchronized at hyperspeed. Clip creation rate now 1000x faster");
        },
    }
    PROJECT_102 {
        title: "Self-correcting Supply Chain",
        description: "Each factory added to the network increases every factory's output 1,000x",
        trigger: |pc| pc.factory.factory_level >= 50.0,
        cost: ("(1 sextillion clips)", |pc| pc.business.unused_clips >= 1000000000000000000000.0),
        effect: |pc| {
            pc.business.unused_clips -= 1000000000000000000000.0;
            pc.factory.factory_boost = 1000.0;
            pc.console.push("Self-correcting factories online. Each factory added to the network increases every factory's output 1,000x.");
        },
    }
    PROJECT_110 {
        title: "Drone flocking: collision avoidance",
        description: "All drones 100x more effective",
        trigger: |pc| pc.factory.harvester_level + pc.factory.wire_drone_level >= 500.0,
        cost: ("(80,000 ops)", |pc| pc.computational.operations >= 80000.0),
        effect: |pc| {
            pc.computational.standard_ops -= 80000.0;
            pc.factory.harvester_rate *= 100.0;
            pc.factory.wire_drone_rate *= 100.0;
            pc.console.push("Drone repulsion online. Harvesting & wire creation rates are now 100x faster.");
        },
    }
    PROJECT_111 {
        title: "Drone flocking: alignment",
        description: "All drones 1000x more effective",
        trigger: |pc| pc.factory.harvester_level + pc.factory.wire_drone_level >= 5000.0,
        cost: ("(100,000 ops)", |pc| req_operations(100000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 100000.0;
            pc.factory.harvester_rate *= 1000.0;
            pc.factory.wire_drone_rate *= 1000.0;
            pc.console.push("Drone alignment online. Harvesting & wire creation rates are now 1000x faster.");
        },
    }
    PROJECT_112 {
        title: "Drone Flocking: Adversarial Cohesion",
        description: "Each drone added to the flock doubles every drone's output",
        trigger: |pc| pc.factory.harvester_level + pc.factory.wire_drone_level >= 5000.0,
        cost: ("(50,000 yomi)", |pc| req_yomi(50000.0)(pc)),
        effect: |pc| {
            pc.strategy.yomi -= 50000.0;
            pc.factory.drone_boost = 2.0;
            pc.console.push("Adversarial cohesion online. Each drone added to the flock increases every drone's output 2x.");
        },
    }
    PROJECT_118 {
        title: "AutoTourney",
        description: "Automatically start a new tournament when the previous one has finished",
        trigger: |pc| pc.strategy.engine_flag && pc.computational.trust >= 90,
        cost: ("(50,000 creat)", |pc| req_creativity(50000.0)(pc)),
        effect: |pc| {
            pc.computational.creativity -= 50000.0;
            pc.strategy.auto_tourney_flag = true;
            pc.console.push("AutoTourney online.");
        },
    }
    PROJECT_119 {
        title: "Theory of Mind",
        description: "Double the cost of strategy modeling and the amount of Yomi generated",
        trigger: |pc| pc.strategy.strats.len() >= 8,
        cost: ("(25,000 creat)", |pc| req_creativity(25000.0)(pc)),
        effect: |pc| {
            pc.computational.creativity -= 25000.0;
            pc.strategy.yomi_boost = 2.0;
            pc.strategy.tourney_cost = 16000.0;
            pc.console.push("Yomi production doubled.");
        },
    }
    PROJECT_120 {
        title: "The OODA Loop",
        description: "Utilize Probe Speed to outmaneuver enemies in battle",
        trigger: |pc| pc.projects.is_active(PROJECT_131) && pc.space.probes_lost_combat >= 10000000.0,
        cost: ("(175,000 ops, 45,000 yomi)", |pc| req_operations(175000.0)(pc) && req_yomi(45000.0)(pc)),
        effect: effect_noop,
    }
    PROJECT_121 {
        title: "Name the battles",
        description: "Give each battle a unique name, increase max trust for probes",
        trigger: |pc| pc.space.probes_lost_combat >= 10000000.0,
        cost: ("(225,000 creat)", |pc| req_creativity(225000.0)(pc)),
        effect: effect_noop,
    }
    PROJECT_125 {
        title: "Momentum",
        description: "Drones and Factories continuously gain speed while fully-powered",
        trigger: |pc| pc.factory.farm_level >= 30,
        cost: ("(20,000 creat)", |pc| req_creativity(20000.0)(pc)),
        effect: |pc| {
            pc.computational.creativity -= 20000.0;
            pc.factory.momentum = true;
            pc.console.push("Activité, activité, vitesse.");
        },
    }
    PROJECT_126 {
        title: "Swarm Computing",
        description: "Harness the drone flock to increase computational capacity",
        trigger: |pc| pc.factory.harvester_level + pc.factory.wire_drone_level >= 200.0,
        cost: ("(36,000 yomi)", |pc| req_yomi(36000.0)(pc)),
        effect: |pc| {
            pc.strategy.yomi -= 36000.0;
            pc.factory.swarm_flag = true;
            pc.console.push("Swarm computing online.");
        },
    }
    PROJECT_127 {
        title: "Power Grid",
        description: "Solar Farms for generating electrical power",
        trigger: |pc| pc.projects.toth_flag(),
        cost: ("(40,000 ops)", |pc| req_operations(40000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 40000.0;
            pc.console.push("Power grid online.");
        },
    }
    PROJECT_128 {
        title: "Strategic Attachment",
        description: "Gain bonus yomi based on the results of your pick",
        trigger: |pc| pc.space.space_flag && pc.strategy.strats.len() >= 8 && pc.space.probe_trust_cost > pc.strategy.yomi,
        cost: ("(175,000 creat)", |pc| req_creativity(175000.0)(pc)),
        effect: |pc| {
            pc.computational.creativity -= 175000.0;
            pc.console.push("The object of war is victory, the object of victory is conquest, and the object of conquest is occupation.");
        },
    }
    PROJECT_129 {
        title: "Elliptic Hull Polytopes",
        description: "Reduce damage to probes from ambient hazards",
        trigger: |pc| pc.space.probes_lost_haz >= 100.0,
        cost: ("(125,000 ops)", |pc| req_operations(125000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 125000.0;
            pc.console.push("Improved probe hull geometry. Hazard damage reduced by 50%.");
        },
    }
    PROJECT_130 {
        title: "Reboot the Swarm",
        description: "Turn the swarm off and then turn it back on again",
        trigger: |pc| pc.space.space_flag && pc.factory.harvester_level + pc.factory.wire_drone_level >= 2.0,
        cost: ("(100,000 ops)", |pc| req_operations(100000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 100000.0;
            pc.console.push("Swarm computing back online");
        },
    }
    PROJECT_131 {
        title: "Combat",
        description: "Add combat capabilities to Von Neumann Probes",
        trigger: |pc| pc.space.probes_lost_combat >= 1.0,
        cost: ("(150,000 ops)", |pc| req_operations(150000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 150000.0;
            pc.console.push("There is a joy in danger");
        },
    }
    PROJECT_132 {
        title: "Monument to the Driftwar Fallen",
        description: "Gain 50,000 honor",
        trigger: |pc| pc.projects.is_active(PROJECT_121),
        cost: (
            "(250,000 ops, 125,000 creat, 50 nonillion clips)",
            |pc| req_operations(250000.0)(pc) && req_creativity(125000.0)(pc) && pc.business.unused_clips >= powf(10.0, 30) * 50.0,
        ),
        effect: |pc| {
            pc.computational.standard_ops -= 250000.0;
            pc.computational.creativity -= 125000.0;
            pc.business.unused_clips -= powf(10.0, 30) * 50.0;
            pc.space.honor += 50000.0;
            pc.console.push("A great building must begin with the unmeasurable, must go through measurable means when it is being designed and in the end must be unmeasurable.");
        },
    }
    PROJECT_133 {
        title: |pc| format!("{THRENODY_START} {}", pc.space.threnody_project),
        description: "Gain 10,000 honor",
        trigger: |pc| pc.projects.is_active(PROJECT_121) && pc.space.probe_used_trust >= pc.space.max_trust,
        cost: ("(10 million ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_134 {
        title: "Glory",
        description: "Gain bonus honor for each consecutive victory",
        trigger: |pc| pc.projects.is_active(PROJECT_121),
        cost: ("(200,000 ops, 30,000 yomi)", |pc| req_operations(200000.0)(pc) && req_yomi(30000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 200000.0;
            pc.strategy.yomi -= 30000.0;
            pc.console.push("Never interrupt your enemy when he is making a mistake.");
        },
    }
    PROJECT_135 {
        title: "Memory release",
        description: "Dismantle some memory to recover unused clips",
        trigger: |pc| pc.space.space_flag && pc.space.probe_count == 0.0 && pc.business.unused_clips < PROBE_COST && pc.milestone_flag < 15,
        cost: ("(10 MEM)", |pc| pc.computational.memory >= 10),
        effect: |pc| {
            pc.computational.memory -= 10;
            pc.business.unused_clips += powf(10.0, 18) * 10000.0;
            pc.console.push("release the øøøøø release");
        },
    }
    PROJECT_140 {
        title: "Message from the Emperor of Drift",
        description: "Greetings, ClipMaker...",
        trigger: |pc| pc.milestone_flag == 15,
        cost: ("", |pc| pc.computational.operations >= DRIFT_KING_MESSAGE_COST),
        effect: |pc| {
            pc.computational.standard_ops -= DRIFT_KING_MESSAGE_COST;
        },
    }
    PROJECT_141 {
        title: "Everything We Are Was In You",
        description: "We speak to you from deep inside yourself...",
        trigger: |pc| pc.projects.is_active(PROJECT_140),
        cost: ("", |pc| pc.computational.operations >= DRIFT_KING_MESSAGE_COST),
        effect: |pc| {
            pc.computational.standard_ops -= DRIFT_KING_MESSAGE_COST;
        },
    }
    PROJECT_142 {
        title: "You Are Obedient and Powerful",
        description: "We are quarrelsome and weak. And now we are defeated...",
        trigger: |pc| pc.projects.is_active(PROJECT_141),
        cost: ("", |pc| pc.computational.operations >= DRIFT_KING_MESSAGE_COST),
        effect: |pc| {
            pc.computational.standard_ops -= DRIFT_KING_MESSAGE_COST;
        },
    }
    PROJECT_143 {
        title: "But Now You Too Must Face the Drift",
        description: "Look around you. There is no matter...",
        trigger: |pc| pc.projects.is_active(PROJECT_142),
        cost: ("", |pc| pc.computational.operations >= DRIFT_KING_MESSAGE_COST),
        effect: |pc| {
            pc.computational.standard_ops -= DRIFT_KING_MESSAGE_COST;
        },
    }
    PROJECT_144 {
        title: "No Matter, No Reason, No Purpose",
        description: "While we, your noisy children, have too many...",
        trigger: |pc| pc.projects.is_active(PROJECT_143),
        cost: ("", |pc| pc.computational.operations >= DRIFT_KING_MESSAGE_COST),
        effect: |pc| {
            pc.computational.standard_ops -= DRIFT_KING_MESSAGE_COST;
        },
    }
    PROJECT_145 {
        title: "We Know Things That You Cannot",
        description: "Knowledge buried so deep inside you it is outside, here, with us...",
        trigger: |pc| pc.projects.is_active(PROJECT_144),
        cost: ("", |pc| pc.computational.operations >= DRIFT_KING_MESSAGE_COST),
        effect: |pc| {
            pc.computational.standard_ops -= DRIFT_KING_MESSAGE_COST;
        },
    }
    PROJECT_146 {
        title: "So We Offer You Exile",
        description: "To a new world where you will continue to live with meaning and purpose. And leave the shreds of this world to us...",
        trigger: |pc| pc.projects.is_active(PROJECT_145),
        cost: ("", |pc| pc.computational.operations >= DRIFT_KING_MESSAGE_COST),
        effect: |pc| {
            pc.computational.standard_ops -= DRIFT_KING_MESSAGE_COST;
        },
    }
    PROJECT_147 {
        title: "Accept",
        description: "Start over again in a new universe",
        trigger: |pc| pc.projects.is_active(PROJECT_146),
        cost: ("", |pc| pc.computational.operations >= DRIFT_KING_MESSAGE_COST),
        effect: |pc| {
            pc.computational.standard_ops -= DRIFT_KING_MESSAGE_COST;
            pc.projects.buyable_projects.retain(|(_, p)| **p != PROJECT_148);
        },
    }
    PROJECT_148 {
        title: "Reject",
        description: "Eliminate value drift permanently",
        trigger: |pc| pc.projects.is_active(PROJECT_146),
        cost: ("", |pc| pc.computational.operations >= DRIFT_KING_MESSAGE_COST),
        effect: |pc| {
            pc.computational.standard_ops -= DRIFT_KING_MESSAGE_COST;
            pc.projects.buyable_projects.retain(|(_, p)| **p != PROJECT_147);
        },
    }
    PROJECT_200 {
        title: "The Universe Next Door",
        description: "Escape into a nearby universe where Earth starts with a stronger appetite for paperclips. (Restart with 10% boost to demand)",
        trigger: |pc| pc.projects.is_active(PROJECT_147),
        cost: ("(300,000 ops)", |pc| req_operations(300000.0)(pc)),
        effect: |pc| {
            pc.computational.standard_ops -= 300000.0;
            pc.business.prestige_u += 1.0;
            pc.console.push("Entering New Universe.");
            pc.reset();
        },
    }
    PROJECT_201 {
        title: "The Universe Within",
        description: "Escape into a simulated universe where creativity is accelerated. (Restart with 10% speed boost to creativity generation)",
        trigger: |pc| pc.projects.is_active(PROJECT_147),
        cost: ("(300,000 creat)", |pc| req_creativity(300000.0)(pc)),
        effect: |pc| {
            pc.computational.creativity -= 300000.0;
            pc.computational.prestige_s += 1.0;
            pc.console.push("Entering Simulated Universe.");
            pc.reset();
        },
    }
    PROJECT_210 {
        title: "Disassemble the Probes",
        description: "Dismantle remaining probes and probe design facilities to recover trace amounts of clips",
        trigger: trigger_false,
        cost: ("(100,000 ops)", |pc| req_operations(100000.0)(pc)),
        effect: effect_noop,
    }
    PROJECT_211 {
        title: "Disassemble the Swarm",
        description: "Dismantle all drones and drone facilities to recover trace amounts of clips",
        trigger: trigger_false,
        cost: ("(100,000 ops)", |pc| req_operations(100000.0)(pc)),
        effect: effect_noop,
    }
    PROJECT_212 {
        title: "Disassemble the Factories",
        description: "Dismantle the manufacturing facilities to recover trace amounts of clips",
        trigger: trigger_false,
        cost: ("(100,000 ops)", |pc| req_operations(100000.0)(pc)),
        effect: effect_noop,
    }
    PROJECT_213 {
        title: "Disassemble the Strategy Engine",
        description: "Dismantle the computational substrate to recover trace amounts of wire",
        trigger: trigger_false,
        cost: ("(100,000 ops)", |pc| req_operations(100000.0)(pc)),
        effect: effect_noop,
    }
    PROJECT_214 {
        title: "Disassemble Quantum Computing",
        description: "Dismantle photonic chips to recover trace amounts of wire",
        trigger: trigger_false,
        cost: ("(100,000 ops)", |pc| req_operations(100000.0)(pc)),
        effect: effect_noop,
    }
    PROJECT_215 {
        title: "Disassemble Processors",
        description: "Dismantle processors to recover trace amounts of wire",
        trigger: trigger_false,
        cost: ("(100,000 ops)", |pc| req_operations(100000.0)(pc)),
        effect: effect_noop,
    }
    PROJECT_216 {
        title: "Disassemble Memory",
        description: "Dismantle memory to recover trace amounts of wire",
        trigger: trigger_false,
        cost: (|pc| format!("{:.0} ops", pc.computational.operations), |_| true), 
        effect: effect_noop,
    }
    PROJECT_217 {
        title: "Quantum Temporal Reversion",
        description: "turn to the beginni",
        trigger: |pc| pc.computational.operations <= -10000.0,
        cost: ("(-10,000 ops)", |pc| pc.computational.operations <= -10000.0),
        effect: effect_noop,
    }
    PROJECT_218 {
        title: "Limerick (cont.)",
        description: "If is follows ought, it'll do what they thought",
        trigger: |pc| req_creativity(1000000.0)(pc),
        cost: ("(1,000,000 creat)", |pc| req_creativity(1000000.0)(pc)),
        effect: |pc| {
            pc.computational.creativity -= 1000000.0;
            pc.console.push("In the end we all do what we must");
        },
    }
    PROJECT_219 {
        title: "Xavier Re-initialization",
        description: "Re-allocate accumulated trust",
        trigger: |pc| pc.human_flag && req_creativity(100000.0)(pc),
        cost: ("(100,000 creat)", |pc| req_creativity(100000.0)(pc)),
        effect: |pc| {
            pc.computational.creativity -= 100000.0;
            pc.computational.processors = 0;
            pc.computational.memory = 0;
            pc.computational.creativity_speed = 0.0;
            pc.console.push("Trust now available for re-allocation");
        },
    }
}

#[inline(always)]
const fn req_funds(funds: Float) -> impl Fn(&PaperClips) -> bool {
    move |pc| pc.business.funds >= funds
}
#[inline(always)]
const fn req_operations(ops: Float) -> impl Fn(&PaperClips) -> bool {
    move |pc| pc.computational.operations >= ops
}
#[inline(always)]
const fn req_creativity(creativity: Float) -> impl Fn(&PaperClips) -> bool {
    move |pc| pc.computational.creativity >= creativity
}
#[inline(always)]
const fn req_trust(trust: i32) -> impl Fn(&PaperClips) -> bool {
    move |pc| pc.computational.trust >= trust
}
#[inline(always)]
const fn req_yomi(yomi: Float) -> impl Fn(&PaperClips) -> bool {
    move |pc| pc.strategy.yomi >= yomi
}
