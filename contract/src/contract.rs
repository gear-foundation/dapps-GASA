use gstd::{
    exec,
    msg::{self, send_delayed, send_delayed_from_reservation},
    prelude::*,
    ActorId, ReservationId,
};
use launch_io::*;

pub const WEATHER_RANGE: u32 = 5;
pub const MIN_FUEL_PRICE: u32 = 80;
pub const MAX_FUEL_PRICE: u32 = 120;

pub const MIN_PAYLOAD_VALUE: u32 = 80;
pub const MAX_PAYLOAD_VALUE: u32 = 120;

pub const MIN_ALTITUDE: u32 = 8_000;
pub const MAX_ALTITUDE: u32 = 15_000;

const RESERVATION_AMOUNT: u64 = 240_000_000_000;
const GAS_FOR_UPDATE: u64 = 4_000_000_000;
static mut RESERVATION: Vec<ReservationId> = vec![];

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct LaunchSite {
    pub name: String,
    pub owner: ActorId,
    pub participants: BTreeMap<ActorId, Participant>,
    pub current_session: Option<CurrentSession>,
    pub events: BTreeMap<u32, Vec<CurrentStat>>,
    pub state: SessionState,
    pub session_id: u32,
    pub execute_session_period: u32,
}

static mut LAUNCH_SITE: Option<LaunchSite> = None;

impl LaunchSite {
    fn info(&self) {
        msg::reply(
            Event::Info {
                name: self.name.clone(),
                owner: self.owner,
                has_current_session: self.current_session.is_some(),
            },
            0,
        )
        .expect("Error in a reply `::info");
    }

    fn new_participant(&mut self, name: String) {
        let actor_id = msg::source();

        if self.participants.contains_key(&actor_id) {
            panic!("There is already participant registered with this id");
        }

        self.participants.insert(
            actor_id,
            Participant {
                name: name.clone(),
                balance: 0,
            },
        );

        msg::reply(Event::NewParticipant { id: actor_id, name }, 0)
            .expect("failed to reply in ::new_participant");
    }

    fn rename_participant(&mut self, name: String) {
        let actor_id = msg::source();

        if !self.participants.contains_key(&actor_id) {
            panic!("There is no participant registered with this id");
        }

        let participant = self
            .participants
            .get_mut(&actor_id)
            .expect("checked above that exists");

        participant.name = name.clone();

        msg::reply(Event::ParticipantNameChange { id: actor_id, name }, 0)
            .expect("failed to reply in ::rename_participant");
    }

    fn new_session(&mut self) {
        let actor_id = msg::source();

        assert_eq!(actor_id, self.owner);
        assert!(self.state == SessionState::SessionIsOver || self.state == SessionState::NoSession);

        // 0 - Sunny, 1 - Cloudy, 2 - Rainy, 3 - Stormy, 4 - Thunder, 5 - Tornado
        let random_weather = generate(0, WEATHER_RANGE);
        let random_fuel_price = generate(MIN_FUEL_PRICE, MAX_FUEL_PRICE);
        let random_payload_value = generate(MIN_PAYLOAD_VALUE, MAX_PAYLOAD_VALUE);
        let random_altitude = generate(MIN_ALTITUDE, MAX_ALTITUDE);

        self.current_session = Some(CurrentSession {
            weather: random_weather,
            fuel_price: random_fuel_price,
            reward: random_payload_value as u128,
            altitude: random_altitude,
            registered: Default::default(),
            bet: None,
        });
        self.session_id = self.session_id.saturating_add(1);
        self.state = SessionState::Registration;
        self.events = BTreeMap::new();

        msg::reply(
            Event::NewLaunch {
                id: 0,
                name: "Unnamed".to_string(),
                weather: random_weather,
                fuel_price: random_fuel_price,
                payload_value: random_payload_value,
                altitude: random_altitude,
            },
            0,
        )
        .expect("failed to reply in ::new_session");
    }

    // fn register_on_launch(&mut self, fuel_amount: u32, payload_amount: u32) {
    //     let actor_id = msg::source();

    //     assert!(self.current_session.is_some());

    //     assert!(fuel_amount <= 100 && payload_amount <= 100, "Limit is 100%");

    //     let current_session = self
    //         .current_session
    //         .as_mut()
    //         .expect("checked above that exists");

    //     if current_session.registered.contains_key(&actor_id) {
    //         // already registered

    //         panic!("Participant already registered on the session");
    //     }

    //     current_session.registered.insert(
    //         actor_id,
    //         SessionStrategy {
    //             fuel: fuel_amount,
    //             payload: payload_amount,
    //         },
    //     );

    //     msg::reply(
    //         Event::LaunchRegistration {
    //             id: 0,
    //             participant: actor_id,
    //         },
    //         0,
    //     )
    //     .expect("failed to reply in ::new_session");
    // }

