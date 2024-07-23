use crate::{config, Error};

use common::{api::{self, RegisterAgent}, crypto};

use ed25519_dalek::Signer;
use rand::RngCore;
use std::path::PathBuf;
use std::{convert::TryInto, fs};
use x25519_dalek::{x25519, X25519_BASEPOINT_BYTES};

