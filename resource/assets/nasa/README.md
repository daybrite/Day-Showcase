<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: MPL-2.0
-->

# NASA pictures and video

Copies of the NASA pictures and video the showcase loads from the network, for fixture mode, and
the artwork the Media page gives the system's now-playing surface. With
`SHOWCASE_FIXTURES=1` in its environment (`day launch --env SHOWCASE_FIXTURES=1`), the app
answers every request for a picture from NASA's library with the copy here, and the Media page
plays `milton.mp4` instead of streaming the full video. The README at the top of the repository
says how to run it.

Every file comes from the [NASA Image and Video Library](https://images.nasa.gov). NASA's
[media usage guidelines](https://www.nasa.gov/nasa-brand-center/images-and-media/) say that its
images and video are generally not subject to copyright in the United States, that they may be
used in a factual manner that does not imply endorsement without explicit permission, and that
NASA should be acknowledged as the source. A picture made with a partner names that partner in
its credit below. One picture is an
exception: `PIA22946.jpg`, the Juno view of Jupiter, was processed from NASA's images by Kevin M.
Gill, who released it under Creative Commons Attribution (CC BY), so it carries his credit.

The pictures were downloaded at the library's `~small` size (640 pixels on the long side),
scaled to 512 pixels (256 for the six that only the Feeds page shows, as thumbnails), and saved
as JPEG at quality 68 with their metadata removed. Fixture mode answers every size the app asks
for with the one copy.

## Video

| File | Source | Credit |
|---|---|---|
| `milton.mp4` | [Cameras On The International Space Station Capture New Views Of Hurricane Milton, Oct. 9, 2024](https://images.nasa.gov/details/jsc2024m000174_Cameras_On_The_International_Space_Station_Capture_New_Views_Of_Hurricane_Milton-241009) | NASA (JSC) |
| `artwork.jpg` | The same video, one frame cropped to a square | NASA (JSC) |

The Media page streams that video's `~small.mp4` (640×360 H.264, AAC, 5 minutes 30 seconds,
63 MB). `milton.mp4` is six seconds of it from 1:00, scaled to 480×270 and re-encoded as H.264
Baseline with a mono AAC track (80 KB). The video carries no narration or music.

## Pictures

| File | Title | Credit |
|---|---|---|
| `PIA26681.jpg` | Image of Sun From NASA's Solar Dynamics Observatory | NASA/GSFC/Solar Dynamics Observatory |
| `GSFC_20171208_Archive_e001386.jpg` | Blue Marble 2012 | NASA (GSFC) |
| `GSFC_20171208_Archive_e001939.jpg` | The Far Side of the Moon | NASA (GSFC) |
| `PIA00003.jpg` | Valles Marineris Hemisphere | NASA/JPL/USGS |
| `PIA06193.jpg` | The Greatest Saturn Portrait ...Yet | NASA/JPL/Space Science Institute |
| `PIA00046.jpg` | Neptune Full Disk | NASA/JPL |
| `carina_nebula.jpg` | Webb's NIRCam image of the "Cosmic Cliffs" in the Carina Nebula | NASA, ESA, CSA, STScI |
| `PIA04921.jpg` | Andromeda Galaxy | NASA/JPL/California Institute of Technology |
| `GSFC_20171208_Archive_e000842.jpg` | Hubble's high-definition "Pillars of Creation" | NASA, ESA (GSFC) |
| `PIA04230.jpg` | Whirlpool Galaxy | NASA and The Hubble Heritage Team (STScI/AURA) |
| `PIA22946.jpg` | Jupiter Marble | Enhanced image by Kevin M. Gill (CC BY), based on images courtesy of NASA/JPL-Caltech/SwRI/MSSS |
| `PIA03606.jpg` | Most Detailed Image of the Crab Nebula | NASA/ESA/JPL/Arizona State Univ. |
| `PIA11245.jpg` | Mercury as Never Seen Before | NASA/Johns Hopkins University Applied Physics Laboratory/Carnegie Institution of Washington |
| `PIA00104.jpg` | Venus, computer-simulated global view | NASA/JPL |
| `as17-148-22727.jpg` | View of the Earth seen by the Apollo 17 crew traveling toward the moon | NASA (JSC) |
| `PIA02879.jpg` | A New Year for Jupiter and Io | NASA/JPL/University of Arizona |
| `PIA00032.jpg` | Uranus in True and False Color | NASA/JPL |
| `PIA19952.jpg` | The Rich Color Variations of Pluto | NASA/Johns Hopkins University Applied Physics Laboratory/Southwest Research Institute |

Each file is named after the picture's id in the library: the page for `PIA00003.jpg` is
`https://images.nasa.gov/details/PIA00003`.