    fn execute_session(&mut self) {
        // Trying to rerun execute session after period
        let action = Action::ExecuteSession;
        let gas_available = exec::gas_available();
        if gas_available <= GAS_FOR_UPDATE {
            let reservations = unsafe { &mut RESERVATION };
            let reservation_id = reservations.pop().expect("Need more gas");
            send_delayed_from_reservation(
                reservation_id,
                exec::program_id(),
                action,
                0,
                self.execute_session_period,
            )
            .expect("Can't send delayed from reservation");
        } else {
            send_delayed(exec::program_id(), action, 0, self.execute_session_period)
                .expect("Can't send delayed");
        }

        let session_data = self
            .current_session
            .as_ref()
            .expect("There should be active session to execute");
        let mut current_altitude = 0;
        let total_rounds = 3;
        let weather = session_data.weather;

        let mut current_stats: BTreeMap<ActorId, CurrentStat> = BTreeMap::new();

        for (id, (strategy, _participant)) in session_data.registered.iter() {
            current_stats.insert(
                *id,
                CurrentStat {
                    participant: *id,
                    alive: true,
                    fuel_left: strategy.fuel,
                    fuel_capacity: strategy.fuel,
                    last_altitude: 0,
                    payload: strategy.payload,
                    halt: None,
                },
            );
        }

        for i in 0..total_rounds {
            current_altitude += session_data.altitude / total_rounds;

            for (id, (strategy, _participant)) in session_data.registered.iter() {
                // if 1/3 or 2/3 of distance the probability of separation failure

                // risk factor of burning fuel
                let fuel_burn = (strategy.payload + 2 * weather) / total_rounds;

                let current_stat = current_stats.get_mut(id).expect("all have stats");

                if !current_stat.alive {
                    continue;
                } // already failed;

                // if 1/3 distance then probability of engine error is 3%
                if i == 0 && generate_event(3) {
                    current_stat.halt = Some(RocketHalt::EngineError);
                    current_stat.alive = false;
                };

                // if 1/3 distance and fuel > 80% - risk factor of weather
                if i == 0 && current_stat.fuel_left >= (80 - 2 * weather) && generate_event(10) {
                    current_stat.halt = Some(RocketHalt::Overfuelled);
                    current_stat.alive = false;
                };

                // if  2/3 of distance and filled > 80% - risk factor of weather
                // 10 percent that will be overfilled
                if i == 1 && strategy.payload >= (80 - 2 * weather) && generate_event(10) {
                    current_stat.halt = Some(RocketHalt::Overfilled);
                    current_stat.alive = false;
                };

                // if 2/3 of distance
                // 5 percent that will be separation failure
                if i == 1 && generate_event(5 + weather as u8) {
                    current_stat.halt = Some(RocketHalt::SeparationFailure);
                    current_stat.alive = false;
                };

                // if last distance 10 percent od asteroid
                // 10 percent that will be asteroid + weather factor
                if i == 2 && generate_event(10 + weather as u8) {
                    current_stat.halt = Some(RocketHalt::Asteroid);
                    current_stat.alive = false;
                };

                if current_stat.fuel_left < fuel_burn {
                    // fuel is over
                    current_stat.alive = false;
                    current_stat.halt = Some(RocketHalt::NotEnoughFuel);
                } else {
                    current_stat.last_altitude = current_altitude;
                    current_stat.fuel_left -= fuel_burn;
                }

                // weather random affect?
                self.events
                    .entry(i)
                    .and_modify(|events| events.push(current_stat.clone()))
                    .or_insert_with(|| vec![current_stat.clone()]);
            }
        }

        let mut outcome_participants = vec![];
        let mut max_fuel_left = 0;

        for (_, stat) in current_stats.iter() {
            if stat.alive && stat.fuel_left > max_fuel_left {
                max_fuel_left = stat.fuel_left;
            }
        }
        let mut winner = (ActorId::default(), 0);
        for (id, stat) in current_stats.iter() {
            if stat.alive {
                let coef = if stat.fuel_left == 0 {
                    // 1.7 if fuel tank = 0
                    17
                } else {
                    // max fuel left -> multiply by 0.5
                    5 * stat.fuel_left / max_fuel_left
                };

                let mut earnings = (stat.payload as u128 * session_data.reward * coef as u128);
                earnings = earnings - (session_data.fuel_price * stat.fuel_capacity) as u128; 
                earnings = earnings / 10;
                outcome_participants.push((*id, stat.alive, stat.last_altitude, earnings));

                if winner.1 < earnings {
                    winner.0 = *id;
                    winner.1 = earnings;
                }

                let leaderboard_entry = self
                    .participants
                    .get_mut(id)
                    .expect("Should have existed in leaderboards");

                leaderboard_entry.balance += earnings;
            }
        }
        if let Some(bet) = session_data.bet {
            let prize = bet * current_stats.len() as u128;
            let prize = if prize < 200 {
                prize - 1
            } else {
                prize - prize * 5 / 1000
            };

            msg::send(winner.0, (), prize).expect("Can't send total deposit"); // send total deposit to winner
        }
        self.state = SessionState::SessionIsOver;

        // handle round results

        msg::reply(
            Event::LaunchFinished {
                id: 0,
                stats: outcome_participants,
            },
            0,
        )
        .expect("failed to reply in ::new_session");
    }

