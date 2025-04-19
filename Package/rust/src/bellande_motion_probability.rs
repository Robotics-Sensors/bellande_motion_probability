// Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use reqwest;
use serde_json::{json, Value};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "bellande_motion_probability",
    about = "Bellande Motion Probability Tool"
)]
struct Opt {
    #[structopt(
        long,
        help = "Current particle state as JSON-formatted list [x, y, heading, weight]"
    )]
    particle_state: String,

    #[structopt(long, help = "Previous pose as JSON-formatted list [x, y, heading]")]
    previous_pose: String,

    #[structopt(long, help = "Current pose as JSON-formatted list [x, y, heading]")]
    current_pose: String,

    #[structopt(
        long,
        help = "Noise parameters as JSON object with trans_sigma, rot_sigma, and head_sigma"
    )]
    noise_params: Option<String>,

    #[structopt(
        long,
        default_value = "50.0",
        help = "Search radius for motion probability calculation"
    )]
    search_radius: f64,

    #[structopt(
        long,
        default_value = "20",
        help = "Number of sample points for motion probability calculation"
    )]
    sample_points: i32,

    #[structopt(long, help = "Use local executable instead of API")]
    use_executable: bool,
}

pub async fn make_bellande_motion_probability_request(
    particle_state: Value,
    previous_pose: Value,
    current_pose: Value,
    noise_params: Option<Value>,
    search_radius: f64,
    sample_points: i32,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Motion_Probability/bellande_motion_probability";

    let payload = json!({
        "particle_state": particle_state,
        "previous_pose": previous_pose,
        "current_pose": current_pose,
        "noise_params": noise_params.unwrap_or(json!({
            "trans_sigma": 0.1,
            "rot_sigma": 0.1,
            "head_sigma": 0.1
        })),
        "search_radius": search_radius,
        "sample_points": sample_points,
        "auth": {
            "authorization_key": "bellande_web_api_opensource"
        }
    });

    let response = client
        .post(url)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(response)
}

pub async fn make_bellande_motion_probability_request_local(
    url: &str,
    particle_state: Value,
    previous_pose: Value,
    current_pose: Value,
    noise_params: Option<Value>,
    search_radius: f64,
    sample_points: i32,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = url;

    let payload = json!({
        "particle_state": particle_state,
        "previous_pose": previous_pose,
        "current_pose": current_pose,
        "noise_params": noise_params.unwrap_or(json!({
            "trans_sigma": 0.1,
            "rot_sigma": 0.1,
            "head_sigma": 0.1
        })),
        "search_radius": search_radius,
        "sample_points": sample_points,
        "auth": {
            "authorization_key": "bellande_web_api_opensource"
        }
    });

    let response = client
        .post(url)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(response)
}

pub fn get_executable_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Bellande_Motion_Probability.exe")
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Bellande_Motion_Probability")
    }
}

pub fn run_bellande_motion_probability_executable(
    particle_state: &str,
    previous_pose: &str,
    current_pose: &str,
    noise_params: Option<&str>,
    search_radius: f64,
    sample_points: i32,
) -> Result<(), Box<dyn Error>> {
    let executable_path = get_executable_path();
    let passcode = "bellande_motion_probability_executable_access_key";

    // Parse and validate inputs
    let particle_state_val: Value = serde_json::from_str(particle_state)?;
    let previous_pose_val: Value = serde_json::from_str(previous_pose)?;
    let current_pose_val: Value = serde_json::from_str(current_pose)?;
    let noise_params_val: Value = noise_params
        .map(|n| serde_json::from_str(n))
        .transpose()?
        .unwrap_or(json!({
            "trans_sigma": 0.1,
            "rot_sigma": 0.1,
            "head_sigma": 0.1
        }));

    // Validate particle state dimensions
    let particle_state_arr = particle_state_val
        .as_array()
        .ok_or("Invalid particle state format")?;
    if particle_state_arr.len() != 4 {
        return Err("Particle state must have 4 components [x, y, heading, weight]".into());
    }

    // Validate pose dimensions
    for (name, pose) in [
        ("Previous pose", &previous_pose_val),
        ("Current pose", &current_pose_val),
    ] {
        if let Some(arr) = pose.as_array() {
            if arr.len() != 3 {
                return Err(format!("{} must have 3 components [x, y, heading]", name).into());
            }
        }
    }

    // Prepare and run command
    let mut command = Command::new(executable_path);
    command.args(&[
        passcode,
        particle_state,
        previous_pose,
        current_pose,
        &serde_json::to_string(&noise_params_val)?,
        &search_radius.to_string(),
        &sample_points.to_string(),
    ]);

    let output = command.output()?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    } else {
        Err(format!(
            "Process failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into())
    }
}
