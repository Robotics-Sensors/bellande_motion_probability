# 📦 Bellande Motion Probability

## 🧙 Organization Website
- [![Organization Website](https://img.shields.io/badge/Explore%20Our-Website-0099cc?style=for-the-badge)](https://robotics-sensors.github.io)

## 🧙 Organization Github
- [![Organization Github ](https://img.shields.io/badge/Explore%20Our-Github-0099cc?style=for-the-badge)](https://github.com/Robotics-Sensors)

# Author, Creator and Maintainer
- **Ronaldson Bellande**

# API HTTP Usability (BELLANDE FORMAT)
```
# Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
# 
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
# GNU General Public License v3.0 or later

url: https://bellande-robotics-sensors-research-innovation-center.org

endpoint_path:
    bellande_motion_probability: /api/Bellande_Motion_Probability/bellande_motion_probability

Bellande_Framework_Access_Key: bellande_web_api_opensource
```

# API HTTP Usability (JSON FORMAT)
```
{
  "license": [
    "Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande",
    "This program is free software: you can redistribute it and/or modify",
    "it under the terms of the GNU General Public License as published by",
    "the Free Software Foundation, either version 3 of the License, or",
    "(at your option) any later version.",
    "",
    "This program is distributed in the hope that it will be useful,",
    "but WITHOUT ANY WARRANTY; without even the implied warranty of",
    "MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the",
    "GNU General Public License for more details.",
    "",
    "You should have received a copy of the GNU General Public License",
    "along with this program.  If not, see <https://www.gnu.org/licenses/>.",
    "GNU General Public License v3.0 or later"
  ],
  "url": "https://bellande-robotics-sensors-research-innovation-center.org",
  "endpoint_path": {
    "bellande_motion_probability": "/api/Bellande_Motion_Probability/bellande_motion_probability"
  },
  "Bellande_Framework_Access_Key": "bellande_web_api_opensource"
}
```

# API Payload Example
```
{
    "particle_state": [0, 0, 0, 1.0],
    "previous_pose": [0, 0, 0],
    "current_pose": [100, 100, 90],
    "noise_params": {
      "trans_sigma": 0.1,
      "rot_sigma": 0.1,
      "head_sigma": 0.1
    },
    "search_radius": 50,
    "sample_points": 20,
    "auth": {
      "authorization_key": "bellande_web_api_opensource"
    }
```

# 🧙 Website Bellande API Testing 
- [![Website API Testing](https://img.shields.io/badge/Bellande%20API-Testing-0099cc?style=for-the-badge)](https://bellande-robotics-sensors-research-innovation-center.org/api/bellande_motion_probability_experiment)
  
# Quick Bellande API Testing
```
curl -X 'POST' \
  'https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Motion_Probability/bellande_motion_probability' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
    "particle_state": [0, 0, 0, 1.0],
    "previous_pose": [0, 0, 0],
    "current_pose": [100, 100, 90],
    "noise_params": {
      "trans_sigma": 0.1,
      "rot_sigma": 0.1,
      "head_sigma": 0.1
    },
    "search_radius": 50,
    "sample_points": 20,
    "auth": {
      "authorization_key": "bellande_web_api_opensource"
    }
  }'
```

# Bellande Motion_Probability Usage

## Website Crates
- https://crates.io/crates/bellande_motion_probability

### Installation
- `cargo add bellande_motion_probability`

## Website PYPI
- https://pypi.org/project/bellande_motion_probability

### Installation
- `$ pip install bellande_motion_probability`

### Usage 
```
bellande_motion_probability \
  --particle-state "[1.0, 2.0, 0.5, 1.0]" \
  --previous-pose "[0.0, 0.0, 0.0]" \
  --current-pose "[1.0, 1.0, 0.5]" \
  --noise-params '{"trans_sigma": 0.1, "rot_sigma": 0.1, "head_sigma": 0.1}'
```

### Upgrade (if not upgraded)
- `$ pip install --upgrade bellande_motion_probability`

```
Name: bellande_motion_probability
Summary: The Bellande Motion Probability algorithm calculates particle movement probabilities using Bellande distributions for enhanced robot motion estimation
Home-page: github.com/Robotics-Sensors/bellande_motion_probability
Author: Ronaldson Bellande
Author-email: ronaldsonbellande@gmail.com
License: GNU General Public License v3.0
```


## License
This Algorithm or Models is distributed under the [Creative Commons Attribution-ShareAlike 4.0 International License](http://creativecommons.org/licenses/by-sa/4.0/), see [LICENSE](https://github.com/RonaldsonBellande/bellande_step/blob/main/LICENSE) and [NOTICE](https://github.com/RonaldsonBellande/bellande_step/blob/main/LICENSE) for more information.
