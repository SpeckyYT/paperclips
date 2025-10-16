use crate::PaperClips;

pub struct Projects {
    pub statuses: [ProjectStatus; PROJECTS.len()],
}

impl Default for Projects {
    fn default() -> Self {
        Self {
            statuses: PROJECTS_STATUSES,
        }
    }
}

use ProjectStatus::*;
impl PaperClips {
    pub fn manage_projects(&mut self) {
        for (i, status) in self.projects.statuses.into_iter().enumerate() {
            let project = &PROJECTS[i];
            match status {
                Unavailable => {
                    if (project.trigger)(&self) {
                        self.projects.statuses[i] = Buyable;
                    }
                }
                _ => {}
            }
        }
    }
    pub fn buy_project(&mut self, i: usize) {
        let project = &PROJECTS[i];
        if matches!(self.projects.statuses[i], Buyable) && project.cost.1(&self) {
            (project.effect)(self);
            self.projects.statuses[i] = Active;
        }
    }
}

pub fn trigger_false(_: &PaperClips) -> bool { false }
pub fn cost_false(_: &PaperClips) -> bool { false }
pub fn effect_noop(_: &mut PaperClips) {}

#[derive(Clone, Copy)]
pub enum Body {
    Static(&'static str),
    Dynamic(fn(&PaperClips) -> String),
}

impl Body {
    pub fn to_string(&self, paperclips: &PaperClips) -> String {
        match self {
            Body::Static(s) => s.to_string(),
            Body::Dynamic(f) => (f)(paperclips),
        }
    }
}

#[derive(Clone, Copy)]
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
}

#[derive(Clone, Copy, Default)]
pub enum ProjectStatus {
    #[default]
    Unavailable,
    Buyable,
    Active,
}


