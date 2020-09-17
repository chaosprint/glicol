mod calc_node;
mod osc_node;
mod sampler_node;
mod env_node;
mod control_node;

use osc_node::{SinOsc, Impulse};
use calc_node::{Add, Mul};
use sampler_node::{Sampler};
use control_node::{Sequencer, Speed};
use env_node::EnvPerc;