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
echo ""