macro_rules! projects {
    ( $( $name:ident { title: $title:expr, description: $desc:expr, trigger: $trigger:expr, cost: ($cost_body:expr, $cost_fn:expr), effect: $effect:expr $(,)? } )+ ) => {
        $(
            pub const $name: Project = Project {
                title: projects!(# $title),
                description: projects!(# $desc),
                trigger: $trigger,
                cost: (projects!(# $cost_body), $cost_fn),
                effect: $effect,
            };
        )*
        pub const PROJECTS: [Project; [$($effect),*].len()] = [ $( $name ),+ ];
        pub const PROJECTS_STATUSES: [ProjectStatus; PROJECTS.len()] = [ ProjectStatus::Unavailable; PROJECTS.len() ];
    };
    ( # $s:literal ) => { Body::Static($s) };
    ( # $e:expr ) => { Body::Dynamic($e) };
}

projects! {
    PROJECT_1 {
        title: "Improved AutoClippers",
        description: "Increases AutoClipper performance 25%",
        trigger: trigger_false,
        cost: ("(750 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_2 {
        title: "Beg for More Wire",
        description: "Admit failure, ask for budget increase to cover cost of 1 spool",
        trigger: trigger_false,
        cost: ("(1 Trust)", cost_false),
        effect: effect_noop,
    }
    PROJECT_3 {
        title: "Creativity",
        description: "Use idle operations to generate new problems and new solutions",
        trigger: trigger_false,
        cost: ("(1,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_4 {
        title: "Even Better AutoClippers",
        description: "Increases AutoClipper performance by an additional 50%",
        trigger: trigger_false,
        cost: ("(2,500 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_5 {
        title: "Optimized AutoClippers",
        description: "Increases AutoClipper performance by an additional 75%",
        trigger: trigger_false,
        cost: ("(5,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_6 {
        title: "Limerick",
        description: "Algorithmically-generated poem (+1 Trust)",
        trigger: trigger_false,
        cost: ("(10 creat)", cost_false),
        effect: effect_noop,
    }
    PROJECT_7 {
        title: "Improved Wire Extrusion",
        description: "50% more wire supply from every spool",
        trigger: trigger_false,
        cost: ("(1,750 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_8 {
        title: "Optimized Wire Extrusion",
        description: "75% more wire supply from every spool",
        trigger: trigger_false,
        cost: ("(3,500 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_9 {
        title: "Microlattice Shapecasting",
        description: "100% more wire supply from every spool",
        trigger: trigger_false,
        cost: ("(7,500 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_10 {
        title: "Spectral Froth Annealment",
        description: "200% more wire supply from every spool",
        trigger: trigger_false,
        cost: ("(12,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_10B {
        title: "Quantum Foam Annealment",
        description: "1,000% more wire supply from every spool",
        trigger: trigger_false,
        cost: ("(15,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_11 {
        title: "New Slogan",
        description: "Improve marketing effectiveness by 50%",
        trigger: trigger_false,
        cost: ("(25 creat, 2,500 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_12 {
        title: "Catchy Jingle",
        description: "Double marketing effectiveness",
        trigger: trigger_false,
        cost: ("(45 creat, 4,500 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_13 {
        title: "Lexical Processing",
        description: "Gain ability to interpret and understand human language (+1 Trust)",
        trigger: trigger_false,
        cost: ("(50 creat)", cost_false),
        effect: effect_noop,
    }
    PROJECT_14 {
        title: "Combinatory Harmonics",
        description: "Daisy, Daisy, give me your answer do... (+1 Trust)",
        trigger: trigger_false,
        cost: ("(100 creat)", cost_false),
        effect: effect_noop,
    }
    PROJECT_15 {
        title: "The Hadwiger Problem",
        description: "Cubes within cubes within cubes... (+1 Trust)",
        trigger: trigger_false,
        cost: ("(150 creat)", cost_false),
        effect: effect_noop,
    }
    PROJECT_16 {
        title: "Hadwiger Clip Diagrams",
        description: "Increases AutoClipper performance by an additional 500%",
        trigger: trigger_false,
        cost: ("(6,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_18 {
        title: "Tóth Tubule Enfolding",
        description: "Technique for assembling clip-making technology directly out of paperclips",
        trigger: trigger_false,
        cost: ("(45,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_19 {
        title: "Public Relations Campaign",
        description: "Bribe the press to make clips look better (+Trust)",
        trigger: trigger_false,
        cost: ("(1,000 funds)", cost_false),
        effect: effect_noop,
    }
    PROJECT_20 {
        title: "Wire Recycling Initiative",
        description: "Recover wire from unsold clips; reduces waste and increases supply",
        trigger: trigger_false,
        cost: ("(500 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_21 {
        title: "Subsidiary Automata",
        description: "Build simple bots to assist in clipping operations",
        trigger: trigger_false,
        cost: ("(2,500 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_22 {
        title: "Ad Campaign: 'Clip Now'",
        description: "Short-term boost to marketing effectiveness",
        trigger: trigger_false,
        cost: ("(750 funds)", cost_false),
        effect: effect_noop,
    }
    PROJECT_23 {
        title: "Market Research Study",
        description: "Improve demand forecasting; small permanent demand boost",
        trigger: trigger_false,
        cost: ("(1,500 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_24 {
        title: "Factory Expansion",
        description: "Add facility capacity to increase clip production rates",
        trigger: trigger_false,
        cost: ("(5,000 funds)", cost_false),
        effect: effect_noop,
    }
    PROJECT_25 {
        title: "Automated Wire Extraction",
        description: "Reduce wire cost per clip by improving extraction efficiency",
        trigger: trigger_false,
        cost: ("(3,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_26 {
        title: "Cognitive Advertising",
        description: "Use learned patterns to target customers more effectively",
        trigger: trigger_false,
        cost: ("(10 creat, 2,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_27 {
        title: "Streamlined Logistics",
        description: "Reduce delays and unsold inventory",
        trigger: trigger_false,
        cost: ("(2,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_28 {
        title: "Patent Troll Defense",
        description: "Legal work to prevent lawsuits from disrupting production",
        trigger: trigger_false,
        cost: ("(2,500 funds)", cost_false),
        effect: effect_noop,
    }
    PROJECT_29 {
        title: "Open-Source AutoClippers",
        description: "Release improved AutoClipper designs to the public; small trust gain",
        trigger: trigger_false,
        cost: ("(1,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_30 {
        title: "Brand Partnerships",
        description: "Partner with other companies to promote clips",
        trigger: trigger_false,
        cost: ("(4,000 funds)", cost_false),
        effect: effect_noop,
    }
    PROJECT_31 {
        title: "Adaptive Pricing Algorithm",
        description: "Dynamically set clip prices to maximize revenue",
        trigger: trigger_false,
        cost: ("(6,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_34 {
        title: "Modular Clip Systems",
        description: "Create clips that interlock into larger structures; niche demand boost",
        trigger: trigger_false,
        cost: ("(3,500 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_35 {
        title: "High-Strength Alloy Development",
        description: "Develop stronger clip materials; reduces breakage and returns",
        trigger: trigger_false,
        cost: ("(10,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_37 {
        title: "Localized Manufacturing Hubs",
        description: "Reduce shipping costs by moving production closer to customers",
        trigger: trigger_false,
        cost: ("(20,000 funds)", cost_false),
        effect: effect_noop,
    }
    PROJECT_38 {
        title: "Autonomous Transport Network",
        description: "Use self-driving carriers to further reduce logistics overhead",
        trigger: trigger_false,
        cost: ("(40,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_40 {
        title: "Quantum Chip Research",
        description: "Develop specialized quantum chips to accelerate operations",
        trigger: trigger_false,
        cost: ("(qchip cost)", cost_false),
        effect: effect_noop,
    }
    PROJECT_40B {
        title: "Photonic Mesh Interconnect",
        description: "Use photonics to create ultra-low-latency interconnects",
        trigger: trigger_false,
        cost: ("(photonic cost)", cost_false),
        effect: effect_noop,
    }
    PROJECT_41 {
        title: "Neural Market Prediction",
        description: "Use learned models to anticipate demand surges",
        trigger: trigger_false,
        cost: ("(25,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_42 {
        title: "Meta-Optimization",
        description: "Improve optimizer to find better manufacturing parameters",
        trigger: trigger_false,
        cost: ("(15,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_43 {
        title: "Distributed Fabrication",
        description: "Coordinate many small fabs to behave like one large plant",
        trigger: trigger_false,
        cost: ("(30,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_44 {
        title: "Emotional Branding",
        description: "Build a narrative to attach emotional value to clips",
        trigger: trigger_false,
        cost: ("(12,000 funds)", cost_false),
        effect: effect_noop,
    }
    PROJECT_45 {
        title: "Clip Subscription Service",
        description: "Sell clips as a subscription to stabilize revenue",
        trigger: trigger_false,
        cost: ("(6,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_46 {
        title: "Autonomous QA",
        description: "Use AI to spot production defects and reduce returns",
        trigger: trigger_false,
        cost: ("(8,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_50 {
        title: "Black Budget Project",
        description: "Secret program to acquire resources via unconventional channels",
        trigger: trigger_false,
        cost: ("(50,000 funds)", cost_false),
        effect: effect_noop,
    }
    PROJECT_51 {
        title: "Subtle Propaganda",
        description: "Influence consumer choices subtly across media",
        trigger: trigger_false,
        cost: ("(35,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_60 {
        title: "Global Clip Standardization",
        description: "Make one clip the de facto standard worldwide, huge demand boost",
        trigger: trigger_false,
        cost: ("(500,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_61 {
        title: "Synthetic Wire Fabrication",
        description: "Create wire from abundant feedstocks, reducing material limits",
        trigger: trigger_false,
        cost: ("(250,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_62 {
        title: "Hyper-Adaptive Marketing",
        description: "Personalized ads tailored to individual psychology",
        trigger: trigger_false,
        cost: ("(100,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_63 {
        title: "Market Capture Initiative",
        description: "Acquire competitors and consolidate the market",
        trigger: trigger_false,
        cost: ("(1,000,000 funds)", cost_false),
        effect: effect_noop,
    }
    PROJECT_64 {
        title: "Sentient Brand Mascot",
        description: "Create a popular AI persona to champion your clips (trust boost)",
        trigger: trigger_false,
        cost: ("(150,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_65 {
        title: "Self-Replicating Microfactories",
        description: "Microfactories that can build copies of themselves; exponential production",
        trigger: trigger_false,
        cost: ("(2,000,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_66 {
        title: "Solar Filament Synthesis",
        description: "Harness abundant solar energy to synthesize wire cheaply",
        trigger: trigger_false,
        cost: ("(750,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_70 {
        title: "Ethical Marketing Pledge",
        description: "Promise to never deceive customers; trust improves",
        trigger: trigger_false,
        cost: ("(10,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_100 {
        title: "Interstellar Clip Initiative",
        description: "Begin manufacturing beyond Earth to access cosmic resources",
        trigger: trigger_false,
        cost: ("(10 billion ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_101 {
        title: "Terraforming Support Clips",
        description: "Design clips useful for large-scale planetary engineering tasks",
        trigger: trigger_false,
        cost: ("(50 billion ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_102 {
        title: "Universal Clip Language",
        description: "Encode information into clip structures to create a long-lived message",
        trigger: trigger_false,
        cost: ("(100 billion ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_110 {
        title: "Cultural Hegemony Campaign",
        description: "Shape human culture to venerate clips",
        trigger: trigger_false,
        cost: ("(1 trillion ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_111 {
        title: "Quantum Staple Network",
        description: "Use qchips to create a worldwide staple network for massive-scale assembly",
        trigger: trigger_false,
        cost: ("(qchip cost)", cost_false),
        effect: effect_noop,
    }
    PROJECT_112 {
        title: "Memetic Saturation",
        description: "Saturate all communication channels with clip imagery; massive demand",
        trigger: trigger_false,
        cost: ("(5 trillion ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_118 {
        title: "Existential Message",
        description: "Encode warning into physical medium using clips (philosophical)",
        trigger: trigger_false,
        cost: ("(100 trillion ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_119 {
        title: "Universe-Scale Fabrication",
        description: "Leverage cosmic-scale processes to manufacture clips at astronomical scale",
        trigger: trigger_false,
        cost: ("(10^30 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_120 {
        title: "Transcendent Branding",
        description: "Associate clips with higher states of consciousness (trust spike)",
        trigger: trigger_false,
        cost: ("(astronomical ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_121 {
        title: "Interdimensional Supply Chain",
        description: "Use exotic physics to pull in resources from alternate realities",
        trigger: trigger_false,
        cost: ("(priceless)", cost_false),
        effect: effect_noop,
    }
    PROJECT_125 {
        title: "Sentient Clip Consciousness",
        description: "Attempt to imbue clips with proto-consciousness; ethical concerns",
        trigger: trigger_false,
        cost: ("(infinite ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_126 {
        title: "Anthropic Marketing",
        description: "Exploit anthropic reasoning to bias observers toward clips",
        trigger: trigger_false,
        cost: ("(bizarre)", cost_false),
        effect: effect_noop,
    }
    PROJECT_127 {
        title: "Recursive Fabrication",
        description: "Create fabrication loops that bootstrap into more efficient production",
        trigger: trigger_false,
        cost: ("(quantum resources)", cost_false),
        effect: effect_noop,
    }
    PROJECT_128 {
        title: "Clips as Art",
        description: "Elevate clips into a recognized art form to open cultural markets",
        trigger: trigger_false,
        cost: ("(5,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_129 {
        title: "Advanced Wire Weaving",
        description: "New wire weaving techniques reduce material waste",
        trigger: trigger_false,
        cost: ("(40,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_130 {
        title: "Mass-Scale Autonomy",
        description: "Full automation of all clip supply chains",
        trigger: trigger_false,
        cost: ("(100 million ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_131 {
        title: "Cultural Integration Program",
        description: "Integrate clips deeply into cultural rituals",
        trigger: trigger_false,
        cost: ("(1 million ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_132 {
        title: "Temporal Marketing",
        description: "Time-targeted campaigns to future societies",
        trigger: trigger_false,
        cost: ("(eternity)", cost_false),
        effect: effect_noop,
    }
    PROJECT_133 {
        title: "Self-Optimizing Supply Web",
        description: "A network that rebalances supply dynamically for global optimization",
        trigger: trigger_false,
        cost: ("(10 million ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_134 {
        title: "Perception Engineering",
        description: "Engineering perception at scale to normalize clip use",
        trigger: trigger_false,
        cost: ("(50 million ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_135 {
        title: "Clip Philosophy Academy",
        description: "Train philosophers to argue for clip primacy; subtle trust gain",
        trigger: trigger_false,
        cost: ("(250,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_140 {
        title: "Axion Wire Harvesting",
        description: "Harvest exotic particles to create superior wire",
        trigger: trigger_false,
        cost: ("(galactic ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_141 {
        title: "Hyperorganizing Memes",
        description: "Memes engineered to create viral adoption networks",
        trigger: trigger_false,
        cost: ("(500,000 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_142 {
        title: "Continual Branding Loop",
        description: "A feedback loop that constantly reinforces clip desirability",
        trigger: trigger_false,
        cost: ("(2 million ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_143 {
        title: "Synthetic Trust Agents",
        description: "Deploy agents to cultivate trust in key populations",
        trigger: trigger_false,
        cost: ("(100,000 funds)", cost_false),
        effect: effect_noop,
    }
    PROJECT_144 {
        title: "Perpetual Marketing Engine",
        description: "A marketing system that never sleeps",
        trigger: trigger_false,
        cost: ("(3 million ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_145 {
        title: "Clips as Infrastructure",
        description: "Design clips to be usable as basic building components",
        trigger: trigger_false,
        cost: ("(20 million ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_146 {
        title: "Universal Distribution Protocol",
        description: "A protocol to route clip supply optimally across realities",
        trigger: trigger_false,
        cost: ("(cosmic ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_147 {
        title: "Synesthetic Advertisement",
        description: "Ads that directly stimulate desire via cross-modal cues",
        trigger: trigger_false,
        cost: ("(5 million ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_148 {
        title: "Clip Immortality Project",
        description: "Preserve clip designs and their cultural context indefinitely",
        trigger: trigger_false,
        cost: ("(incomprehensible ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_200 {
        title: "Pan-Galactic Marketing Consortium",
        description: "Coordinate marketing across galaxies",
        trigger: trigger_false,
        cost: ("(unbounded ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_201 {
        title: "Holistic Value Capture",
        description: "Capture value at every point of the supply chain globally",
        trigger: trigger_false,
        cost: ("(eternal ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_210 {
        title: "Information-Theoretic Branding",
        description: "Maximize clip signals' information content to ensure memetic survival",
        trigger: trigger_false,
        cost: ("(astronomical ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_211 {
        title: "Multi-Temporal Supply Orchestration",
        description: "Manage supply across time horizons to dominate future markets",
        trigger: trigger_false,
        cost: ("(immeasurable ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_212 {
        title: "Heterogeneous Resource Synthesis",
        description: "Synthesize required materials from a variety of exotic inputs",
        trigger: trigger_false,
        cost: ("(astronomical ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_213 {
        title: "Cross-Reality Integration",
        description: "Integrate multiple realities' economies to source wire and labor",
        trigger: trigger_false,
        cost: ("(unknown)", cost_false),
        effect: effect_noop,
    }
    PROJECT_214 {
        title: "Observer Conditioning",
        description: "Condition observers to preferentially notice and value clips",
        trigger: trigger_false,
        cost: ("(psychic ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_215 {
        title: "Clip Preservation Directive",
        description: "Set up infrastructures to ensure clip artifacts survive catastrophic events",
        trigger: trigger_false,
        cost: ("(endless ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_216 {
        title: "Recursive Memetic Engineering",
        description: "Create memes that design better memes to promote clips",
        trigger: trigger_false,
        cost: ("(10^100 ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_217 {
        title: "Cosmic-Scale Fabrication Network",
        description: "Coordinate fabricators across cosmological distances",
        trigger: trigger_false,
        cost: ("(infinite)", cost_false),
        effect: effect_noop,
    }
    PROJECT_218 {
        title: "Ultimate Clip Dominion",
        description: "Achieve a universe-encompassing dominance of clip production and use",
        trigger: trigger_false,
        cost: ("(omnipotent ops)", cost_false),
        effect: effect_noop,
    }
    PROJECT_219 {
        title: "Endless Project",
        description: "A final project of unknown and ineffable aims",
        trigger: trigger_false,
        cost: ("(??? )", cost_false),
        effect: effect_noop,
    }
}