    fn reserve_gas(&self) {
        let reservations = unsafe { &mut RESERVATION };
        let reservation_id =
            ReservationId::reserve(RESERVATION_AMOUNT, 600).expect("reservation across executions");
        reservations.push(reservation_id);
        msg::reply(Event::GasReserved, 0).expect("");
    }

    fn register_participant_on_launch(
        &mut self,
        name: String,
        fuel_amount: u32,
        payload_amount: u32,
    ) {
        //new_participant
        let actor_id = msg::source();
        let value = msg::value();

        if self.participants.contains_key(&actor_id) {
            panic!("There is already participant registered with this id");
        }
        let participant = Participant { name, balance: 0 };
        self.participants.insert(actor_id, participant.clone());

        // comment original reply from `new_participant()`
        // msg::reply(Event::NewParticipant { id: actor_id, name }, 0)
        //     .expect("failed to reply in ::new_participant");

        assert!(self.current_session.is_some());

        assert!(fuel_amount <= 100 && payload_amount <= 100, "Limit is 100%");

        let current_session = self
            .current_session
            .as_mut()
            .expect("checked above that exists");

        if current_session.registered.contains_key(&actor_id) {
            // already registered

            panic!("Participant already registered on the session");
        }
        current_session.bet = Some(value);

        let session_strategy = SessionStrategy {
            fuel: fuel_amount,
            payload: payload_amount,
        };
        current_session
            .registered
            .insert(actor_id, (session_strategy, participant));

        msg::reply(
            Event::LaunchRegistration {
                id: 0,
                participant: actor_id,
            },
            0,
        )
        .expect("failed to reply in ::new_session");
    }
}

#[gstd::async_main]
async fn main() {
    let action: Action = msg::load().expect("Unable to decode `Action`");
    let launch_site = unsafe { LAUNCH_SITE.get_or_insert(Default::default()) };
    match action {
        Action::Info => {
            launch_site.info();
        }
        Action::RegisterParticipant(name) => {
            launch_site.new_participant(name);
        }
        Action::ChangeParticipantName(name) => {
            launch_site.rename_participant(name);
        }
        Action::StartNewSession => {
            launch_site.new_session();
        }
        Action::RegisterOnLaunch {
            fuel_amount,
            payload_amount,
        } => {
            // launch_site.register_on_launch(fuel_amount, payload_amount);
            unreachable!("You should youse RegisterParticipantOnLaunch")
        }
        Action::ExecuteSession => {
            launch_site.execute_session();
        }
        Action::ReserveGas => {
            launch_site.reserve_gas();
        }
        Action::RegisterParticipantOnLaunch {
            name,
            fuel_amount,
            payload_amount,
        } => {
            launch_site.register_participant_on_launch(name, fuel_amount, payload_amount);
        }
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let init: Initialize = msg::load().expect("Error in decoding");
    let launch_site = LaunchSite {
        name: init.name,
        execute_session_period: init.period_sec,
        owner: msg::source(),
        ..Default::default()
    };
    // send_delayed(
    //     exec::program_id(),
    //     Action::ExecuteSession,
    //     0,
    //     init.period_sec,
    // )
    // .expect("Can't send delayed Action::ExecuteSession at init()");
    LAUNCH_SITE = Some(launch_site);
}

#[no_mangle]
extern "C" fn state() {
    let launch_site = unsafe { LAUNCH_SITE.get_or_insert(Default::default()) };
    msg::reply(launch_site, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}

static mut SEED: u8 = 0;

fn generate_event(probability: u8) -> bool {
    let seed = unsafe { SEED };
    unsafe { SEED = SEED.wrapping_add(1) };
    let random_input: [u8; 32] = [seed; 32];
    let (random, _) = exec::random(random_input).expect("Error in getting random number");

    let prob = random[0] % 100;
    prob <= probability
}

fn generate(min: u32, max: u32) -> u32 {
    let seed = unsafe { SEED };
    unsafe { SEED += 1 };
    let random_input: [u8; 32] = [seed; 32];
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    let bytes = [random[0], random[1], random[2], random[3]];
    min + u32::from_be_bytes(bytes) % (max - min)
}
