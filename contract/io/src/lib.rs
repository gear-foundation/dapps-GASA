#![no_std]

use codec::{Decode, Encode};
use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<String, ()>;
    type Handle = InOut<Action, Event>;
    type Reply = InOut<(), ()>;
    type Others = InOut<(), ()>;
    type Signal = ();
    type State = LaunchSite;
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum Action {
    Info,
    RegisterParticipant(String),
    ChangeParticipantName(String),
    StartNewSession,
    RegisterOnLaunch {
        fuel_amount: u32,
        payload_amount: u32,
    },
    ExecuteSession,
    ReserveGas,
}

#[derive(Encode, Debug, PartialEq, Eq, Decode, TypeInfo)]
pub enum Event {
    Info {
        owner: ActorId,
        name: String,
        has_current_session: bool,
    },
    NewParticipant {
        id: ActorId,
        name: String,
    },
    ParticipantNameChange {
        id: ActorId,
        name: String,
    },
    NewLaunch {
        id: u32,
        name: String,
        weather: u32,
        altitude: u32,
        fuel_price: u32,
        payload_value: u32,
    },
    LaunchRegistration {
        id: u32,
        participant: ActorId,
    },
    LaunchStarted {
        id: u32,
    },
    LaunchFinished {
        id: u32,
        stats: Vec<(ActorId, bool, u32, u32)>, // participant id, success, final altitude, earnings
    },
    SessionInfo {
        weather: u32,
        altitude: u32,
        fuel_price: u32,
        payload_value: u32,
    },
    NoCurrentSession,
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct CurrentSesionInfo {
    pub name: String,
    pub weather: u32,
    pub altitude: u32,
    pub fuel_price: u32,
    pub payload_value: u32,
}

#[derive(Default, Encode, Decode, TypeInfo, Clone, Debug)]
pub struct CurrentStat {
    pub participant: ActorId,
    pub alive: bool,
    pub fuel_left: u32,
    pub last_altitude: u32,
    pub payload: u32,
    pub halt: Option<RocketHalt>,
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct ParticipantState {
    pub name: String,
    pub balance: u32,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct LaunchSite {
    pub name: String,
    pub owner: ActorId,
    pub participants: BTreeMap<ActorId, Participant>,
    pub current_session: Option<CurrentSession>,
    pub events: BTreeMap<u32, Vec<CurrentStat>>,
    pub state: SessionState,
    pub session_id: u32,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct SessionStrategy {
    pub fuel: u32,
    pub payload: u32,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct CurrentSession {
    pub altitude: u32,
    pub weather: u32,
    pub fuel_price: u32,
    pub payload_value: u32,
    pub registered: BTreeMap<ActorId, SessionStrategy>,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct Participant {
    pub name: String,
    pub balance: u32,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
pub enum RocketHalt {
    Overfilled,
    Overfuelled,
    SeparationFailure,
    Asteroid,
    NotEnoughFuel,
    EngineError,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    SessionIsOver,
    #[default]
    NoSession,
    Registration,
}
